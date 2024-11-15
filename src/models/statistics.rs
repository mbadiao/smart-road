extern crate sdl2;
use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use super::vehicules::Vehicule;

#[derive(Debug)]
pub struct Statistics {
    pub number_of_vehicles: f32,
    pub max_velocity: i32,
    pub min_velocity: f32,
    pub max_time: Duration,
    pub min_time: Duration,
    pub close_calls: f32,
    pub show_statistics: bool,
}

impl Statistics {
    pub fn new() -> Statistics {
        Statistics {
            number_of_vehicles: 0.,
            max_velocity: 0,
            min_velocity: 0.,
            max_time: Duration::new(0, 0),
            min_time: Duration::new(0, 0),
            close_calls: 0.,
            show_statistics: false,
        }
    }
    pub fn display(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        let texture_creator = canvas.texture_creator();
        let font_path = "./src/models/Roboto-BlackItalic.ttf";
        let font_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let font = font_context.load_font(font_path, 20)?;
        let font_small = font_context.load_font(font_path, 16)?;

        let title = "================ STATISTICS ================";
        let stats_text = vec![
            format!(
                "Max number of vehicles that passed the intersection: {}",
                self.number_of_vehicles
            ),
            format!("Max velocity of all vehicles: {:.1} m/s", self.max_velocity),
            format!("Min velocity of all vehicles: {:.1} m/s", self.min_velocity),
            format!(
                "Max time that the vehicles took to pass the intersection: {:.2} seconds",
                self.max_time.as_secs_f64()
            ),
            format!(
                "Min time that the vehicles took to pass the intersection: {:.2} seconds",
                self.min_time.as_secs_f64()
            ),
            format!("Close calls: {}", self.close_calls),
            format!("Collisions: {}", 0),
        ];
        let quit_message = "Press Esc again to Quit";

        let title_surface = font
            .render(title)
            .blended(Color::RGBA(255, 255, 255, 255))
            .map_err(|e| e.to_string())?;
        let title_texture = texture_creator
            .create_texture_from_surface(&title_surface)
            .map_err(|e| e.to_string())?;
        canvas.copy(
            &title_texture,
            None,
            Some(Rect::new(
                50,
                30,
                title_surface.width(),
                title_surface.height(),
            )),
        )?;

        for (i, text) in stats_text.iter().enumerate() {
            let surface = font_small
                .render(text)
                .blended(Color::RGBA(200, 200, 200, 255))
                .map_err(|e| e.to_string())?;
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;

            let y_position = 80 + i as i32 * 40;
            canvas.copy(
                &texture,
                None,
                Some(Rect::new(50, y_position, surface.width(), surface.height())),
            )?;
        }

        let quit_surface = font_small
            .render(quit_message)
            .blended(Color::RGBA(255, 255, 255, 255))
            .map_err(|e| e.to_string())?;
        let quit_texture = texture_creator
            .create_texture_from_surface(&quit_surface)
            .map_err(|e| e.to_string())?;
        let quit_y_position = 120 + stats_text.len() as i32 * 40;
        canvas.copy(
            &quit_texture,
            None,
            Some(Rect::new(
                50,
                quit_y_position,
                quit_surface.width(),
                quit_surface.height(),
            )),
        )?;

        canvas.present();
        Ok(())
    }

    pub fn toggle_statistics_display(&mut self, canvas: &mut Canvas<Window>) {
        self.show_statistics = !self.show_statistics;
        if self.show_statistics {
            _ = self.display(canvas);
        }
    }

    pub fn increment(&mut self, vehicle: &[Vehicule]) {
        self.number_of_vehicles = vehicle
            .iter()
            .filter(|v| v.passed_inter == true)
            .collect::<Vec<&Vehicule>>()
            .len() as f32;
    }

    pub fn max_velocity(&mut self, vehicle: &[Vehicule]) {
        for vehi in vehicle {
            if self.max_velocity < vehi.velocity {
                self.max_velocity = vehi.velocity;
            }
        }
    }

    pub fn min_velocity(&mut self, vehicle: &mut [Vehicule]) {
        for vehi in vehicle {
            if self.min_velocity > vehi.velocity as f32 {
                self.min_velocity = vehi.velocity as f32;
            }
        }
    }

    pub fn get_time(&mut self, vehicles: &mut [Vehicule]) {
        for vehi in vehicles.iter_mut() { 
            if vehi.passed_inter && !vehi.time_recorded {
                let duration = vehi.duration.elapsed();
                self.update_max_time(duration);
                self.update_min_time(duration);
                vehi.time_recorded = true; 
            }
        }
    }
    

    pub fn update_max_time(&mut self, time: Duration) {
        if time > self.max_time {
            self.max_time = time;
        }
    }

    pub fn update_min_time(&mut self, time: Duration) {
        if self.min_time == Duration::new(0, 0) || time < self.min_time {
            self.min_time = time;
        }
    }
}
