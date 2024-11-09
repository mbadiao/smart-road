use rand::Rng;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub enum VehiclePriority {
    High,  
    Medium,
    Low,   
}

const VEHICLE_CREATION_COOLDOWN: Duration = Duration::from_millis(1500);
const SAFETY_DISTANCE: i32 = 100;
#[derive(Clone, PartialEq, Eq, Copy, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}
#[derive(Clone, PartialEq, Eq, Copy, Debug)]
pub enum Turn {
    Left,
    Right,
    Forward,
}

pub struct Vehicule<'a> {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub direction: Direction,
    pub angle: f64,
    pub texture: Texture<'a>,
    pub turn: Turn,
    pub time: i64,
    pub velocity: i32,
    pub distance: i64,
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

        let mut rng = rand::thread_rng();
        let lane = rng.gen_range(3..=3);

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
        let turn = match lane {
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
            time: 2,
            distance: 700,
            velocity: 5,
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
    pub fn render(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) -> Result<(), String> {
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
            (Direction::North ,  _, Direction::West, Turn::Forward | Turn::Left) => VehiclePriority::Medium,
             _ => VehiclePriority::Low,
        }
    }

    fn is_at_intersection_start(&self) -> bool {
        match self.direction {
            Direction::North => self.y <= 480 && self.y > 170, // && self.x >= intersection_start_x && self.x <= intersection_end_x,
            Direction::South => self.y >= 185 && self.y < 440, // && self.x >= intersection_start_x && self.x <= intersection_end_x,
            Direction::East => self.x >= 185 && self.x < 440, // && self.y >= intersection_start_y && self.y <= intersection_end_y,
            Direction::West => self.x <= 480 && self.x > 170, // && self.y >= intersection_start_y && self.y <= intersection_end_y,
        }
    }

    
    fn check_safety_distance(&self, vehicle_data: &Vec<(i32, i32, Direction, Turn)>) -> bool {
        for &(other_x, other_y, other_dir, _) in vehicle_data {
            if self.direction == other_dir {
                // Vérifie uniquement les véhicules dans la même direction
                match self.direction {
                    Direction::North => {
                        if self.x == other_x && // Même voie
                            self.y > other_y && // Véhicule devant
                            self.y - other_y < SAFETY_DISTANCE {
                            return true;
                        }
                    },
                    Direction::South => {
                        if self.x == other_x &&
                            self.y < other_y &&
                            other_y - self.y < SAFETY_DISTANCE {
                            return true;
                        }
                    },
                    Direction::East => {
                        if self.y == other_y &&
                            self.x < other_x &&
                            other_x - self.x < SAFETY_DISTANCE {
                            return true;
                        }
                    },
                    Direction::West => {
                        if self.y == other_y &&
                            self.x > other_x &&
                            self.x - other_x < SAFETY_DISTANCE {
                            return true;
                        }
                    },
                }
            }
        }
        false
    }   

    fn collision(&self, vehicle_data: &Vec<(i32, i32, Direction, Turn)>) -> bool {
        let mut any_collision = false; 
        for &(vx, vy, dir, turn) in vehicle_data.iter() {
            match self.direction {
                Direction::North => {
                    let  egale = false;
                    if dir == Direction::East && (turn == Turn::Forward || (turn == Turn::Left && self.turn == Turn::Left)) {
                        if self.x > vx && (self.y - vy).abs() > (self.x - vx).abs() && vy < self.y {
                            any_collision = true;
                            
                        }
                        // if self.y - self.x == self.x - vx {
                        //     egale = true;
                        // }
                    }
                    if dir == Direction::West && (turn == Turn::Forward || (turn == Turn::Left && self.turn == Turn::Left)) {
                        if self.x < vx && (self.y - vy).abs() > (vx - self.x).abs() && vy < self.y {
                            any_collision = true;
                        }
                        // if self.y - self.x == vx - self.x {
                        //     egale = true;
                        // }
                    }
                
                    if egale {
                        any_collision = true;
                    }
                }

                Direction::South => {
                    let  egale = false;
                    // println!("{} {} {} {:?}", self.y < vy, self.y , vy, self.direction);
                    if dir == Direction::East && (turn == Turn::Forward || (turn == Turn::Left && self.turn == Turn::Left)) {
                        if self.y < vy && (self.y - vy).abs() > (self.x - vx).abs() && (vx < self.x) {
                            any_collision = true;
                            
                        }
                        // if self.y - self.x == self.x - vx {
                        //     egale = true;
                        // }
                    }
                    if dir == Direction::West && (turn == Turn::Forward || (turn == Turn::Left && self.turn == Turn::Left)) {
                        if self.y < vy && (self.y - vy).abs() > (vx - self.x).abs() && (vx > self.x) {
                            any_collision = true;
                        }
                        // if self.y - self.x == vx - self.x {
                        //     egale = true;
                        // }
                    }
                
                    if egale {
                        any_collision = true;
                    }
                }

                Direction::East => {
                    let  egale = false;
                    if dir == Direction::North && (turn == Turn::Forward || (turn == Turn::Left && self.turn == Turn::Left)) {
                        // println!("{} {:?} {:?}", self.x > vx , self.direction, vehicle_data);
                        if self.x < vx && (vx - self.x).abs() > ( vy - self.y).abs() && vy > self.y {
                            any_collision = true;
                        }
                        // if self.y - self.x == self.x - vx {
                        //     egale = true;
                        // }
                    }
                    if dir == Direction::South && (turn == Turn::Forward || (turn == Turn::Left && self.turn == Turn::Left)) {
                        if self.x < vx && (vx - self.x).abs() > ( vy - self.y).abs() && vy < self.y  {
                            any_collision = true;
                        }
                        // if self.y - self.x == vx - self.x {
                        //     egale = true;
                        // }
                    }
                
                    if egale {
                        any_collision = true;
                    }
                }

                Direction::West => {
                    let  egale = false;
                    if dir == Direction::North && (turn == Turn::Forward || (turn == Turn::Left && self.turn == Turn::Left)) {
                        // println!("{} {:?} {:?}", self.x > vx , self.direction, vehicle_data);
                        if self.x > vx && (self.x - vx).abs() > ( vy - self.y).abs() && vy > self.y {
                            any_collision = true;
                        }
                        // if self.y - self.x == self.x - vx {
                        //     egale = true;
                        // }
                    }
                    if dir == Direction::South && (turn == Turn::Forward || (turn == Turn::Left && self.turn == Turn::Left)) {
                        if self.x > vx && (self.x - vx).abs() > (vy - self.y).abs() &&  vy < self.y  {
                            any_collision = true;
                        }
                        // if self.y - self.x == vx - self.x {
                        //     egale = true;
                        // }
                    }
                
                    if egale {
                        any_collision = true;
                    }
                }
                
               
                

               _=>  continue
            }
        }
        any_collision
    }
    

    pub fn update_position(&mut self, vehicle_data: &Vec<(i32, i32, Direction, Turn)>) {
        match self.direction {
            Direction::North => self.y -= self.velocity,
            Direction::South => self.y += self.velocity,
            Direction::East => self.x += self.velocity,
            Direction::West => self.x -= self.velocity,
        }

        if self.is_at_intersection_start() {
            if self.turn != Turn::Right {
                let  countinus  = vehicle_data.iter().any(|&x| 
                    match self.get_priority(x.2, x.3) {
                        VehiclePriority::High => false,
                        VehiclePriority::Low => true,
                        VehiclePriority::Medium => true,
                    }
                );
                println!("{} - {} - {:?}" ,countinus, self.collision(vehicle_data), vehicle_data);
                if  countinus && self.collision(vehicle_data) {
                    self.velocity = 0;
                } else {
                    self.velocity = 5
                }
            }
        }

        let is_left = self.turn == Turn::Left;
        match self.direction {
            Direction::North => match self.y {
                425 => {
                    if !is_left {
                        self.execute_turn()
                    }
                }
                310 => {
                    if is_left {
                        self.execute_turn()
                    }
                }
                _ => return,
            },

            Direction::South => match self.y {
                230 => {
                    if !is_left {
                        self.execute_turn()
                    }
                }
                350 => {
                    if is_left {
                        self.execute_turn()
                    }
                }
                _ => return,
            },
            Direction::East => match self.x {
                230 => {
                    if !is_left {
                        self.execute_turn()
                    }
                }
                350 => {
                    if is_left {
                        self.execute_turn()
                    }
                }
                _ => return,
            },
            Direction::West => match self.x {
                425 => {
                    if !is_left {
                        self.execute_turn()
                    }
                }
                310 => {
                    if is_left {
                        self.execute_turn()
                    }
                }
                _ => return,
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
            self.angle = match self.direction.clone() {
                Direction::West => 270.0,
                Direction::East => 90.0,
                Direction::North => 0.0,
                Direction::South => 180.0,
            };
            self.turn = Turn::Forward;
        }
    }
}
