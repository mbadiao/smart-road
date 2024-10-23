use sdl2::render::Canvas;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::video::Window;

pub fn new_path(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for i in (0..900).step_by(20) {
        let _ = canvas.fill_rect(Rect::new(370, i, 1, 10)); 

        let _ = canvas.fill_rect(Rect::new(410, i, 1, 10)); 

        let _ = canvas.fill_rect(Rect::new(490, i, 1, 10));

        let _ = canvas.fill_rect(Rect::new(530, i, 1, 10));



        let _ = canvas.fill_rect(Rect::new(i, 370, 10, 1)); 

        let _ = canvas.fill_rect(Rect::new(i, 410, 10, 1)); 

        let _ = canvas.fill_rect(Rect::new(i, 490, 10, 1));
        
        let _ = canvas.fill_rect(Rect::new(i, 530, 10, 1));


    }
    let _ = canvas.fill_rect(Rect::new(330, 0, 1, 900)); 

    let _ = canvas.fill_rect(Rect::new(450, 0, 1, 900));
    
    let _ = canvas.fill_rect(Rect::new(570, 0, 1, 900));


    let _ = canvas.fill_rect(Rect::new(0, 330, 900, 1)); 

    let _ = canvas.fill_rect(Rect::new(0, 450, 900, 1));
    
    let _ = canvas.fill_rect(Rect::new(0, 570,  900, 1));

    canvas.set_draw_color(Color::RGB(0, 0, 0));

    let _ = canvas.fill_rect(Rect::new(330, 330,  570 - 329, 570 - 329));

    
    canvas.present();
}