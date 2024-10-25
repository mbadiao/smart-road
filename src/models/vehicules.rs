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

        let (x, y) = match direction {
            Direction::North => (450, 900),
            Direction::South => (450, 0),
            Direction::East => (0, 450),
            Direction::West => (900 , 450),
        };

        Ok(Vehicule {
            x,
            y,
            width: 50,
            height: 50,
            direction,
            texture,
        })
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        let target_rect = sdl2::rect::Rect::new(self.x, self.y, self.width, self.height);
        canvas.copy(&self.texture, None, Some(target_rect))?;
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
