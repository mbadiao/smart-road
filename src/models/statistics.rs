extern crate sdl2;

// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
// use sdl2::ttf::Font;

#[derive(Debug)]
pub struct Statistics {
    pub number_of_vehicles: i32,
    pub max_velocity: i32,
    pub min_velocity: i32,
    pub max_time_to_pass_intersection: i32,
    pub min_time_to_pass_intersection: i32,
    pub close_calls: i32,
    pub show_statistics: bool,
}

impl Statistics {
    pub fn new() -> Statistics {
        Statistics {
            number_of_vehicles: 0,
            max_velocity: 0,
            min_velocity: 0,
            max_time_to_pass_intersection: 0,
            min_time_to_pass_intersection: 0,
            close_calls: 0,
            show_statistics: false,
        }
    }
    pub fn display(&self, canvas: &mut Canvas<Window>) {
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
        let font = ttf_context.load_font("./src/models/Roboto-BlackItalic.ttf", 24).unwrap();

        let text_surface = font
            .render(&format!(
                "Statistics:\nNumber of Vehicles: {}",
                self.number_of_vehicles,
            ))
            .blended(Color::RGB(255, 255, 255))
            .unwrap();
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(&text_surface).unwrap();

        let (w, h) = text_surface.size();
        let rect = Rect::new((640 - w as i32) / 2, (480 - h as i32) / 2, w as u32, h as u32);
        canvas.copy(&texture, None, Some(rect)).unwrap();
        // canvas.present();
    }

   
    pub fn toggle_statistics_display(&mut self, canvas: &mut Canvas<Window>) {
        self.show_statistics = !self.show_statistics;
        if self.show_statistics {
            self.display(canvas);
        }
    }

    pub fn increment(&mut self) {
        self.number_of_vehicles +=1 ;
        // println!("{}",self.number_of_vehicles);
    }


}
