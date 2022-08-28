mod drawing_window;

use std::time::Duration;
use rand::Rng;
use drawing_window::DrawingWindow;
use sdl2::{event::Event, keyboard::Keycode, EventPump};


fn draw(window:&mut DrawingWindow){
    window.clear_pixels();
    for i in 0..window.height{
        for j in 0..window.width{
            let red:u32 = rand::thread_rng().gen_range(1..=255);
            // let red:u32 = 255;
            let green:u32 = 0;
            let blue:u32 = 0;
            let colour:u32 = (red<<24) + (green<<16) + (blue<<8) + 0;
            window.set_pixel_color(j, i, colour);
        }
    }
}

pub fn main() {

    let mut drawing_window = DrawingWindow::new_drawing_window(320, 240, false).unwrap();
    let sdl_context = drawing_window.get_sdl_context();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        draw(&mut drawing_window);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        drawing_window.render_frame();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
   
   
}
