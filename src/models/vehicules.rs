// use rand::Rng;
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use sdl2::keyboard::Keycode;


pub enum VehiclePriority {
    High,
    Medium,
    Low
}

const VEHICLE_CREATION_COOLDOWN: Duration = Duration::from_millis(1500);
#[derive(Clone, PartialEq, Eq, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}
#[derive(Clone, PartialEq, Eq, Copy)]
pub enum Turn {
    Left,
    Right,
    Forward
}

pub struct Vehicule<'a> {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub direction: Direction,
    pub angle: f64,
    pub texture: Texture<'a>,
    pub turn : Turn,
    pub time : i64,
    pub velocity : i32,
    pub distance : i64
}


impl<'a> Vehicule<'a> {
    pub fn new(
        _sdl_context: &sdl2::Sdl,
        _canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        texture_creator: &'a TextureCreator<WindowContext>,
        direction: Direction,
    ) -> Result<Vehicule<'a>, String> {
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
        let texture = texture_creator.load_texture("./assets/vehicles.png")?;

        // let mut rng = rand::thread_rng();
        // let lane = rng.gen_range(1..=3);
        let lane = 2;

        let (x, y, angle) = match direction {
            Direction::North => match lane {
                1 => (425, 700, 0.0),
                2 => (350, 700, 0.0),
                3 => (390, 700, 0.0),
                _ => unreachable!(),
            },
            Direction::South => match lane {
                1 => (230, 0, 180.0),
                2 => (310, 0, 180.0),
                3 => (270, 0, 180.0),
                _ => unreachable!(),
            },
            Direction::East => match lane {
                1 => (0, 425, 90.0),
                2 => (0, 350, 90.0),
                3 => (0, 390, 90.0),
                _ => unreachable!(),
            },
            Direction::West => match lane {
                1 => (700, 230, 270.0),
                2 => (700, 310, 270.0),
                3 => (700, 270, 270.0),
                _ => unreachable!(),
            },
        };
        let turn = match  lane {
            1 => Turn::Right,
            2 => Turn::Left,
            3 => Turn::Forward,
            _ => unreachable!(),
            
        };

        Ok(Vehicule {
            x,
            y,
            width: 50,
            height: 50,
            direction,
            angle,
            texture,
            turn,
            time : 2,
            distance : 700,
            velocity : 5
        })
    }
    pub fn can_add_vehicle(
        last_key_press: &mut HashMap<Keycode, Instant>,
        keycode: Keycode,
    ) -> bool {
        let now = Instant::now();
        if let Some(&last_time) = last_key_press.get(&keycode) {
            if now.duration_since(last_time) < VEHICLE_CREATION_COOLDOWN {
                return false;
            }
        }
        last_key_press.insert(keycode, now);
        true
    }
    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        let target_rect = sdl2::rect::Rect::new(self.x, self.y, self.width, self.height);

        canvas.copy_ex(
            &self.texture,
            None,
            Some(target_rect),
            self.angle,
            None,
            false,
            false,
        )?;
        Ok(())
    }



    pub fn get_priority(&self, other_dir: Direction, other_turn: Turn) -> VehiclePriority {
        match (self.direction, self.turn, other_dir, other_turn) {
            // Rule 1: Vehicles going straight have priority over turning vehicles
            (_, Turn::Forward, _, turn) if turn != Turn::Forward => VehiclePriority::High,
            (_, turn, _, Turn::Forward) if turn != Turn::Forward => VehiclePriority::Low,

            // Rule 2: Right turns have priority over left turns
            (_, Turn::Right, _, Turn::Left) => VehiclePriority::High,
            (_, Turn::Left, _, Turn::Right) => VehiclePriority::Low,

            // Rule 3: Right-hand traffic rule (vehicle coming from the right has priority)
            (Direction::North, Turn::Forward, Direction::East, Turn::Forward) => VehiclePriority::Low,
            (Direction::East, Turn::Forward, Direction::South, Turn::Forward) => VehiclePriority::Low,
            (Direction::South, Turn::Forward, Direction::West, Turn::Forward) => VehiclePriority::Low,
            (Direction::West, Turn::Forward, Direction::North, Turn::Forward) => VehiclePriority::Low,

            // If directions are opposite or same, assign medium priority
            _ => VehiclePriority::Medium
        }
    }

    pub fn collision(&mut self, vehicle_data: &Vec<(i32, i32, Direction, Turn)>) {
        const COLLISION_BUFFER: i32 = 40;

        fn ranges_overlap(start1: i32, end1: i32, start2: i32, end2: i32) -> bool {
            start1 < end2 && end1 > start2
        }

        fn is_in_intersection_zone(x: i32, y: i32) -> bool {
            x >= 270 && x <= 390 && y >= 230 && y <= 415
        }

        // Calculate current vehicle's collision box
        let self_x_range = (self.x, self.x + self.width as i32);
        let self_y_range = (self.y, self.y + self.height as i32);

        // Check if we're approaching or in the intersection
        let is_near_intersection = match self.direction {
            Direction::North => self.y > 415 && self.y <= 415 + COLLISION_BUFFER,
            Direction::South => self.y < 230 && self.y >= 230 - COLLISION_BUFFER,
            Direction::East => self.x < 390 && self.x >= 390 - COLLISION_BUFFER,
            Direction::West => self.x > 270 && self.x <= 270 + COLLISION_BUFFER,
        };

        if is_near_intersection {
            // Get all vehicles that could potentially cause a conflict
            let potential_conflicts: Vec<_> = vehicle_data.iter()
                .filter(|&&(other_x, other_y, _, _)| {
                    is_in_intersection_zone(other_x, other_y) ||  // Vehicle in intersection
                        match self.direction {
                            Direction::North | Direction::South => {
                                ranges_overlap(270, 390, other_x, other_x + 50)  // Check East-West corridor
                            },
                            Direction::East | Direction::West => {
                                ranges_overlap(230, 415, other_y, other_y + 50)  // Check North-South corridor
                            }
                        }
                })
                .collect();

            // Check if we need to stop based on priority rules
            let should_stop = potential_conflicts.iter().any(|&&(other_x, other_y, other_dir, other_turn)| {

                // First check if there's a potential physical collision
                let physical_collision = match (self.direction, other_dir) {
                    (Direction::North, Direction::East)  => {
                        ranges_overlap(self_x_range.0, self_x_range.1, other_x, other_x + 50)
                    },
                    (Direction::East, Direction::North)  => {
                        ranges_overlap(self_x_range.0, self_x_range.1, other_x, other_x + 50)
                    },
                    (Direction::North, Direction::West)  => {
                        ranges_overlap(self_y_range.0, self_y_range.1, other_y, other_y + 50)
                    },
                    (Direction::West, Direction::North)  => {
                        ranges_overlap(self_y_range.0, self_y_range.1, other_y, other_y + 50)
                    },
                     (Direction::South, Direction::West) => {
                        ranges_overlap(self_x_range.0, self_x_range.1, other_x, other_x + 50)
                    },
                    // (Direction::West, Direction::South) => {
                    //     ranges_overlap(self_x_range.0, self_x_range.1, other_x, other_x + 50)
                    // },
                    (Direction::South, Direction::East) => {
                        ranges_overlap(self_x_range.0, self_x_range.1, other_x, other_x + 50)
                    },
                    (Direction::East, Direction::South) => {
                        ranges_overlap(self_x_range.0, self_x_range.1, other_x, other_x + 50)
                    },
                    _ => false
                };

                if physical_collision {
                    // Check priority rules
                    match self.get_priority(other_dir, other_turn) {
                        VehiclePriority::High => false,  // We have priority, don't stop
                        VehiclePriority::Medium => {
                            // For medium priority, stop only if other vehicle also has medium or high priority
                            matches!(self.get_priority(other_dir, other_turn),
                                VehiclePriority::Medium | VehiclePriority::High)
                        },
                        VehiclePriority::Low => true,  // We should yield
                    }
                } else {
                    false
                }
            });
            println!("should_stop:{}", should_stop);

            if should_stop {
                self.velocity = 0;
            } else {
                self.velocity = 5;
            }
        } else if self.velocity == 0 && !is_near_intersection {
            // Resume movement if we're clear of the intersection
            self.velocity = 5;
        }
    }
    pub fn update_position(&mut self, vehicle_data: &Vec<(i32, i32, Direction, Turn)>) {
        match self.direction {
            Direction::North => self.y -= self.velocity,
            Direction::South => self.y += self.velocity,
            Direction::East => self.x += self.velocity,
            Direction::West => self.x -= self.velocity,
        }
        self.collision(vehicle_data);
        let is_left = self.turn == Turn::Left;
        match self.direction {
            Direction::North  => {
                match  self.y {
                    425 => {
                        if !is_left {
                            self.execute_turn()
                        }
                    },
                    310 => {
                        if is_left {
                            self.execute_turn()
                        }
                    },
                    _=> return,
                }
            },

            Direction::South  => {
                match  self.y {
                    230 => {
                        if !is_left {
                            self.execute_turn()
                        }
                    },
                    350 => {
                        if is_left {
                            self.execute_turn()
                        }
                    },
                    _=> return,
                }
            },
            Direction::East => {
                match  self.x {
                    230 => {
                        if !is_left {
                            self.execute_turn()
                        }
                    },
                    350 => {
                        if is_left {
                            self.execute_turn()
                        }
                    },
                    _=> return,
                }
            },
            Direction::West => {
                match  self.x {
                    425 => {
                        if !is_left {
                            self.execute_turn()
                        }
                    },
                    310 => {
                        if is_left {
                            self.execute_turn()
                        }
                    },
                    _=> return,
                }
            },
        }

        
    }


    pub fn execute_turn(&mut self) {
        if self.turn != Turn::Forward {
            self.direction = match (self.direction.clone(), self.turn) {
                (Direction::North, Turn::Left) | (Direction::South, Turn::Right) => Direction::West,
                (Direction::North, Turn::Right) | (Direction::South, Turn::Left) => Direction::East,
                (Direction::East, Turn::Left) | (Direction::West, Turn::Right) => Direction::North,
                (Direction::East, Turn::Right) | (Direction::West, Turn::Left) => Direction::South,
                _ => self.direction.clone(),
            };
            self.angle = match self.direction.clone()  {
                Direction::West => 270.0,
                Direction::East => 90.0,
                Direction::North => 0.0,
                Direction::South => 180.0,
            };
            self.turn = Turn::Forward; // Reset turn after executing it
        }
    }
}
