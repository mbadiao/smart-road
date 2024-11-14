use smart_road::models::{path, vehicules::Vehicule, vehicules::Direction, statistics::Statistics};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;
use smart_road::models::vehicules::Turn;
use rand::Rng;

const WIDTH: u32 = 700;
const HEIGHT: u32 = 700;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();

    let mut vehicles = Vec::new();
    let mut last_key_press = HashMap::new();
    let mut should_quit = false;
    let mut statistics = Statistics::new();
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        if should_quit {
            break 'running;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        path::Path::new_path(&mut canvas);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    should_quit = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    if statistics.show_statistics {
                        should_quit = true;
                    } else {
                        statistics.toggle_statistics_display(&mut canvas);
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    let mut vehicle = Vehicule::new(
                        &sdl_context,
                        &mut canvas,
                        &texture_creator,
                        Direction::North,
                        &vehicles
                    )?;
                    if vehicle.can_add_vehicle(&mut last_key_press, Keycode::Up, &vehicles) {
                        vehicles.push(vehicle);
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    let mut vehicle = Vehicule::new(
                        &sdl_context,
                        &mut canvas,
                        &texture_creator,
                        Direction::South,
                        &vehicles
                    )?;
                    if vehicle.can_add_vehicle(&mut last_key_press, Keycode::Down, &vehicles) {
                        vehicles.push(vehicle);
                    }
                    
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    let mut vehicle = Vehicule::new(
                        &sdl_context,
                        &mut canvas,
                        &texture_creator,
                        Direction::West,
                        &vehicles
                    )?;
                    if vehicle.can_add_vehicle(&mut last_key_press, Keycode::Left, &vehicles) {
                        vehicles.push(vehicle);
                    }
                }
                
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    let mut vehicle = Vehicule::new(
                        &sdl_context,
                        &mut canvas,
                        &texture_creator,
                        Direction::East,
                        &vehicles
                    )?;
                    if vehicle.can_add_vehicle(&mut last_key_press, Keycode::Right, &vehicles) {
                        vehicles.push(vehicle);
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    let mut vehicle = Vehicule::new(
                        &sdl_context,
                        &mut canvas,
                        &texture_creator,
                        Direction::East,
                        &vehicles
                    )?;
                    if vehicle.can_add_vehicle(&mut last_key_press, Keycode::R, &vehicles) {
                        let num_vehicles = rand::thread_rng().gen_range(1..=3); 
            
                        for _ in 0..num_vehicles {
                            let random_direction = Vehicule::get_random_direction();
                            
                            if let Ok(vehicle) = Vehicule::new(
                                &sdl_context,
                                &mut canvas,
                                &texture_creator,
                                random_direction,
                                &vehicles
                            ) {
                                vehicles.push(vehicle);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        let vehicle_positions: Vec<(i32, i32, Direction, Turn , bool)> = vehicles
            .iter()
            .map(|v| (v.x, v.y, v.direction, v.turn, v.passed))
            .collect();

        for i in 0..vehicles.len() {
            vehicles[i].update(&vehicle_positions,&mut statistics);
            vehicles[i].render(&mut canvas)?;
        }
        statistics.increment(&vehicles);
        statistics.max_velocity(&vehicles);
        statistics.min_velocity(&vehicles);
        if statistics.show_statistics {
            statistics.display(&mut canvas);
        }
        canvas.present();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}