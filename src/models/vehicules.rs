use rand::Rng;
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Vehicule<'a> {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub direction: Direction,
    pub angle: f64,
    pub texture: Texture<'a>,
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

        // Randomly select a lane (1, 2, or 3) for each direction
        let mut rng = rand::thread_rng();
        let lane = rng.gen_range(1..=3);

        // Set starting position and angle based on direction and lane
        let (x, y, angle) = match direction {
            Direction::North => match lane {
                1 => (425, 700, 0.0),
                2 => (350, 700, 0.0),
                3 => (390, 700, 0.0),
                _ => (350, 700, 0.0),
            },
            Direction::South => match lane {
                1 => (390, 0, 180.0),
                2 => (350, 0, 180.0),
                3 => (425, 0, 180.0),
                _ => (350, 0, 180.0),
            },
            Direction::East => match lane {
                1 => (0, 425, 90.0),
                2 => (0, 350, 90.0),
                3 => (0, 390, 90.0),
                _ => (0, 350, 90.0),
            },
            Direction::West => match lane {
                1 => (700, 390, 270.0),
                2 => (700, 350, 270.0),
                3 => (700, 425, 270.0),
                _ => (700, 350, 270.0),
            },
        };

        Ok(Vehicule {
            x,
            y,
            width: 50,
            height: 50,
            direction,
            angle,
            texture,
        })
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

    pub fn update_position(&mut self) {
        match self.direction {
            Direction::North => self.y -= 5,
            Direction::South => self.y += 5,
            Direction::East => self.x += 5,
            Direction::West => self.x -= 5,
        }
    }
}
