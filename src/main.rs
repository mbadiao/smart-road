use smart_road::models::{path, vehicules::Vehicule, vehicules::Direction};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;
use smart_road::models::vehicules::Turn;

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

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        path::Path::new_path(&mut canvas);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    if Vehicule::can_add_vehicle(&mut last_key_press, Keycode::Up) {
                        vehicles.push(Vehicule::new(
                            &sdl_context,
                            &mut canvas,
                            &texture_creator,
                            Direction::North,
                        )?);
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if Vehicule::can_add_vehicle(&mut last_key_press, Keycode::Down) {
                        vehicles.push(Vehicule::new(
                            &sdl_context,
                            &mut canvas,
                            &texture_creator,
                            Direction::South,
                        )?);
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    if Vehicule::can_add_vehicle(&mut last_key_press, Keycode::Left) {
                        vehicles.push(Vehicule::new(
                            &sdl_context,
                            &mut canvas,
                            &texture_creator,
                            Direction::West,
                        )?);
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    if Vehicule::can_add_vehicle(&mut last_key_press, Keycode::Right) {
                        vehicles.push(Vehicule::new(
                            &sdl_context,
                            &mut canvas,
                            &texture_creator,
                            Direction::East,
                        )?);
                    }
                }
                _ => {}
            }
        }
        let vehicle_positions: Vec<(i32, i32, Direction, Turn)> = vehicles
            .iter()
            .map(|v| (v.x, v.y, v.direction, v.turn))
            .collect();

        for i in 0..vehicles.len() {
            vehicles[i].update_position(&vehicle_positions);
            vehicles[i].render(&mut canvas)?;
        }

        canvas.present();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}