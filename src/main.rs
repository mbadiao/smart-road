use smart_road::models::{path, vehicules::Vehicule, vehicules::Direction};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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
                    vehicles.push(Vehicule::new(
                        &sdl_context,
                        &mut canvas,
                        &texture_creator,
                        Direction::North,
                    )?);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    vehicles.push(Vehicule::new(
                        &sdl_context,
                        &mut canvas,
                        &texture_creator,
                        Direction::South,
                    )?);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    vehicles.push(Vehicule::new(
                        &sdl_context,
                        &mut canvas,
                        &texture_creator,
                        Direction::West,
                    )?);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    vehicles.push(Vehicule::new(
                        &sdl_context,
                        &mut canvas,
                        &texture_creator,
                        Direction::East,
                    )?);
                }
                _ => {}
            }
        }

        for vehicle in &mut vehicles {
            vehicle.update_position();
            vehicle.render(&mut canvas)?;
        }

        canvas.present();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}
