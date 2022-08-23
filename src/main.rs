mod drawing_window;

use std::time::Duration;

use drawing_window::DrawingWindow;
use sdl2::{event::Event, keyboard::Keycode};



pub fn main() {

    let mut drawing_window = DrawingWindow::new_drawing_window(800, 600, false);
    let sdl_context = drawing_window.get_sdl_context();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        drawing_window.set_pixel_color(150, 150, 155);
        drawing_window.render_frame();
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
