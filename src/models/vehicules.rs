
use rand::Rng;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::models::statistics::Statistics;

pub enum VehiclePriority {
    High,
    Medium,
    Low,
}

const VEHICLE_CREATION_COOLDOWN: Duration = Duration::from_millis(1500);
const SAFETY_DISTANCE: i32 = 60;
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
    pub time: i32,
    pub velocity: i32,
    pub distance: i32,
    pub passed : bool,
    pub passed_inter: bool
}

impl<'a> Vehicule<'a> {
    pub fn new(
        _sdl_context: &sdl2::Sdl,
        _canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        texture_creator: &'a TextureCreator<WindowContext>,
        direction: Direction,
        vehicule_data: &[Vehicule],
    ) -> Result<Vehicule<'a>, String> {
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
        let texture = texture_creator.load_texture("./assets/vehicles.png")?;

        let mut rng = rand::thread_rng();
        let mut lane = rng.gen_range(1..=3);

        // println!("{}",is_in_bounds(vehicule_data));
        // println!("{:?}",vehicule_data);
        if is_in_bounds(vehicule_data) {
            lane = 1;
        }
        // println!("{}", lane);

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

        pub fn is_in_bounds(vehicle_data: &[Vehicule]) -> bool {
            for vehicule in vehicle_data {
                if vehicule.x > 230 && vehicule.x < 425 && vehicule.y > 230 && vehicule.y < 425 {
                    return true
                }
            }
            false
        }
       
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
            time: 140,
            distance: 700,
            velocity: 5,
            passed : false,
            passed_inter: false,
        })
    }
    pub fn can_add_vehicle(
        &mut self,
        last_key_press: &mut HashMap<Keycode, Instant>,
        keycode: Keycode,
        vehicles: &[Vehicule],
    ) -> bool {
        let now = Instant::now();
        
        if let Some(&last_time) = last_key_press.get(&keycode) {
            if now.duration_since(last_time) < VEHICLE_CREATION_COOLDOWN {
                return false;
            }
        }
        if self.check_safety_distance_car(vehicles) {
            return false
        }
    
        // let new_vehicle_position = self.get_position_for_new_vehicle(self.direction); // assuming west direction
        for vehicle in vehicles {
            // let vehicle_position = vehicle.get_position();
            if vehicle.x > self.x - 40 && vehicle.x <= self.x {
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
            (Direction::North, _, Direction::West, Turn::Forward | Turn::Left) => {
                VehiclePriority::Low
            }
            (Direction::West, _, Direction::South, Turn::Forward | Turn::Left) => {
                VehiclePriority::Low
            }
            (Direction::South, _, Direction::East, Turn::Forward | Turn::Left) => {
                VehiclePriority::Low
            }
            (Direction::East, _, Direction::North, Turn::Forward | Turn::Left) => {
                VehiclePriority::Low
            }
            _ => VehiclePriority::High,
        }
    }

    fn is_at_intersection_start(&self) -> bool {
        match self.direction {
            Direction::North => self.y <= 480 && self.y > 170,
            Direction::South => self.y >= 185 && self.y < 440,
            Direction::East => self.x >= 185 && self.x < 440,
            Direction::West => self.x <= 480 && self.x > 170,
        }
    }
    
    fn if_pass_intersection(&self) -> bool {

        match self.direction {
            Direction::North => self.y <= 230,
            Direction::South => self.y >= 470,
            Direction::East => self.x >= 470,
            Direction::West => self.x <= 230, 
        }
    }

    pub fn check_safety_distance(&self, vehicle_data: &Vec<(i32, i32, Direction, Turn, bool)>) -> bool {
        for &(other_x, other_y, other_dir, _, _) in vehicle_data {
            if self.direction == other_dir {
                match self.direction {
                    Direction::North => {
                        if self.x == other_x
                            && self.y > other_y
                            && self.y - other_y < SAFETY_DISTANCE
                        {
                            return true;
                        }
                    }
                    Direction::South => {
                        if self.x == other_x
                            && self.y < other_y
                            && other_y - self.y < SAFETY_DISTANCE
                        {
                            return true;
                        }
                    }
                    Direction::East => {
                        if self.y == other_y
                            && self.x < other_x
                            && other_x - self.x < SAFETY_DISTANCE
                        {
                            return true;
                        }
                    }
                    Direction::West => {
                        if self.y == other_y
                            && self.x > other_x
                            && self.x - other_x < SAFETY_DISTANCE
                        {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn check_safety_distance_car(&self, vehicle_data: &[Vehicule]) -> bool {
        for other_vehicle in vehicle_data {
            if self.direction == other_vehicle.direction {
                match self.direction {
                    Direction::North => {
                        if self.x == other_vehicle.x
                            && self.y > other_vehicle.y
                            && self.y - other_vehicle.y < 80
                        {
                            return true;
                        }
                    }
                    Direction::South => {
                        if self.x == other_vehicle.x
                            && self.y < other_vehicle.y
                            && other_vehicle.y - self.y < 80
                        {
                            return true;
                        }
                    }
                    Direction::East => {
                        if self.y == other_vehicle.y
                            && self.x < other_vehicle.x
                            && other_vehicle.x - self.x < 80
                        {
                            return true;
                        }
                    }
                    Direction::West => {
                        if self.y == other_vehicle.y
                            && self.x > other_vehicle.x
                            && self.x - other_vehicle.x < 80
                        {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
    
    fn collision(
        &self,
        vehicle_data: &Vec<(i32, i32, Direction, Turn, bool)>,
    ) -> (bool, (Direction, (Direction, i32, i32)), (i32, i32)) {
        let mut direction_other = (self.direction, self.x, self.y);
        let mut any_collision = false;
        let mut distance = (0, 0);
        for &(vx, vy, dir, turn, _) in vehicle_data.iter() {
            match self.direction {
                Direction::North => {
                    let egale = false;
                    if dir == Direction::East && (turn == Turn::Forward || (turn == Turn::Left && self.turn == Turn::Left))
                    {
                        direction_other = (Direction::East, vx, vy);
                        distance = ((self.y - vy).abs(), (self.x - vx).abs());
                        if self.x >= vx - 70 && (self.y - vy).abs() >= (self.x - vx ).abs() && vy <= self.y + 70
                        {
                            any_collision = true;
                        }
                    }
                    if dir == Direction::West && (turn == Turn::Forward || (turn == Turn::Left && self.turn == Turn::Left))
                    {
                        direction_other = (Direction::West, vx, vy);

                        distance = ((self.y - vy).abs(), (vx - self.x).abs());
                        if self.x <= vx + 140  && (self.y - vy).abs() >= (vx - self.x).abs() && vy <= self.y + 140
                        {
                            any_collision = true;
                        }
                    }

                    if egale {
                        any_collision = true;
                    }
                }

                Direction::South => {
                    let egale = false;
                    if dir == Direction::East && (turn == Turn::Forward || (turn == Turn::Left && self.turn == Turn::Left))
                    {
                        direction_other = (Direction::East, vx, vy);

                        distance = ((self.y - vy).abs(), (self.x - vx).abs());
                        if self.y <= vy  && (self.y - vy).abs() >= (self.x - vx).abs() && (vx - 70 <= self.x )
                        {
                            any_collision = true;
                        }
                    }
                    if dir == Direction::West && (turn == Turn::Forward || (turn == Turn::Left && self.turn == Turn::Left))
                    {
                        direction_other = (Direction::West, vx, vy);

                        distance = ((self.y - vy).abs(), (vx - self.x).abs());
                        if self.y <= vy && (self.y - vy).abs() >= (vx - self.x).abs()  && (vx + 70 >= self.x)
                        {
                            any_collision = true;
                        }
                    }

                    if egale {
                        any_collision = true;
                    }
                }

                Direction::East => {
                    let egale = false;
                    if dir == Direction::North && (turn == Turn::Forward || (turn == Turn::Left && self.turn == Turn::Left))
                    {
                        direction_other = (Direction::North, vx, vy);
                        distance = ((vx - self.x).abs(), (vy - self.y).abs());
                        if self.x <= vx  && (vx - self.x).abs() >= (vy - self.y).abs() && vy + 50 >= self.y
                        {
                            any_collision = true;
                        }
                    }
                    if dir == Direction::South && (turn == Turn::Forward || (turn == Turn::Left && self.turn == Turn::Left))
                    {
                        direction_other = (Direction::South, vx, vy);

                        distance = ((vx - self.x).abs(), (vy - self.y).abs());
                        if self.x <= vx && (vx - self.x).abs() >= (vy - self.y).abs() && vy - 70 <= self.y
                        {
                            any_collision = true;
                        }
                    }

                    if egale {
                        any_collision = true;
                    }
                }

                Direction::West => {
                    let egale = false;
                    if dir == Direction::North && (turn == Turn::Forward || (turn == Turn::Left && self.turn == Turn::Left))
                    {
                        direction_other = (Direction::North, vx, vy);
                        distance = ((self.x - vx).abs(), (vy - self.y).abs());
                        if self.x >= vx && (self.x - vx).abs() >= (vy - self.y).abs() && vy + 70 >= self.y
                        {
                            any_collision = true;
                        }
                    }
                    if dir == Direction::South && (turn == Turn::Forward  || (turn == Turn::Left && self.turn == Turn::Left))
                    {
                        direction_other = (Direction::South, vx, vy);

                        distance = ((self.x - vx).abs(), (vy - self.y).abs());
                        if self.x >= vx && (self.x - vx).abs() >= (vy - self.y).abs() && vy - 70 <= self.y
                        {
                            any_collision = true;
                        }
                    }

                    if egale {
                        any_collision = true;
                    }
                }

                _ => continue,
            }
        }
        (any_collision, (self.direction, direction_other), distance)
    }



    pub fn get_random_direction() -> Direction {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::East,
            _ => Direction::West,
        }
    }

    
   

    
    pub fn update(&mut self, vehicle_data: &Vec<(i32, i32, Direction, Turn, bool)>,stat: &mut Statistics) {
        if self.is_at_intersection_start() {
            if self.collision(vehicle_data).0 {
                match self.turn {
                    Turn::Left => self.time = 1000,
                    Turn::Right => self.time = 140,
                    Turn::Forward => match self.direction {
                        Direction::East | Direction::West => {
                            if vehicle_data.iter().any(|&(_vx, _vy, dir, _turn, has_turned)| {
                                (dir == Direction::South || dir == Direction::North) && !has_turned
                            }) {
                                self.time = 1000;
                            }
                        }
                        _ => self.time = 35,
                    },
                }
            } else {
                match self.turn {
                    Turn::Left => self.time = 1000,  // Voie 2
                    Turn::Right => self.time = 140,  // Voie 1
                    Turn::Forward => self.time = 15, // Voie 3
                }
            }

          
            if self.velocity == 0
                && vehicle_data.iter().all(|&(vx, vy, _dir, turn, _has_turned)| {
                    turn != Turn::Forward || (vx < 0 || vx > 700 || vy < 0 || vy > 700)
                })

            {
                let is_left = self.turn == Turn::Left;

                match self.direction {

                    Direction::North => {
                        if vehicle_data.iter().any(|&(_vx, _vy, dir, turn, has_turned)| {
                            (dir == Direction::South) && !has_turned && turn != Turn::Forward
                        }) {
                            self.time = 1000;
                        } else {
                            self.time = 35;
                        }
                        match self.y {
                            290..=310 => {
                                if is_left {
                                    self.execute_turn();
                                }
                            }
                            _ => (),
                        }
                    }
                    Direction::South => {
                        self.time = 35;
                        match self.y {
                            340..=350 => {
                                if is_left {
                                    self.execute_turn();
                                }
                            }
                            _ => (),
                        }
                    }
                    Direction::East => {

                        if vehicle_data.iter().any(|&(_vx, _vy, dir, turn, has_turned)| {
                            (dir == Direction::West || dir == Direction::South) && !has_turned && turn != Turn::Forward
                        }) {
                            self.time = 1000;
                        } else {
                            self.time = 35;
                        }
                        match self.x {
                            340..=350  => {
                                if is_left {
                                    self.execute_turn();
                                }
                            }
                            _ => (),
                        }
                    }
                    Direction::West => {

                        if vehicle_data.iter().any(|&(_vx, _vy, dir, turn, has_turned)| {
                            (dir == Direction::North) && !has_turned && turn != Turn::Forward
                        }) {
                            self.time = 1000;
                        } else {
                            self.time = 35;
                        }
                        match self.x {
                            290..=310 => {
                                if is_left {
                                    self.execute_turn();
                                }
                            }
                            _ => (),
                        }
                    }
                }
            } else {
                if self.turn == Turn::Right {
                let is_left = self.turn == Turn::Left;
                    match self.direction {
                        Direction::North => {
                            self.time = 140;
                            match self.y {
                                425 => {
                                    if !is_left {
                                        self.execute_turn()
                                    }
                                }
                                _ => (),
                            }
                        }
                        Direction::South => {
                            self.time = 140;
                            match self.y {
                                230 => {
                                    if !is_left {
                                        self.execute_turn()
                                    }
                                }
                                _ => (),
                            }
                        }
                        Direction::East => {
                            self.time = 140;
                            match self.x {
                                230 => {
                                    if !is_left {
                                        self.execute_turn()
                                    }
                                }
                                _ => (),
                            }
                        }
                        Direction::West => {
                            self.time = 140;
                            match self.x {
                                425 => {
                                    if !is_left {
                                        self.execute_turn()
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }
        }
        
        self.velocity = (self.distance / self.time) as i32;
        if self.check_safety_distance(vehicle_data) {
            self.velocity = 0
        }
        match self.direction {
            Direction::North => self.y -= self.velocity,
            Direction::South => self.y += self.velocity,
            Direction::East => self.x += self.velocity,
            Direction::West => self.x -= self.velocity,
        }

        if self.if_pass_intersection() {
            self.passed_inter = true;
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
        self.passed = true
    }
}
