use rand::Rng;
use sdl2::{
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::{BlendMode, Canvas, Texture, TextureCreator},
    surface::Surface,
    video::WindowContext,
};

use crate::{lanes, types::Dimensions};

pub fn create_textures<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    dimensions: &Dimensions,
    canvas: &mut Canvas<sdl2::video::Window>,
) -> (
    Texture<'a>,
    Texture<'a>,
    [Texture<'a>; 4],
    Vec<(Texture<'a>, [f64; 2])>,
) {
    let background_texture = create_speckled_texture(
        &texture_creator,
        dimensions.window_width as u32,
        dimensions.window_height as u32,
        canvas,
    );
    let lanes_texture = lanes::draw(canvas, &dimensions, &texture_creator);
    let car_textures = create_car_textures(&texture_creator, &dimensions);

    // Previously these were loaded from files under `images/`.
    // Generate enhanced procedural tree textures with gradients and depth
    let mut tree_textures: Vec<(Texture, [f64; 2])> = Vec::new();

    // Helper to create an enhanced tree with gradient, shadow, and depth
    let make_tree = |w: u32, h: u32, trunk_color: Color, foliage_color: Color, has_shadow: bool| {
        let mut surf = Surface::new(w, h, PixelFormatEnum::RGBA8888)
            .expect("Failed to create tree surface");
        surf.fill_rect(None, Color::RGBA(0, 0, 0, 0)).ok();

        // Draw shadow if enabled (for larger trees)
        if has_shadow {
            let shadow_w = (w as f32 * 0.4) as u32;
            let shadow_h = (h as f32 * 0.08) as u32;
            let shadow_x = (w / 2) as i32 - (shadow_w as i32 / 2);
            let shadow_y = h as i32 - shadow_h as i32;
            surf.fill_rect(
                Rect::new(shadow_x, shadow_y, shadow_w, shadow_h),
                Color::RGBA(0, 0, 0, 60),
            ).ok();
        }

        // Draw trunk with gradient effect (darker on left, lighter on right)
        let trunk_w = (w / 5).max(2);
        let trunk_h = (h / 3).max(4);
        let trunk_x = (w / 2) as i32 - (trunk_w as i32 / 2);
        let trunk_y = (h - trunk_h) as i32;
        
        // Main trunk
        surf.fill_rect(
            Rect::new(trunk_x, trunk_y, trunk_w, trunk_h),
            trunk_color,
        ).ok();
        
        // Trunk highlight (lighter right side for depth)
        let highlight_w = trunk_w / 3;
        surf.fill_rect(
            Rect::new(trunk_x + trunk_w as i32 - highlight_w as i32, trunk_y, highlight_w, trunk_h),
            Color::RGB(
                trunk_color.r.saturating_add(20),
                trunk_color.g.saturating_add(15),
                trunk_color.b.saturating_add(10),
            ),
        ).ok();
        
        // Trunk shadow (darker left side)
        let shadow_w = trunk_w / 4;
        surf.fill_rect(
            Rect::new(trunk_x, trunk_y, shadow_w, trunk_h),
            Color::RGB(
                trunk_color.r.saturating_sub(15),
                trunk_color.g.saturating_sub(10),
                trunk_color.b.saturating_sub(5),
            ),
        ).ok();

        // Draw foliage with layered effect for depth
        let leaf_w = (w * 3 / 4).max(4);
        let leaf_h = (h * 2 / 3).max(4);
        let leaf_x = (w / 2) as i32 - (leaf_w as i32 / 2);
        let leaf_y = (h / 6) as i32;
        
        // Back layer (darker, larger)
        surf.fill_rect(
            Rect::new(leaf_x - 2, leaf_y + 2, leaf_w + 4, leaf_h + 4),
            Color::RGB(
                foliage_color.r.saturating_sub(30),
                foliage_color.g.saturating_sub(30),
                foliage_color.b.saturating_sub(20),
            ),
        ).ok();
        
        // Main foliage
        surf.fill_rect(Rect::new(leaf_x, leaf_y, leaf_w, leaf_h), foliage_color).ok();
        
        // Highlight spots (lighter patches for realism)
        let highlight_spots = [
            (leaf_x + (leaf_w / 3) as i32, leaf_y + (leaf_h / 4) as i32, leaf_w / 4, leaf_h / 4),
            (leaf_x + (leaf_w * 2 / 3) as i32, leaf_y + (leaf_h / 2) as i32, leaf_w / 5, leaf_h / 5),
        ];
        
        for (hx, hy, hw, hh) in highlight_spots {
            surf.fill_rect(
                Rect::new(hx, hy, hw, hh),
                Color::RGB(
                    foliage_color.r.saturating_add(40),
                    foliage_color.g.saturating_add(40),
                    foliage_color.b.saturating_add(30),
                ),
            ).ok();
        }
        
        // Add some texture detail with small darker spots
        let mut rng = rand::thread_rng();
        for _ in 0..8 {
            let spot_x = leaf_x + rng.gen_range(0..leaf_w as i32);
            let spot_y = leaf_y + rng.gen_range(0..leaf_h as i32);
            let spot_size = rng.gen_range(2..5);
            surf.fill_rect(
                Rect::new(spot_x, spot_y, spot_size, spot_size),
                Color::RGB(
                    foliage_color.r.saturating_sub(20),
                    foliage_color.g.saturating_sub(20),
                    foliage_color.b.saturating_sub(15),
                ),
            ).ok();
        }

        let tex = texture_creator
            .create_texture_from_surface(&surf)
            .expect("Failed to create tree texture");
        (tex, [w as f64, h as f64])
    };

    // Create varied trees with different sizes and colors
    tree_textures.push(make_tree(200, dimensions.window_height as u32, Color::RGB(90, 50, 20), Color::RGB(34, 139, 34), true));
    tree_textures.push(make_tree(120, 220, Color::RGB(75, 45, 18), Color::RGB(60, 179, 113), true));
    tree_textures.push(make_tree(100, 200, Color::RGB(85, 50, 22), Color::RGB(40, 150, 40), true));
    tree_textures.push(make_tree(64, 64, Color::RGB(80, 40, 18), Color::RGB(50, 160, 90), false));
    tree_textures.push(make_tree(128, 160, Color::RGB(92, 52, 24), Color::RGB(45, 155, 50), true));

    (
        background_texture,
        lanes_texture,
        car_textures,
        tree_textures,
    )
}

// The old `create_texture_from_image` helper (which attempted to open files under
// `images/`) has been removed because tree assets are now generated procedurally.

pub fn create_speckled_texture<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    width: u32,
    height: u32,
    canvas: &mut Canvas<sdl2::video::Window>,
) -> Texture<'a> {
    // Create texture with blending enabled.
    let mut texture = texture_creator
        .create_texture_target(Some(PixelFormatEnum::RGBA8888), width, height)
        .expect("Failed to create texture target");

    texture.set_blend_mode(BlendMode::Blend);

    canvas
        .with_texture_canvas(&mut texture, |texture_canvas| {
            // Clear with fully transparent color
            texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
            texture_canvas.clear();

            let mut rng = rand::thread_rng();
            
            // Add varied sizes and opacity for more natural look
            for _ in 0..400 {
                let x = rng.gen_range(0..width as i32);
                let y = rng.gen_range(0..height as i32);
                let size = rng.gen_range(1..5);
                let gray = rng.gen_range(120..240);
                let alpha = rng.gen_range(180..255);

                texture_canvas.set_draw_color(Color::RGBA(gray, gray, gray, alpha));
                texture_canvas
                    .fill_rect(Rect::new(x, y, size, size))
                    .expect("Failed to fill rect");
            }
            
            // Add some colored speckles for visual interest
            for _ in 0..50 {
                let x = rng.gen_range(0..width as i32);
                let y = rng.gen_range(0..height as i32);
                let size = rng.gen_range(1..3);
                
                // Subtle blue/green tints
                let color_variant = rng.gen_range(0..3);
                let color = match color_variant {
                    0 => Color::RGBA(200, 220, 255, 40), // Subtle blue
                    1 => Color::RGBA(220, 255, 220, 40), // Subtle green
                    _ => Color::RGBA(255, 250, 220, 40), // Subtle yellow
                };
                
                texture_canvas.set_draw_color(color);
                texture_canvas
                    .fill_rect(Rect::new(x, y, size, size))
                    .expect("Failed to fill rect");
            }
        })
        .expect("Failed to render speckled texture");

    texture
}

fn create_car_textures<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    dimensions: &Dimensions,
) -> [sdl2::render::Texture<'a>; 4] {
    [
        create_car_texture(texture_creator, dimensions, Color::RGB(255, 0, 0)),
        create_car_texture(texture_creator, dimensions, Color::RGB(0, 255, 0)),
        create_car_texture(texture_creator, dimensions, Color::RGB(0, 0, 255)),
        create_car_texture(texture_creator, dimensions, Color::RGB(255, 255, 0)),
    ]
}

fn create_car_texture<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    dimensions: &Dimensions,
    body_color: Color,
) -> sdl2::render::Texture<'a> {
    let lane_width = dimensions.lane_width as u32;

    // Use RGBA format to support transparency
    let mut car_surface = sdl2::surface::Surface::new(
        lane_width,
        lane_width,
        sdl2::pixels::PixelFormatEnum::RGBA8888,
    )
    .expect("Failed to create car surface");

    car_surface
        .fill_rect(
            Rect::new(0, 0, lane_width, lane_width),
            Color::RGBA(0, 0, 0, 0),
        )
        .unwrap();

    // Calculate proportional dimensions
    let body_w = (lane_width as f32 * 0.5) as u32;
    let body_h = (lane_width as f32 * 0.875) as u32;
    let body_x = ((lane_width - body_w) / 2) as i32;
    let body_y = ((lane_width - body_h) / 2) as i32;

    // Draw shadow/outline (darker body)
    let shadow_offset = 1;
    car_surface
        .fill_rect(
            Rect::new(
                body_x + shadow_offset,
                body_y + shadow_offset,
                body_w,
                body_h,
            ),
            Color::RGB(
                body_color.r.saturating_sub(60),
                body_color.g.saturating_sub(60),
                body_color.b.saturating_sub(60),
            ),
        )
        .unwrap();

    // Draw main body
    car_surface
        .fill_rect(Rect::new(body_x, body_y, body_w, body_h), body_color)
        .unwrap();

    // Add gradient/highlight on top half for 3D effect
    let highlight_h = body_h / 3;
    car_surface
        .fill_rect(
            Rect::new(body_x, body_y, body_w, highlight_h),
            Color::RGB(
                body_color.r.saturating_add(40),
                body_color.g.saturating_add(40),
                body_color.b.saturating_add(40),
            ),
        )
        .unwrap();

    // Draw windows (front and rear with realistic sizing)
    let win_w = (lane_width as f32 * 0.375) as u32;
    let win_h = (lane_width as f32 * 0.15) as u32;
    let win_x = ((lane_width as f32 - win_w as f32) / 2.0) as i32;
    let win1_y = (lane_width as f32 * 0.30) as i32;
    let win2_y = (lane_width as f32 * 0.55) as i32;

    // Window glass (dark with slight blue tint)
    let window_color = Color::RGB(40, 50, 70);
    car_surface
        .fill_rect(Rect::new(win_x, win1_y, win_w, win_h), window_color)
        .unwrap();
    car_surface
        .fill_rect(Rect::new(win_x, win2_y, win_w, win_h), window_color)
        .unwrap();

    // Window highlights (reflection effect)
    let highlight_color = Color::RGBA(150, 170, 200, 120);
    let highlight_w = win_w / 3;
    let highlight_h = win_h / 2;
    car_surface
        .fill_rect(
            Rect::new(win_x + 1, win1_y + 1, highlight_w, highlight_h),
            highlight_color,
        )
        .unwrap();
    car_surface
        .fill_rect(
            Rect::new(win_x + 1, win2_y + 1, highlight_w, highlight_h),
            highlight_color,
        )
        .unwrap();

    // Draw headlights (brighter and more prominent)
    let light_w = (lane_width as f32 * 0.15) as u32;
    let light_h = (lane_width as f32 * 0.08) as u32;
    let light1_x = (lane_width as f32 * 0.28) as i32;
    let light2_x = (lane_width as f32 * 0.57) as i32;
    let light_y = (lane_width as f32 * 0.08) as i32;

    // Headlight glow effect
    car_surface
        .fill_rect(
            Rect::new(light1_x - 1, light_y - 1, light_w + 2, light_h + 2),
            Color::RGB(255, 255, 200),
        )
        .unwrap();
    car_surface
        .fill_rect(
            Rect::new(light2_x - 1, light_y - 1, light_w + 2, light_h + 2),
            Color::RGB(255, 255, 200),
        )
        .unwrap();

    // Bright headlights
    car_surface
        .fill_rect(
            Rect::new(light1_x, light_y, light_w, light_h),
            Color::RGB(255, 255, 240),
        )
        .unwrap();
    car_surface
        .fill_rect(
            Rect::new(light2_x, light_y, light_w, light_h),
            Color::RGB(255, 255, 240),
        )
        .unwrap();

    // Draw tail lights (red, at bottom)
    let tail_y = (lane_width as f32 * 0.88) as i32;
    car_surface
        .fill_rect(
            Rect::new(light1_x, tail_y, light_w, light_h),
            Color::RGB(200, 20, 20),
        )
        .unwrap();
    car_surface
        .fill_rect(
            Rect::new(light2_x, tail_y, light_w, light_h),
            Color::RGB(200, 20, 20),
        )
        .unwrap();

    // Add side mirrors
    let mirror_w = (lane_width as f32 * 0.08) as u32;
    let mirror_h = (lane_width as f32 * 0.1) as u32;
    let mirror_y = (lane_width as f32 * 0.45) as i32;
    
    // Left mirror
    car_surface
        .fill_rect(
            Rect::new(body_x - mirror_w as i32, mirror_y, mirror_w, mirror_h),
            Color::RGB(
                body_color.r.saturating_sub(20),
                body_color.g.saturating_sub(20),
                body_color.b.saturating_sub(20),
            ),
        )
        .unwrap();
    
    // Right mirror
    car_surface
        .fill_rect(
            Rect::new(body_x + body_w as i32, mirror_y, mirror_w, mirror_h),
            Color::RGB(
                body_color.r.saturating_sub(20),
                body_color.g.saturating_sub(20),
                body_color.b.saturating_sub(20),
            ),
        )
        .unwrap();

    texture_creator
        .create_texture_from_surface(&car_surface)
        .expect("Failed to create car texture")
}
