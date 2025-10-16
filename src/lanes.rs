use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{
    pixels::{Color, PixelFormatEnum},
    render::{BlendMode, Texture, TextureCreator},
    video::WindowContext,
};

use crate::types::Dimensions;

pub fn draw<'a>(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    dimensions: &Dimensions,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Texture<'a> {
    let mut texture = texture_creator
        .create_texture_target(
            Some(PixelFormatEnum::RGBA8888),
            dimensions.window_width as u32,
            dimensions.window_height as u32,
        )
        .expect("Failed to create texture target");

    texture.set_blend_mode(BlendMode::Blend);

    canvas
        .with_texture_canvas(&mut texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGB(64, 64, 64));
            texture_canvas
                .fill_rect(sdl2::rect::Rect::new(
                    dimensions.half_width - 3 * dimensions.lane_width,
                    0,
                    6 * dimensions.lane_width as u32,
                    dimensions.window_height as u32,
                ))
                .unwrap();

            texture_canvas
                .fill_rect(sdl2::rect::Rect::new(
                    0,
                    dimensions.half_height - 3 * dimensions.lane_width,
                    dimensions.window_width as u32,
                    6 * dimensions.lane_width as u32,
                ))
                .unwrap();

            texture_canvas.set_draw_color(Color::RGB(255, 255, 255));
            draw_center_lines_to_texture(texture_canvas, dimensions);
            draw_edge_lines_to_texture(texture_canvas, dimensions);
            draw_lane_lines_to_texture(texture_canvas, dimensions);
            draw_give_way_lines_to_texture(texture_canvas, dimensions);
        })
        .expect("Failed to render everything on texture");

    texture
}

fn draw_edge_lines_to_texture(texture_canvas: &mut Canvas<Window>, dimensions: &Dimensions) {
    texture_canvas
        .draw_line(
            (dimensions.half_width - 3 * dimensions.lane_width, 0),
            (
                dimensions.half_width - 3 * dimensions.lane_width,
                dimensions.half_height - 3 * dimensions.lane_width,
            ),
        )
        .unwrap();
    texture_canvas
        .draw_line(
            (
                dimensions.half_width - 3 * dimensions.lane_width,
                dimensions.half_height + 3 * dimensions.lane_width,
            ),
            (
                dimensions.half_width - 3 * dimensions.lane_width,
                dimensions.window_height,
            ),
        )
        .unwrap();

    texture_canvas
        .draw_line(
            (0, dimensions.half_height - 3 * dimensions.lane_width),
            (
                dimensions.half_width - 3 * dimensions.lane_width,
                dimensions.half_height - 3 * dimensions.lane_width,
            ),
        )
        .unwrap();
    texture_canvas
        .draw_line(
            (
                dimensions.half_width + 3 * dimensions.lane_width,
                dimensions.half_height - 3 * dimensions.lane_width,
            ),
            (
                dimensions.window_width,
                dimensions.half_height - 3 * dimensions.lane_width,
            ),
        )
        .unwrap();

    texture_canvas
        .draw_line(
            (dimensions.half_width + 3 * dimensions.lane_width, 0),
            (
                dimensions.half_width + 3 * dimensions.lane_width,
                dimensions.half_height - 3 * dimensions.lane_width,
            ),
        )
        .unwrap();
    texture_canvas
        .draw_line(
            (
                dimensions.half_width + 3 * dimensions.lane_width,
                dimensions.half_height + 3 * dimensions.lane_width,
            ),
            (
                dimensions.half_width + 3 * dimensions.lane_width,
                dimensions.window_height,
            ),
        )
        .unwrap();

    texture_canvas
        .draw_line(
            (0, dimensions.half_height + 3 * dimensions.lane_width),
            (
                dimensions.half_width - 3 * dimensions.lane_width,
                dimensions.half_height + 3 * dimensions.lane_width,
            ),
        )
        .unwrap();
    texture_canvas
        .draw_line(
            (
                dimensions.half_width + 3 * dimensions.lane_width,
                dimensions.half_height + 3 * dimensions.lane_width,
            ),
            (
                dimensions.window_width,
                dimensions.half_height + 3 * dimensions.lane_width,
            ),
        )
        .unwrap();
}

fn draw_give_way_lines_to_texture(texture_canvas: &mut Canvas<Window>, dimensions: &Dimensions) {
    draw_dashed_line_to_texture(
        texture_canvas,
        (
            dimensions.half_width,
            dimensions.half_height + 3 * dimensions.lane_width,
        ),
        (
            dimensions.half_width + 3 * dimensions.lane_width,
            dimensions.half_height + 3 * dimensions.lane_width,
        ),
        4.0,
        4.0,
    );
    draw_dashed_line_to_texture(
        texture_canvas,
        (
            dimensions.half_width,
            dimensions.half_height + 3 * dimensions.lane_width + 8,
        ),
        (
            dimensions.half_width + 3 * dimensions.lane_width,
            dimensions.half_height + 3 * dimensions.lane_width + 8,
        ),
        4.0,
        4.0,
    );

    draw_dashed_line_to_texture(
        texture_canvas,
        (
            dimensions.half_width,
            dimensions.half_height - 3 * dimensions.lane_width,
        ),
        (
            dimensions.half_width - 3 * dimensions.lane_width,
            dimensions.half_height - 3 * dimensions.lane_width,
        ),
        4.0,
        4.0,
    );
    draw_dashed_line_to_texture(
        texture_canvas,
        (
            dimensions.half_width,
            dimensions.half_height - 3 * dimensions.lane_width - 8,
        ),
        (
            dimensions.half_width - 3 * dimensions.lane_width,
            dimensions.half_height - 3 * dimensions.lane_width - 8,
        ),
        4.0,
        4.0,
    );

    draw_dashed_line_to_texture(
        texture_canvas,
        (
            dimensions.half_width + 3 * dimensions.lane_width,
            dimensions.half_height,
        ),
        (
            dimensions.half_width + 3 * dimensions.lane_width,
            dimensions.half_height - 3 * dimensions.lane_width,
        ),
        4.0,
        4.0,
    );
    draw_dashed_line_to_texture(
        texture_canvas,
        (
            dimensions.half_width + 3 * dimensions.lane_width + 8,
            dimensions.half_height,
        ),
        (
            dimensions.half_width + 3 * dimensions.lane_width + 8,
            dimensions.half_height - 3 * dimensions.lane_width,
        ),
        4.0,
        4.0,
    );

    draw_dashed_line_to_texture(
        texture_canvas,
        (
            dimensions.half_width - 3 * dimensions.lane_width,
            dimensions.half_height,
        ),
        (
            dimensions.half_width - 3 * dimensions.lane_width,
            dimensions.half_height + 3 * dimensions.lane_width,
        ),
        4.0,
        4.0,
    );
    draw_dashed_line_to_texture(
        texture_canvas,
        (
            dimensions.half_width - 3 * dimensions.lane_width - 8,
            dimensions.half_height,
        ),
        (
            dimensions.half_width - 3 * dimensions.lane_width - 8,
            dimensions.half_height + 3 * dimensions.lane_width,
        ),
        4.0,
        4.0,
    );
}

fn draw_lane_lines_to_texture(
    texture_canvas: &mut sdl2::render::Canvas<Window>,
    dimensions: &Dimensions,
) {
    for i in 1..3 {
        draw_dashed_line_to_texture(
            texture_canvas,
            (dimensions.half_width - dimensions.lane_width * i, 0),
            (
                dimensions.half_width - dimensions.lane_width * i,
                dimensions.window_height,
            ),
            4.0,
            4.0,
        );
        draw_dashed_line_to_texture(
            texture_canvas,
            (dimensions.half_width + dimensions.lane_width * i, 0),
            (
                dimensions.half_width + dimensions.lane_width * i,
                dimensions.window_height,
            ),
            4.0,
            4.0,
        );
        draw_dashed_line_to_texture(
            texture_canvas,
            (0, dimensions.half_height - dimensions.lane_width * i),
            (
                dimensions.window_width,
                dimensions.half_height - dimensions.lane_width * i,
            ),
            4.0,
            4.0,
        );
        draw_dashed_line_to_texture(
            texture_canvas,
            (0, dimensions.half_height + dimensions.lane_width * i),
            (
                dimensions.window_width,
                dimensions.half_height + dimensions.lane_width * i,
            ),
            4.0,
            4.0,
        );
    }
}

fn draw_center_lines_to_texture(
    texture_canvas: &mut sdl2::render::Canvas<Window>,
    dimensions: &Dimensions,
) {
    texture_canvas
        .draw_line(
            (dimensions.half_width, 0),
            (dimensions.half_width, dimensions.window_height),
        )
        .unwrap();
    texture_canvas
        .draw_line(
            (0, dimensions.half_height),
            (dimensions.window_width, dimensions.half_height),
        )
        .unwrap();
}

fn draw_dashed_line_to_texture(
    texture_canvas: &mut sdl2::render::Canvas<Window>,
    start: (i32, i32),
    end: (i32, i32),
    dash_length: f32,
    gap_length: f32,
) {
    let (x1, y1) = (start.0 as f32, start.1 as f32);
    let (x2, y2) = (end.0 as f32, end.1 as f32);
    let dx = x2 - x1;
    let dy = y2 - y1;
    let line_length = (dx * dx + dy * dy).sqrt();

    if line_length == 0.0 {
        return;
    }

    let dir_x = dx / line_length;
    let dir_y = dy / line_length;

    let mut distance_traveled = 0.0;
    while distance_traveled < line_length {
        let dash_end_dist = distance_traveled + dash_length;
        let p1_x = x1 + dir_x * distance_traveled;
        let p1_y = y1 + dir_y * distance_traveled;

        let p2_x = x1 + dir_x * dash_end_dist.min(line_length);
        let p2_y = y1 + dir_y * dash_end_dist.min(line_length);

        texture_canvas
            .draw_line((p1_x as i32, p1_y as i32), (p2_x as i32, p2_y as i32))
            .unwrap();

        distance_traveled += dash_length + gap_length;
    }
}
