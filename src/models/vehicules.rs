// use rand::Rng;
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use sdl2::keyboard::Keycode;

const VEHICLE_CREATION_COOLDOWN: Duration = Duration::from_millis(2000);
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
        let lane = 3;

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

    pub fn collision(&mut self, vehicle_data: &Vec<(i32, i32, Direction, Turn)>) {
        match self.direction {
            Direction::North => {
                let colliding = vehicle_data
                    .iter()
                    .any(|&(x, _, dir, turn)|
                        dir == Direction::East &&
                            turn == Turn::Forward &&
                            x - 390 < 50
                    );

                if self.y + 50 > 390 && colliding {
                    self.velocity = 0;
                }
            }
            Direction::South => {
                // Implement logic for South
            }
            Direction::East => {
                // Implement logic for East
            }
            Direction::West => {
                // Implement logic for West
            }
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
