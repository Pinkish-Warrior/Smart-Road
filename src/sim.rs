use std::{
    thread,
    time::{Duration, Instant},
};

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    render::{Canvas, Texture, TextureCreator},
    rect::Rect,
    video::{FullscreenType, Window, WindowContext},
    Sdl,
};

use crate::{
    cars::Traffic,
    textures, trees,
    types::{Airt, Dimensions, Speed},
};

pub fn simulate(traffic: &mut Traffic) {
    let (sdl_context, mut canvas, mut dimensions) = setup();
    let texture_creator = canvas.texture_creator();
    let (background_texture, lanes_texture, car_textures, tree_textures) =
        textures::create_textures(&texture_creator, &dimensions, &mut canvas);

    run(
        &sdl_context,
        &mut canvas,
        &mut dimensions,
        traffic,
        &texture_creator,
        &background_texture,
        &lanes_texture,
        &car_textures,
        &tree_textures,
    );
}

fn setup() -> (sdl2::Sdl, Canvas<Window>, Dimensions) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let (_ddpi, hdpi, vdpi) = video_subsystem.display_dpi(0).unwrap();

    let display_mode = video_subsystem.current_display_mode(0).unwrap();
    let screen_height = display_mode.h;

    let window_width = screen_height as f32 * 0.8 * hdpi / 133.0;
    let window_height = screen_height as f32 * 0.8 * vdpi / 139.0;

    const LANE_WIDTH: i32 = 16;
    let speed = Speed {
        fast: LANE_WIDTH * 3 / 4,
        default: LANE_WIDTH / 2,
        slow: LANE_WIDTH / 4,
    };

    let window_width = window_width as i32;
    let window_height = window_height as i32;

    let dimensions = Dimensions {
        window_width,
        window_height,
        half_width: window_width / 2,
        half_height: window_height / 2,
        lane_width: LANE_WIDTH,
        speed,
    };

    let window = video_subsystem
        .window(
            "Smart Road",
            dimensions.window_width as u32,
            dimensions.window_height as u32,
        )
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas
        .set_logical_size(
            dimensions.window_width as u32,
            dimensions.window_height as u32,
        )
        .unwrap();

    (sdl_context, canvas, dimensions)
}

fn run(
    sdl_context: &Sdl,
    canvas: &mut Canvas<sdl2::video::Window>,
    dimensions: &mut Dimensions,
    traffic: &mut Traffic,
    texture_creator: &TextureCreator<WindowContext>,
    background_texture: &sdl2::render::Texture,
    lanes_texture: &Texture,
    car_textures: &[sdl2::render::Texture; 4],
    tree_textures: &Vec<(Texture, [f64; 2])>,
) {
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_keypress_time = Instant::now();
    let keypress_interval = Duration::from_millis(128); // Change, e.g. from 128 to 32 to see gridlock.
    let mut start_time = Instant::now();
    let mut is_fullscreen = false;
    let mut show_help = false;

    'running: loop {
        let now = Instant::now();
        let elapsed = now.duration_since(start_time);
        if elapsed < Duration::from_millis(16) {
            continue;
        }
        start_time = now;
        // instantaneous FPS (may vary a lot); we'll display it in the window title and HUD
        let fps = if elapsed.as_secs_f64() > 0.0 {
            1.0 / elapsed.as_secs_f64()
        } else {
            0.0
        };

        traffic.update(&dimensions);
        render(
            canvas,
            &dimensions,
            &traffic,
            background_texture,
            car_textures,
            lanes_texture,
            texture_creator,
            tree_textures,
            fps,
            show_help,
        );

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    let now = Instant::now();
                    if now.duration_since(last_keypress_time) <= keypress_interval {
                        continue;
                    }
                    match keycode {
                        Keycode::Up => {
                            traffic.push(Airt::Up, &dimensions);
                        }
                        Keycode::Down => {
                            traffic.push(Airt::Down, &dimensions);
                        }
                        Keycode::Left => {
                            traffic.push(Airt::Left, &dimensions);
                        }
                        Keycode::Right => {
                            traffic.push(Airt::Right, &dimensions);
                        }
                        Keycode::R => {
                            traffic.push_random(&dimensions);
                        }

                        Keycode::F => {
                            let window = canvas.window_mut();
                            is_fullscreen = !is_fullscreen;
                            window
                                .set_fullscreen(if is_fullscreen {
                                    FullscreenType::Desktop
                                } else {
                                    FullscreenType::Off
                                })
                                .unwrap();
                        }
                        Keycode::H => {
                            show_help = !show_help;
                        }
                        _ => {}
                    }
                    last_keypress_time = now;
                }
                _ => {}
            }
        }
    }

    // To ward against closing the stats window if you press escape for too long.
    thread::sleep(Duration::from_millis(128));
}

fn render(
    canvas: &mut Canvas<sdl2::video::Window>,
    dimensions: &Dimensions,
    traffic: &Traffic,
    background_texture: &sdl2::render::Texture,
    car_textures: &[sdl2::render::Texture; 4],
    lanes_texture: &Texture,
    texture_creator: &TextureCreator<WindowContext>,
    tree_textures: &Vec<(Texture, [f64; 2])>,
    fps: f64,
    show_help: bool,
) {
    canvas.set_draw_color(Color::RGB(240, 240, 240));
    canvas.clear();

    canvas.copy(&tree_textures[0].0, None, None).unwrap();
    canvas.copy(background_texture, None, None).unwrap();
    canvas.copy(lanes_texture, None, None).unwrap();

    traffic.draw(canvas, &dimensions, car_textures);

    trees::plant(canvas, tree_textures);

    let snow = textures::create_speckled_texture(
        texture_creator,
        dimensions.window_width as u32,
        dimensions.window_height as u32,
        canvas,
    );
    canvas.copy(&snow, None, None).unwrap();

        // Draw a comprehensive HUD with better visual hierarchy
    let hud_height = 90u32;
    let hud_rect = Rect::new(0, 0, dimensions.window_width as u32, hud_height);
    
    // Semi-transparent dark background for better readability
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 180));
    canvas.fill_rect(hud_rect).ok();

    // Draw accent line at bottom of HUD
    canvas.set_draw_color(Color::RGBA(100, 180, 255, 200));
    canvas
        .fill_rect(Rect::new(0, hud_height as i32 - 2, dimensions.window_width as u32, 2))
        .ok();

    // Row 1: Active cars and Cars passed with LABELS
    let row1_y = 8;
    let sq = 16u32;
    let pad = 12;

    // Active cars (red square)
    canvas.set_draw_color(Color::RGB(220, 60, 60));
    canvas.fill_rect(Rect::new(pad, row1_y, sq, sq)).ok();
    // Label: "ACTIVE"
    draw_hud_label(canvas, pad + sq as i32 + 4, row1_y + 2, "ACTIVE", Color::RGB(255, 255, 255));
    draw_hud_number(canvas, pad + sq as i32 + 4, row1_y + 12, traffic.cars.len() as i32, Color::RGB(255, 200, 200));

    // Cars passed (green square)
    canvas.set_draw_color(Color::RGB(60, 220, 60));
    canvas.fill_rect(Rect::new(pad + 140, row1_y, sq, sq)).ok();
    // Label: "PASSED"
    draw_hud_label(canvas, pad + 140 + sq as i32 + 4, row1_y + 2, "PASSED", Color::RGB(255, 255, 255));
    draw_hud_number(canvas, pad + 140 + sq as i32 + 4, row1_y + 12, traffic.cars_passed, Color::RGB(200, 255, 200));

    // Give ways (yellow square)
    canvas.set_draw_color(Color::RGB(220, 220, 60));
    canvas.fill_rect(Rect::new(pad + 280, row1_y, sq, sq)).ok();
    // Label: "YIELDS"
    draw_hud_label(canvas, pad + 280 + sq as i32 + 4, row1_y + 2, "YIELDS", Color::RGB(255, 255, 255));
    draw_hud_number(canvas, pad + 280 + sq as i32 + 4, row1_y + 12, traffic.give_ways, Color::RGB(255, 255, 200));

    // Close calls (orange square)
    canvas.set_draw_color(Color::RGB(255, 140, 0));
    canvas.fill_rect(Rect::new(pad + 420, row1_y, sq, sq)).ok();
    // Label: "CLOSE"
    draw_hud_label(canvas, pad + 420 + sq as i32 + 4, row1_y + 2, "CLOSE", Color::RGB(255, 255, 255));
    draw_hud_number(canvas, pad + 420 + sq as i32 + 4, row1_y + 12, traffic.close_calls, Color::RGB(255, 200, 150));

    // Row 2: Velocity indicators with LABELS
    let row2_y = 36;
    
    // Min velocity label and bar (BLUE)
    draw_hud_label(canvas, pad, row2_y - 10, "MIN VEL", Color::RGB(100, 180, 255));
    let min_vel_width = if traffic.min_velocity != i32::MAX {
        ((traffic.min_velocity as f32 / dimensions.speed.fast as f32) * 100.0) as u32
    } else {
        0
    };
    canvas.set_draw_color(Color::RGB(60, 120, 200));
    canvas.fill_rect(Rect::new(pad, row2_y, min_vel_width.max(2), 10)).ok();
    // Show velocity value
    if traffic.min_velocity != i32::MAX {
        draw_hud_number(canvas, pad + 105, row2_y, traffic.min_velocity, Color::RGB(180, 220, 255));
    }

    // Max velocity label and bar (RED)
    draw_hud_label(canvas, pad, row2_y + 18, "MAX VEL", Color::RGB(255, 150, 150));
    let max_vel_width = ((traffic.max_velocity as f32 / dimensions.speed.fast as f32) * 100.0) as u32;
    canvas.set_draw_color(Color::RGB(220, 60, 60));
    canvas.fill_rect(Rect::new(pad, row2_y + 28, max_vel_width.max(2), 10)).ok();
    // Show velocity value
    draw_hud_number(canvas, pad + 105, row2_y + 28, traffic.max_velocity, Color::RGB(255, 200, 200));

    // Row 3: FPS and time stats with LABEL
    let row3_y = 64;
    
    // FPS Label (top-right corner)
    let fps_label_x = dimensions.window_width as i32 - 130;
    draw_hud_label(canvas, fps_label_x, row3_y - 10, "FPS", Color::RGB(200, 200, 200));
    
    // FPS bar: draw a horizontal indicator proportional to FPS (clamped)
    let fps_clamped = fps.min(120.0).max(0.0);
    let max_bar_w = 80u32;
    let bar_w = ((fps_clamped / 120.0) * max_bar_w as f64) as u32;
    let bar_x = dimensions.window_width as i32 - (max_bar_w as i32) - 16;
    
    // FPS Background
    canvas.set_draw_color(Color::RGB(40, 40, 40));
    canvas.fill_rect(Rect::new(bar_x, row3_y, max_bar_w, 12)).ok();
    
    // FPS Foreground (color changes based on performance)
    let fps_color = if fps >= 50.0 {
        Color::RGB(100, 220, 100)  // Green: good
    } else if fps >= 30.0 {
        Color::RGB(220, 220, 100)  // Yellow: ok
    } else {
        Color::RGB(220, 100, 100)  // Red: poor
    };
    canvas.set_draw_color(fps_color);
    canvas.fill_rect(Rect::new(bar_x, row3_y, bar_w, 12)).ok();
    
    // FPS number value
    draw_hud_number(canvas, bar_x + max_bar_w as i32 + 4, row3_y + 2, fps as i32, fps_color);

    // Update the window title with comprehensive stats
    let window = canvas.window_mut();
    let active_cars = traffic.cars.len();
    let safety_status = if traffic.close_calls == 0 {
        "✓"
    } else if traffic.close_calls < 5 {
        "⚠"
    } else {
        "✗"
    };
    
    let _ = window.set_title(&format!(
        "Smart Road {} | FPS: {:.0} | Active: {} | Passed: {} | Give ways: {} | Close calls: {} | Press H for help",
        safety_status, fps, active_cars, traffic.cars_passed, traffic.give_ways, traffic.close_calls
    ));

    // Draw help overlay if enabled
    if show_help {
        draw_help_overlay(canvas, dimensions);
    }

    canvas.present();
}

fn draw_help_overlay(canvas: &mut Canvas<sdl2::video::Window>, dimensions: &Dimensions) {
    // Semi-transparent dark overlay covering most of screen
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 200));
    let help_width = 500u32;
    let help_height = 440u32;
    let help_x = (dimensions.window_width as u32 - help_width) / 2;
    let help_y = (dimensions.window_height as u32 - help_height) / 2;
    
    canvas
        .fill_rect(Rect::new(help_x as i32, help_y as i32, help_width, help_height))
        .ok();
    
    // Border with glow effect
    canvas.set_draw_color(Color::RGB(100, 180, 255));
    for i in 0..4 {
        canvas
            .draw_rect(Rect::new(
                help_x as i32 + i,
                help_y as i32 + i,
                help_width - (i * 2) as u32,
                help_height - (i * 2) as u32,
            ))
            .ok();
    }
    
    // Title bar with gradient
    canvas.set_draw_color(Color::RGB(60, 120, 180));
    canvas
        .fill_rect(Rect::new(help_x as i32, help_y as i32, help_width, 50))
        .ok();
    canvas.set_draw_color(Color::RGB(80, 140, 200));
    canvas
        .fill_rect(Rect::new(help_x as i32, help_y as i32, help_width, 25))
        .ok();
    
    // "CONTROLS" title using simple pixel art-style letters
    let title_x = help_x as i32 + help_width as i32 / 2 - 60;
    let title_y = help_y as i32 + 15;
    draw_text_controls(canvas, title_x, title_y);
    
    let start_x = help_x as i32 + 30;
    let start_y = help_y as i32 + 70;
    let key_size = 28u32;
    let row_height = 48;
    let label_offset = 40;
    
    // Arrow keys section with visual car icons and directional indicators
    let controls = [
        (0, Color::RGB(220, 60, 60), "↑", "Spawn Red car from SOUTH"),
        (1, Color::RGB(60, 220, 60), "↓", "Spawn Green car from NORTH"),
        (2, Color::RGB(220, 220, 60), "←", "Spawn Yellow car from EAST"),
        (3, Color::RGB(60, 120, 220), "→", "Spawn Blue car from WEST"),
        (4, Color::RGB(180, 100, 220), "R", "Spawn RANDOM cars continuously"),
        (5, Color::RGB(100, 200, 200), "F", "Toggle FULLSCREEN mode"),
        (6, Color::RGB(220, 100, 100), "ESC", "Exit and show STATISTICS"),
        (7, Color::RGB(150, 150, 150), "H", "Toggle this HELP panel"),
    ];
    
    for (idx, color, symbol, _desc) in controls.iter() {
        let y_pos = start_y + (idx * row_height);
        
        // Draw key button with 3D effect
        // Shadow
        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas
            .fill_rect(Rect::new(start_x + 2, y_pos + 2, key_size, key_size))
            .ok();
        
        // Main key
        canvas.set_draw_color(*color);
        canvas
            .fill_rect(Rect::new(start_x, y_pos, key_size, key_size))
            .ok();
        
        // Highlight
        canvas.set_draw_color(Color::RGBA(255, 255, 255, 80));
        canvas
            .fill_rect(Rect::new(start_x, y_pos, key_size, key_size / 3))
            .ok();
        
        // Border
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas
            .draw_rect(Rect::new(start_x, y_pos, key_size, key_size))
            .ok();
        
        // Draw symbol/indicator on key
        let symbol_x = start_x + key_size as i32 / 2;
        let symbol_y = y_pos + key_size as i32 / 2;
        
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        match *symbol {
            "↑" => {
                // Up arrow
                for i in 0..8 {
                    canvas.draw_line(
                        (symbol_x - i as i32 / 2, symbol_y + i as i32 / 2 - 4),
                        (symbol_x + i as i32 / 2, symbol_y + i as i32 / 2 - 4),
                    ).ok();
                }
            }
            "↓" => {
                // Down arrow
                for i in 0..8 {
                    canvas.draw_line(
                        (symbol_x - i as i32 / 2, symbol_y - i as i32 / 2 + 4),
                        (symbol_x + i as i32 / 2, symbol_y - i as i32 / 2 + 4),
                    ).ok();
                }
            }
            "←" => {
                // Left arrow
                for i in 0..8 {
                    canvas.draw_line(
                        (symbol_x + i as i32 / 2 - 4, symbol_y - i as i32 / 2),
                        (symbol_x + i as i32 / 2 - 4, symbol_y + i as i32 / 2),
                    ).ok();
                }
            }
            "→" => {
                // Right arrow
                for i in 0..8 {
                    canvas.draw_line(
                        (symbol_x - i as i32 / 2 + 4, symbol_y - i as i32 / 2),
                        (symbol_x - i as i32 / 2 + 4, symbol_y + i as i32 / 2),
                    ).ok();
                }
            }
            "R" => {
                // R letter (simple)
                canvas.fill_rect(Rect::new(symbol_x - 4, symbol_y - 6, 3, 12)).ok();
                canvas.fill_rect(Rect::new(symbol_x - 4, symbol_y - 6, 6, 3)).ok();
                canvas.fill_rect(Rect::new(symbol_x - 4, symbol_y - 1, 6, 3)).ok();
                canvas.fill_rect(Rect::new(symbol_x + 2, symbol_y - 6, 3, 7)).ok();
            }
            "F" => {
                // F letter
                canvas.fill_rect(Rect::new(symbol_x - 4, symbol_y - 6, 3, 12)).ok();
                canvas.fill_rect(Rect::new(symbol_x - 4, symbol_y - 6, 8, 3)).ok();
                canvas.fill_rect(Rect::new(symbol_x - 4, symbol_y - 1, 6, 3)).ok();
            }
            "ESC" => {
                // ESC (simple representation)
                canvas.fill_rect(Rect::new(symbol_x - 6, symbol_y - 4, 4, 8)).ok();
                canvas.fill_rect(Rect::new(symbol_x, symbol_y - 4, 4, 8)).ok();
            }
            "H" => {
                // H letter
                canvas.fill_rect(Rect::new(symbol_x - 6, symbol_y - 6, 3, 12)).ok();
                canvas.fill_rect(Rect::new(symbol_x + 3, symbol_y - 6, 3, 12)).ok();
                canvas.fill_rect(Rect::new(symbol_x - 6, symbol_y - 1, 12, 3)).ok();
            }
            _ => {}
        }
        
        // Draw descriptive icon/visual next to key
        let icon_x = start_x + label_offset;
        let icon_y = y_pos + 4;
        
        match *idx {
            0..=3 => {
                // Draw mini car icon
                canvas.set_draw_color(*color);
                canvas.fill_rect(Rect::new(icon_x, icon_y, 16, 20)).ok();
                canvas.set_draw_color(Color::RGB(50, 50, 70));
                canvas.fill_rect(Rect::new(icon_x + 3, icon_y + 4, 10, 4)).ok();
                canvas.fill_rect(Rect::new(icon_x + 3, icon_y + 12, 10, 4)).ok();
            }
            4 => {
                // Random symbol (multiple mini cars)
                canvas.set_draw_color(Color::RGB(220, 60, 60));
                canvas.fill_rect(Rect::new(icon_x, icon_y, 8, 10)).ok();
                canvas.set_draw_color(Color::RGB(60, 220, 60));
                canvas.fill_rect(Rect::new(icon_x + 10, icon_y + 5, 8, 10)).ok();
                canvas.set_draw_color(Color::RGB(60, 120, 220));
                canvas.fill_rect(Rect::new(icon_x + 20, icon_y + 10, 8, 10)).ok();
            }
            5 => {
                // Fullscreen icon (expanding arrows)
                canvas.set_draw_color(Color::RGB(100, 200, 200));
                for i in 0..3 {
                    let offset = (i * 5) as i32;
                    let size_w = 20 - i * 8;
                    let size_h = 20 - i * 6;
                    canvas.draw_rect(Rect::new(icon_x + offset, icon_y + offset, size_w, size_h)).ok();
                }
            }
            6 => {
                // Exit door icon
                canvas.set_draw_color(Color::RGB(220, 100, 100));
                canvas.fill_rect(Rect::new(icon_x, icon_y, 24, 20)).ok();
                canvas.set_draw_color(Color::RGB(50, 50, 50));
                canvas.fill_rect(Rect::new(icon_x + 4, icon_y + 4, 16, 12)).ok();
            }
            7 => {
                // Question mark / help icon
                canvas.set_draw_color(Color::RGB(150, 150, 150));
                canvas.fill_rect(Rect::new(icon_x + 8, icon_y, 4, 12)).ok();
                canvas.fill_rect(Rect::new(icon_x + 4, icon_y, 8, 4)).ok();
                canvas.fill_rect(Rect::new(icon_x + 8, icon_y + 16, 4, 4)).ok();
            }
            _ => {}
        }
        
        // Draw simple text description using pixel-style letters
        let text_x = start_x + label_offset + 35;
        let text_y = y_pos + 8;
        draw_simple_text(canvas, text_x, text_y, *symbol, *idx as usize);
    }
    
    // Footer note
    canvas.set_draw_color(Color::RGB(150, 150, 150));
    let footer_y = help_y as i32 + help_height as i32 - 25;
    canvas.fill_rect(Rect::new(help_x as i32 + 20, footer_y, help_width - 40, 2)).ok();
}

fn draw_text_controls(canvas: &mut Canvas<sdl2::video::Window>, x: i32, y: i32) {
    // LARGER "CONTROLS" text with better visibility (doubled size)
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    
    let scale = 2i32; // Make it bigger
    let spacing = 24i32;
    
    // C
    canvas.fill_rect(Rect::new(x, y, (4*scale) as u32, (24*scale) as u32)).ok();
    canvas.fill_rect(Rect::new(x, y, (16*scale) as u32, (4*scale) as u32)).ok();
    canvas.fill_rect(Rect::new(x, y + 20*scale, (16*scale) as u32, (4*scale) as u32)).ok();
    
    // O
    canvas.fill_rect(Rect::new(x + spacing*scale, y, (4*scale) as u32, (24*scale) as u32)).ok();
    canvas.fill_rect(Rect::new(x + spacing*scale, y, (16*scale) as u32, (4*scale) as u32)).ok();
    canvas.fill_rect(Rect::new(x + spacing*scale, y + 20*scale, (16*scale) as u32, (4*scale) as u32)).ok();
    canvas.fill_rect(Rect::new(x + (spacing+12)*scale, y, (4*scale) as u32, (24*scale) as u32)).ok();
    
    // N
    canvas.fill_rect(Rect::new(x + (spacing*2+6)*scale, y, (4*scale) as u32, (24*scale) as u32)).ok();
    canvas.fill_rect(Rect::new(x + (spacing*2+18)*scale, y, (4*scale) as u32, (24*scale) as u32)).ok();
    canvas.fill_rect(Rect::new(x + (spacing*2+6)*scale, y, (16*scale) as u32, (4*scale) as u32)).ok();
    
    // T
    canvas.fill_rect(Rect::new(x + (spacing*3+6)*scale, y, (16*scale) as u32, (4*scale) as u32)).ok();
    canvas.fill_rect(Rect::new(x + (spacing*3+12)*scale, y, (4*scale) as u32, (24*scale) as u32)).ok();
    
    // R
    canvas.fill_rect(Rect::new(x + (spacing*4+6)*scale, y, (4*scale) as u32, (24*scale) as u32)).ok();
    canvas.fill_rect(Rect::new(x + (spacing*4+6)*scale, y, (14*scale) as u32, (4*scale) as u32)).ok();
    canvas.fill_rect(Rect::new(x + (spacing*4+6)*scale, y + 10*scale, (14*scale) as u32, (4*scale) as u32)).ok();
    canvas.fill_rect(Rect::new(x + (spacing*4+16)*scale, y, (4*scale) as u32, (14*scale) as u32)).ok();
    canvas.fill_rect(Rect::new(x + (spacing*4+12)*scale, y + 14*scale, (8*scale) as u32, (10*scale) as u32)).ok();
    
    // O
    canvas.fill_rect(Rect::new(x + (spacing*5+12)*scale, y, (4*scale) as u32, (24*scale) as u32)).ok();
    canvas.fill_rect(Rect::new(x + (spacing*5+12)*scale, y, (16*scale) as u32, (4*scale) as u32)).ok();
    canvas.fill_rect(Rect::new(x + (spacing*5+12)*scale, y + 20*scale, (16*scale) as u32, (4*scale) as u32)).ok();
    canvas.fill_rect(Rect::new(x + (spacing*5+24)*scale, y, (4*scale) as u32, (24*scale) as u32)).ok();
    
    // L
    canvas.fill_rect(Rect::new(x + (spacing*6+18)*scale, y, (4*scale) as u32, (24*scale) as u32)).ok();
    canvas.fill_rect(Rect::new(x + (spacing*6+18)*scale, y + 20*scale, (16*scale) as u32, (4*scale) as u32)).ok();
    
    // S
    canvas.fill_rect(Rect::new(x + (spacing*7+18)*scale, y, (16*scale) as u32, (4*scale) as u32)).ok();
    canvas.fill_rect(Rect::new(x + (spacing*7+18)*scale, y, (4*scale) as u32, (12*scale) as u32)).ok();
    canvas.fill_rect(Rect::new(x + (spacing*7+18)*scale, y + 10*scale, (16*scale) as u32, (4*scale) as u32)).ok();
    canvas.fill_rect(Rect::new(x + (spacing*7+30)*scale, y + 12*scale, (4*scale) as u32, (12*scale) as u32)).ok();
    canvas.fill_rect(Rect::new(x + (spacing*7+18)*scale, y + 20*scale, (16*scale) as u32, (4*scale) as u32)).ok();
    canvas.fill_rect(Rect::new(x + 99, y, 3, 18)).ok();
    
    // L
    canvas.fill_rect(Rect::new(x + 108, y, 3, 18)).ok();
    canvas.fill_rect(Rect::new(x + 108, y + 15, 12, 3)).ok();
}

fn draw_simple_text(canvas: &mut Canvas<sdl2::video::Window>, x: i32, y: i32, _symbol: &str, idx: usize) {
    canvas.set_draw_color(Color::RGB(200, 200, 200));
    
    let descriptions = [
        "SOUTH to NORTH",
        "NORTH to SOUTH",
        "EAST to WEST",
        "WEST to EAST",
        "RANDOM MODE",
        "FULLSCREEN",
        "EXIT & STATS",
        "TOGGLE HELP",
    ];
    
    if idx < descriptions.len() {
        // Draw simplified text representation using basic shapes
        for (i, ch) in descriptions[idx].chars().enumerate() {
            let char_x = x + (i as i32 * 7);
            match ch {
                'A' | 'M' | 'W' => {
                    canvas.fill_rect(Rect::new(char_x, y, 2, 10)).ok();
                    canvas.fill_rect(Rect::new(char_x + 4, y, 2, 10)).ok();
                    canvas.fill_rect(Rect::new(char_x, y, 6, 2)).ok();
                }
                'E' => {
                    canvas.fill_rect(Rect::new(char_x, y, 2, 10)).ok();
                    canvas.fill_rect(Rect::new(char_x, y, 5, 2)).ok();
                    canvas.fill_rect(Rect::new(char_x, y + 4, 4, 2)).ok();
                    canvas.fill_rect(Rect::new(char_x, y + 8, 5, 2)).ok();
                }
                'F' | 'L' | 'T' | 'H' | 'N' | 'O' | 'R' | 'S' | 'U' | 'D' | 'G' | 'X' | 'P' => {
                    canvas.fill_rect(Rect::new(char_x, y, 2, 10)).ok();
                    canvas.fill_rect(Rect::new(char_x, y, 5, 2)).ok();
                }
                ' ' => {
                    // Space - do nothing
                }
                _ => {
                    canvas.fill_rect(Rect::new(char_x, y + 8, 4, 2)).ok();
                }
            }
        }
    }
}

// Helper function to draw HUD labels with pixel-art style
fn draw_hud_label(canvas: &mut Canvas<Window>, x: i32, y: i32, text: &str, color: Color) {
    canvas.set_draw_color(color);
    let mut offset_x = 0;
    
    for ch in text.chars() {
        let char_x = x + offset_x;
        match ch {
            'A' => {
                // A shape
                canvas.fill_rect(Rect::new(char_x, y + 1, 4, 1)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 2, 1, 6)).ok();
                canvas.fill_rect(Rect::new(char_x + 3, y + 2, 1, 6)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 4, 4, 1)).ok();
            }
            'C' => {
                // C shape
                canvas.fill_rect(Rect::new(char_x, y + 1, 4, 1)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 2, 1, 5)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 7, 4, 1)).ok();
            }
            'D' => {
                // D shape
                canvas.fill_rect(Rect::new(char_x, y + 1, 1, 7)).ok();
                canvas.fill_rect(Rect::new(char_x + 1, y + 1, 2, 1)).ok();
                canvas.fill_rect(Rect::new(char_x + 1, y + 7, 2, 1)).ok();
                canvas.fill_rect(Rect::new(char_x + 3, y + 2, 1, 5)).ok();
            }
            'E' => {
                // E shape
                canvas.fill_rect(Rect::new(char_x, y + 1, 1, 7)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 1, 4, 1)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 4, 3, 1)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 7, 4, 1)).ok();
            }
            'I' => {
                // I shape
                canvas.fill_rect(Rect::new(char_x, y + 1, 1, 7)).ok();
            }
            'L' => {
                // L shape
                canvas.fill_rect(Rect::new(char_x, y + 1, 1, 7)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 7, 4, 1)).ok();
            }
            'O' => {
                // O shape
                canvas.fill_rect(Rect::new(char_x, y + 1, 4, 1)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 2, 1, 5)).ok();
                canvas.fill_rect(Rect::new(char_x + 3, y + 2, 1, 5)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 7, 4, 1)).ok();
            }
            'P' => {
                // P shape
                canvas.fill_rect(Rect::new(char_x, y + 1, 1, 7)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 1, 4, 1)).ok();
                canvas.fill_rect(Rect::new(char_x + 3, y + 2, 1, 2)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 4, 4, 1)).ok();
            }
            'S' => {
                // S shape
                canvas.fill_rect(Rect::new(char_x, y + 1, 4, 1)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 2, 1, 2)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 4, 4, 1)).ok();
                canvas.fill_rect(Rect::new(char_x + 3, y + 5, 1, 2)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 7, 4, 1)).ok();
            }
            'T' => {
                // T shape
                canvas.fill_rect(Rect::new(char_x, y + 1, 5, 1)).ok();
                canvas.fill_rect(Rect::new(char_x + 2, y + 2, 1, 6)).ok();
            }
            'V' => {
                // V shape
                canvas.fill_rect(Rect::new(char_x, y + 1, 1, 5)).ok();
                canvas.fill_rect(Rect::new(char_x + 1, y + 6, 1, 1)).ok();
                canvas.fill_rect(Rect::new(char_x + 2, y + 7, 1, 1)).ok();
                canvas.fill_rect(Rect::new(char_x + 3, y + 6, 1, 1)).ok();
                canvas.fill_rect(Rect::new(char_x + 4, y + 1, 1, 5)).ok();
            }
            'Y' => {
                // Y shape
                canvas.fill_rect(Rect::new(char_x, y + 1, 1, 3)).ok();
                canvas.fill_rect(Rect::new(char_x + 4, y + 1, 1, 3)).ok();
                canvas.fill_rect(Rect::new(char_x + 1, y + 4, 3, 1)).ok();
                canvas.fill_rect(Rect::new(char_x + 2, y + 5, 1, 3)).ok();
            }
            _ => {
                // Unknown character - skip
            }
        }
        offset_x += 5;
    }
}

// Helper function to draw numbers in HUD
fn draw_hud_number(canvas: &mut Canvas<Window>, x: i32, y: i32, number: i32, color: Color) {
    canvas.set_draw_color(color);
    let text = format!("{}", number);
    let mut offset_x = 0;
    
    for ch in text.chars() {
        let char_x = x + offset_x;
        match ch {
            '0' => {
                canvas.fill_rect(Rect::new(char_x, y, 3, 1)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 1, 1, 6)).ok();
                canvas.fill_rect(Rect::new(char_x + 2, y + 1, 1, 6)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 7, 3, 1)).ok();
            }
            '1' => {
                canvas.fill_rect(Rect::new(char_x + 1, y, 1, 8)).ok();
            }
            '2' => {
                canvas.fill_rect(Rect::new(char_x, y, 3, 1)).ok();
                canvas.fill_rect(Rect::new(char_x + 2, y + 1, 1, 3)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 4, 3, 1)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 5, 1, 2)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 7, 3, 1)).ok();
            }
            '3' => {
                canvas.fill_rect(Rect::new(char_x, y, 3, 1)).ok();
                canvas.fill_rect(Rect::new(char_x + 2, y + 1, 1, 2)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 3, 3, 1)).ok();
                canvas.fill_rect(Rect::new(char_x + 2, y + 4, 1, 3)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 7, 3, 1)).ok();
            }
            '4' => {
                canvas.fill_rect(Rect::new(char_x, y, 1, 4)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 4, 3, 1)).ok();
                canvas.fill_rect(Rect::new(char_x + 2, y, 1, 8)).ok();
            }
            '5' => {
                canvas.fill_rect(Rect::new(char_x, y, 3, 1)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 1, 1, 2)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 3, 3, 1)).ok();
                canvas.fill_rect(Rect::new(char_x + 2, y + 4, 1, 3)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 7, 3, 1)).ok();
            }
            '6' => {
                canvas.fill_rect(Rect::new(char_x, y, 3, 1)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 1, 1, 6)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 4, 3, 1)).ok();
                canvas.fill_rect(Rect::new(char_x + 2, y + 5, 1, 2)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 7, 3, 1)).ok();
            }
            '7' => {
                canvas.fill_rect(Rect::new(char_x, y, 3, 1)).ok();
                canvas.fill_rect(Rect::new(char_x + 2, y + 1, 1, 7)).ok();
            }
            '8' => {
                canvas.fill_rect(Rect::new(char_x, y, 3, 1)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 1, 1, 2)).ok();
                canvas.fill_rect(Rect::new(char_x + 2, y + 1, 1, 2)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 3, 3, 1)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 4, 1, 3)).ok();
                canvas.fill_rect(Rect::new(char_x + 2, y + 4, 1, 3)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 7, 3, 1)).ok();
            }
            '9' => {
                canvas.fill_rect(Rect::new(char_x, y, 3, 1)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 1, 1, 2)).ok();
                canvas.fill_rect(Rect::new(char_x + 2, y + 1, 1, 6)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 3, 3, 1)).ok();
                canvas.fill_rect(Rect::new(char_x, y + 7, 3, 1)).ok();
            }
            _ => {
                // Unknown character
            }
        }
        offset_x += 4;
    }
}
