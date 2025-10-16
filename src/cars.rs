use rand::Rng;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};

use crate::types::{Airt, Dimensions};

pub struct Traffic {
    pub cars: Vec<Car>,
    pub cars_passed: i32,
    pub give_ways: i32,
    pub max_time: Duration,
    pub min_time: Duration,
    pub max_velocity: i32,
    pub min_velocity: i32,
    pub close_calls: i32,
}

impl Traffic {
    pub fn new() -> Self {
        Traffic {
            cars: Vec::new(),
            cars_passed: 0,
            give_ways: 0,
            max_time: Duration::from_millis(0),
            min_time: Duration::MAX,
            max_velocity: 0,
            min_velocity: i32::MAX,
            close_calls: 0,
        }
    }

    pub fn draw(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        dimensions: &Dimensions,
        car_textures: &[sdl2::render::Texture; 4],
    ) {
        for car in &self.cars {
            car.draw(canvas, &dimensions, &car_textures);
        }
    }

    pub fn format(&self) -> String {
        if self.cars_passed == 0 {
            "=== SMART ROAD STATISTICS ===\n\n\
             Cars passed: 0\n\
             Give ways: 0\n\
             Close calls: 0\n\n\
             Velocity Stats:\n\
             • Max velocity: N/A\n\
             • Min velocity: N/A\n\n\
             Time Stats:\n\
             • Max time: N/A\n\
             • Min time: N/A\n\n\
             Status: No data collected yet".to_string()
        } else {
            let min_vel = if self.min_velocity == i32::MAX {
                "N/A".to_string()
            } else {
                format!("{}px/s", self.min_velocity)
            };
            
            format!(
                "=== SMART ROAD STATISTICS ===\n\n\
                 Traffic Summary:\n\
                 • Cars passed: {}\n\
                 • Give ways: {}\n\
                 • Close calls: {}\n\n\
                 Velocity Stats:\n\
                 • Max velocity: {}px/s\n\
                 • Min velocity: {}\n\n\
                 Time Stats:\n\
                 • Max time: {:.2}s\n\
                 • Min time: {:.2}s\n\n\
                 Safety Rating: {}",
                self.cars_passed,
                self.give_ways,
                self.close_calls,
                self.max_velocity,
                min_vel,
                self.max_time.as_secs_f64(),
                self.min_time.as_secs_f64(),
                if self.close_calls == 0 {
                    "✓ EXCELLENT (No close calls)"
                } else if self.close_calls < 5 {
                    "⚠ GOOD (Few close calls)"
                } else {
                    "✗ NEEDS IMPROVEMENT (Many close calls)"
                }
            )
        }
    }

    pub fn push(&mut self, initial_direction: Airt, dimensions: &Dimensions) {
        self.cars
            .push(Car::spawn(initial_direction, self.cars.len(), dimensions));
    }

    pub fn push_random(&mut self, dimensions: &Dimensions) {
        let directions = [Airt::Up, Airt::Down, Airt::Left, Airt::Right];
        let random_direction = directions[rand::thread_rng().gen_range(0..directions.len())];
        self.cars
            .push(Car::spawn(random_direction, self.cars.len(), &dimensions));
    }

    pub fn update(&mut self, dimensions: &Dimensions) {
        for (i, car) in self.cars.iter().enumerate() {
            debug_assert!(
                car.index == i,
                "Mismatch: car.index {} does not match position {}",
                car.index,
                i
            );
        }

        let mut prospective_positions = self
            .cars
            .iter()
            .map(|car| (car.x, car.y, car.index)) // The `index` is used to ignore "collisions" of a car with itself.
            .collect::<Vec<(i32, i32, usize)>>();

        // Check for close calls before updating positions
        self.check_close_calls(dimensions);

        for car in self.cars.iter_mut() {
            // Track velocity stats
            if car.speed > self.max_velocity {
                self.max_velocity = car.speed;
            }
            if car.speed < self.min_velocity {
                self.min_velocity = car.speed;
            }

            if !car.update(
                &mut prospective_positions,
                &mut self.cars_passed,
                &mut self.max_time,
                &mut self.min_time,
                &dimensions,
            ) {
                self.give_ways += 1;
            }
        }

        self.cars.retain(|car| !car.gone);

        for (index, car) in self.cars.iter_mut().enumerate() {
            car.index = index;
        }
    }

    fn check_close_calls(&mut self, dimensions: &Dimensions) {
        let safety_distance = (dimensions.lane_width as f32 * 1.5) as i32;
        for i in 0..self.cars.len() {
            for j in (i + 1)..self.cars.len() {
                let car1 = &self.cars[i];
                let car2 = &self.cars[j];
                
                let dx = car1.x - car2.x;
                let dy = car1.y - car2.y;
                let distance = ((dx * dx + dy * dy) as f32).sqrt() as i32;
                
                if distance < safety_distance && distance > dimensions.lane_width {
                    self.close_calls += 1;
                }
            }
        }
    }
}

pub struct Car {
    x: i32,
    y: i32,
    color_code: usize,
    direction: Direction,
    speed: i32,
    target_speed: i32,  // For smooth acceleration/deceleration
    current_angle: f64,  // Current rotation angle for smooth turning
    target_angle: f64,   // Target rotation angle
    vertical: bool,
    gone: bool,
    index: usize,
    birthday: Instant,
    in_turn: bool,  // Track if currently turning
}

struct Direction {
    start: Airt,
    end: Airt,
}

impl Car {
    pub fn spawn(initial_direction: Airt, index: usize, dimensions: &Dimensions) -> Self {
        let r = rand::thread_rng().gen_range(0..3);

        let (x, y, final_direction, color_code, speed, vertical) = match initial_direction {
            Airt::Up => {
                let y = dimensions.window_height - dimensions.lane_width;
                let (x, final_direction, speed) = match r {
                    0 => (
                        dimensions.half_width,
                        Airt::Left,
                        dimensions.speed.default,
                    ),
                    1 => (
                        dimensions.half_width + dimensions.lane_width,
                        Airt::Up,
                        dimensions.speed.fast,
                    ),
                    _ => (
                        dimensions.half_width + 2 * dimensions.lane_width,
                        Airt::Right,
                        dimensions.speed.slow,
                    ),
                };
                (x, y, final_direction, 0, speed, true)
            }
            Airt::Down => {
                let y = 0;
                let (x, final_direction, speed) = match r {
                    0 => (
                        dimensions.half_width - 3 * dimensions.lane_width,
                        Airt::Left,
                        dimensions.speed.slow,
                    ),
                    1 => (
                        dimensions.half_width - 2 * dimensions.lane_width,
                        Airt::Down,
                        dimensions.speed.fast,
                    ),
                    _ => (
                        dimensions.half_width - dimensions.lane_width,
                        Airt::Right,
                        dimensions.speed.default,
                    ),
                };
                (x, y, final_direction, 1, speed, true)
            }
            Airt::Right => {
                let x = 0;
                let (y, final_direction, speed) = match r {
                    0 => (dimensions.half_height, Airt::Up, dimensions.speed.default),
                    1 => (
                        dimensions.half_height + dimensions.lane_width,
                        Airt::Right,
                        dimensions.speed.fast,
                    ),
                    _ => (
                        dimensions.half_height + 2 * dimensions.lane_width,
                        Airt::Down,
                        dimensions.speed.slow,
                    ),
                };
                (x, y, final_direction, 2, speed, false)
            }
            Airt::Left => {
                let x = dimensions.window_width - dimensions.lane_width;
                let (y, final_direction, speed) = match r {
                    0 => (
                        dimensions.half_height - 3 * dimensions.lane_width,
                        Airt::Up,
                        dimensions.speed.slow,
                    ),
                    1 => (
                        dimensions.half_height - 2 * dimensions.lane_width,
                        Airt::Left,
                        dimensions.speed.fast,
                    ),
                    _ => (
                        dimensions.half_height - dimensions.lane_width,
                        Airt::Down,
                        dimensions.speed.default,
                    ),
                };
                (x, y, final_direction, 3, speed, false)
            }
        };

        let initial_angle = match initial_direction {
            Airt::Up => 0.0,
            Airt::Down => 180.0,
            Airt::Left => -90.0,
            Airt::Right => 90.0,
        };

        Car {
            x,
            y,
            color_code,
            direction: Direction {
                start: initial_direction,
                end: final_direction,
            },
            speed,
            target_speed: speed,
            current_angle: initial_angle,
            target_angle: initial_angle,
            vertical,
            gone: false,
            index,
            birthday: Instant::now(),
            in_turn: false,
        }
    }

    fn will_collide(
        &self,
        new_x: i32,
        new_y: i32,
        prospective_positions: &Vec<(i32, i32, usize)>,
        dimensions: &Dimensions,
    ) -> bool {
        for other in prospective_positions {
            if other.2 == self.index {
                continue;
            }
            if new_x < other.0 + dimensions.lane_width
                && new_x + dimensions.lane_width > other.0
                && new_y < other.1 + dimensions.lane_width
                && new_y + dimensions.lane_width > other.1
            {
                return true;
            }
        }
        false
    }

    fn update(
        &mut self,
        prospective_positions: &mut Vec<(i32, i32, usize)>,
        cars_passed: &mut i32,
        max_time: &mut Duration,
        min_time: &mut Duration,
        dimensions: &Dimensions,
    ) -> bool {
        if self.x < 0
            || self.x + dimensions.lane_width > dimensions.window_width
            || self.y < 0
            || self.y + dimensions.lane_width > dimensions.window_height
        {
            *cars_passed += 1;
            self.gone = true;
            let elapsed = Instant::now().duration_since(self.birthday);
            if *max_time < elapsed {
                *max_time = elapsed;
            }
            if *min_time > elapsed {
                *min_time = elapsed;
            }
            return true;
        }

        let (new_x, new_y) = self.calculate_new_position(dimensions);

        if self.will_collide(new_x, new_y, prospective_positions, dimensions) {
            return false;
        }

        prospective_positions[self.index] = (new_x, new_y, self.index);

        self.x = new_x;
        self.y = new_y;

        return true;
    }

    fn get_current_direction(&self) -> Airt {
        let start_is_vertical = matches!(self.direction.start, Airt::Up | Airt::Down);

        if self.vertical == start_is_vertical {
            // If the car's orientation matches its starting orientation, it hasn't turned yet.
            self.direction.start
        } else {
            // The car has turned.
            self.direction.end
        }
    }

    fn draw(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        dimensions: &Dimensions,
        car_textures: &[sdl2::render::Texture; 4],
    ) {
        if self.x < 0
            || self.x + dimensions.lane_width > dimensions.window_width
            || self.y < 0
            || self.y + dimensions.lane_width > dimensions.window_height
        {
            return;
        }

        let x = self.x as i32;
        let y = self.y as i32;
        let lane_width = dimensions.lane_width as u32;

        // Draw speed trail effect behind the car (visual feedback for velocity)
        let speed_ratio = self.speed as f32 / dimensions.speed.fast as f32;
        if speed_ratio > 0.3 {
            // Calculate trail direction (opposite of movement)
            let trail_length = (speed_ratio * 12.0) as i32;
            let trail_segments = 3;
            
            for i in 0..trail_segments {
                let alpha = (100.0 * (1.0 - i as f32 / trail_segments as f32) * speed_ratio) as u8;
                let offset = trail_length * (i + 1) / trail_segments;
                
                let (trail_x, trail_y) = match self.get_current_direction() {
                    Airt::Up => (x, y + offset),
                    Airt::Down => (x, y - offset),
                    Airt::Left => (x + offset, y),
                    Airt::Right => (x - offset, y),
                };
                
                // Speed trail color based on car color but semi-transparent
                let trail_color = match self.color_code {
                    0 => sdl2::pixels::Color::RGBA(255, 100, 100, alpha),
                    1 => sdl2::pixels::Color::RGBA(100, 255, 100, alpha),
                    2 => sdl2::pixels::Color::RGBA(100, 100, 255, alpha),
                    _ => sdl2::pixels::Color::RGBA(255, 255, 100, alpha),
                };
                
                canvas.set_draw_color(trail_color);
                let trail_size = (lane_width as f32 * 0.7 * (1.0 - i as f32 / trail_segments as f32)) as u32;
                canvas
                    .fill_rect(Rect::new(
                        trail_x + (lane_width as i32 - trail_size as i32) / 2,
                        trail_y + (lane_width as i32 - trail_size as i32) / 2,
                        trail_size,
                        trail_size,
                    ))
                    .ok();
            }
        }

        // Use smooth interpolated angle for better turning animation
        let angle = self.current_angle;

        let center = sdl2::rect::Point::new(lane_width as i32 / 2, lane_width as i32 / 2);

        let car_texture = &car_textures[self.color_code];

        canvas
            .copy_ex(
                car_texture,
                None, // No cropping (draw the whole texture).
                Some(Rect::new(x, y, lane_width, lane_width)),
                angle,
                Some(center),
                false,
                false,
            )
            .expect("Failed to draw car with rotation");
            
        // Draw speed indicator glow around fast cars
        if speed_ratio > 0.8 {
            let glow_color = sdl2::pixels::Color::RGBA(255, 255, 100, 40);
            canvas.set_draw_color(glow_color);
            let glow_size = lane_width + 4;
            canvas
                .draw_rect(Rect::new(x - 2, y - 2, glow_size, glow_size))
                .ok();
        }
    }

    fn calculate_new_position(&mut self, dimensions: &Dimensions) -> (i32, i32) {
        // Smooth speed transitions (acceleration/deceleration)
        let acceleration = 1; // Pixels per frame acceleration
        if self.speed < self.target_speed {
            self.speed = (self.speed + acceleration).min(self.target_speed);
        } else if self.speed > self.target_speed {
            self.speed = (self.speed - acceleration).max(self.target_speed);
        }

        // Smooth rotation interpolation
        let rotation_speed = 8.0; // Degrees per frame
        let angle_diff = self.target_angle - self.current_angle;
        
        // Handle angle wrapping (shortest path)
        let normalized_diff = if angle_diff > 180.0 {
            angle_diff - 360.0
        } else if angle_diff < -180.0 {
            angle_diff + 360.0
        } else {
            angle_diff
        };
        
        if normalized_diff.abs() > rotation_speed {
            self.current_angle += normalized_diff.signum() * rotation_speed;
        } else {
            self.current_angle = self.target_angle;
        }
        
        // Normalize angle to 0-360 range
        while self.current_angle < 0.0 {
            self.current_angle += 360.0;
        }
        while self.current_angle >= 360.0 {
            self.current_angle -= 360.0;
        }

        let mut new_x = self.x;
        let mut new_y = self.y;

        match self.direction.start {
            Airt::Up => match self.direction.end {
                Airt::Left => {
                    if self.y > dimensions.half_height - dimensions.lane_width {
                        new_y = self.y - self.speed;
                    } else {
                        new_y = dimensions.half_height - dimensions.lane_width;
                        self.vertical = false;
                        new_x = self.x - self.speed;
                        if !self.in_turn {
                            self.in_turn = true;
                            self.target_angle = -90.0;
                        }
                    }
                }
                Airt::Up => {
                    new_y = self.y - self.speed;
                }
                Airt::Right => {
                    if self.y > dimensions.half_height + 2 * dimensions.lane_width {
                        new_y = self.y - self.speed;
                    } else {
                        new_y = dimensions.half_height + 2 * dimensions.lane_width;
                        self.vertical = false;
                        new_x = self.x + self.speed;
                        if !self.in_turn {
                            self.in_turn = true;
                            self.target_angle = 90.0;
                        }
                    }
                }
                _ => panic!("Invalid turn"),
            },
            Airt::Down => match self.direction.end {
                Airt::Left => {
                    if self.y < dimensions.half_height - 3 * dimensions.lane_width {
                        new_y = self.y + self.speed;
                    } else {
                        new_y = dimensions.half_height - 3 * dimensions.lane_width;
                        new_x = self.x - self.speed;
                        self.vertical = false;
                        if !self.in_turn {
                            self.in_turn = true;
                            self.target_angle = -90.0;
                        }
                    }
                }
                Airt::Down => {
                    new_y = self.y + self.speed;
                }
                Airt::Right => {
                    if self.y < dimensions.half_height {
                        new_y = self.y + self.speed;
                    } else {
                        new_y = dimensions.half_height;
                        new_x = self.x + self.speed;
                        self.vertical = false;
                        if !self.in_turn {
                            self.in_turn = true;
                            self.target_angle = 90.0;
                        }
                    }
                }
                _ => panic!("Invalid turn"),
            },
            Airt::Left => match self.direction.end {
                Airt::Up => {
                    if self.x > dimensions.half_width + 2 * dimensions.lane_width {
                        new_x = self.x - self.speed;
                    } else {
                        new_x = dimensions.half_width + 2 * dimensions.lane_width;
                        self.vertical = true;
                        new_y = self.y - self.speed;
                        if !self.in_turn {
                            self.in_turn = true;
                            self.target_angle = 0.0;
                        }
                    }
                }
                Airt::Left => {
                    new_x = self.x - self.speed;
                }
                Airt::Down => {
                    if self.x > dimensions.half_width - dimensions.lane_width {
                        new_x = self.x - self.speed;
                    } else {
                        new_x = dimensions.half_width - dimensions.lane_width;
                        self.vertical = true;
                        new_y = self.y + self.speed;
                        if !self.in_turn {
                            self.in_turn = true;
                            self.target_angle = 180.0;
                        }
                    }
                }
                _ => panic!("Invalid turn"),
            },
            Airt::Right => match self.direction.end {
                Airt::Up => {
                    if self.x < dimensions.half_width {
                        new_x = self.x + self.speed;
                    } else {
                        new_x = dimensions.half_width;
                        self.vertical = true;
                        new_y = self.y - self.speed;
                        if !self.in_turn {
                            self.in_turn = true;
                            self.target_angle = 0.0;
                        }
                    }
                }
                Airt::Right => {
                    new_x = self.x + self.speed;
                }
                Airt::Down => {
                    if self.x < dimensions.half_width - 3 * dimensions.lane_width {
                        new_x = self.x + self.speed;
                    } else {
                        new_x = dimensions.half_width - 3 * dimensions.lane_width;
                        self.vertical = true;
                        new_y = self.y + self.speed;
                        if !self.in_turn {
                            self.in_turn = true;
                            self.target_angle = 180.0;
                        }
                    }
                }
                _ => panic!("Invalid turn"),
            },
        }

        (new_x, new_y)
    }
}
