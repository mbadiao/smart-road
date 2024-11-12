use sdl2::render::Canvas;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::video::Window;
use super::vehicules::Vehicule;


pub struct  Path<'a> {
   pub vehicules : Vec< Vehicule<'a>>,
}

impl<'a>  Path<'a> {
    pub fn new_path(canvas: &mut Canvas<Window>)  -> Path<'a> {
        // Fond noir
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
    
        // Couleur des lignes blanches
        canvas.set_draw_color(Color::RGB(255, 255, 255));
    
        // Lignes verticales et horizontales centrées
        for i in (0..700).step_by(20) {
            // Lignes verticales
            let _ = canvas.fill_rect(Rect::new(270, i, 1, 10));
            let _ = canvas.fill_rect(Rect::new(310, i, 1, 10));
            let _ = canvas.fill_rect(Rect::new(390, i, 1, 10));
            let _ = canvas.fill_rect(Rect::new(430, i, 1, 10));
    
            // Lignes horizontales
            let _ = canvas.fill_rect(Rect::new(i, 270, 10, 1));
            let _ = canvas.fill_rect(Rect::new(i, 310, 10, 1));
            let _ = canvas.fill_rect(Rect::new(i, 390, 10, 1));
            let _ = canvas.fill_rect(Rect::new(i, 430, 10, 1));
        }
    
        // Bordures principales du quadrillage
        let _ = canvas.fill_rect(Rect::new(230, 0, 1, 700));  // Gauche
        let _ = canvas.fill_rect(Rect::new(350, 0, 1, 700));  // Centre
        let _ = canvas.fill_rect(Rect::new(470, 0, 1, 700));  // Droite
        let _ = canvas.fill_rect(Rect::new(0, 230, 700, 1));  // Haut
        let _ = canvas.fill_rect(Rect::new(0, 350, 700, 1));  // Centre
        let _ = canvas.fill_rect(Rect::new(0, 470, 700, 1));  // Bas
    
        // Rectangles noirs pour les zones bloquées au centre
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        
        
        let _ = canvas.fill_rect(Rect::new(230, 230, 470 - 229, 470 - 229));
        
        Path {
            vehicules: Vec::new(),
        }
        // canvas.present();
    }


}