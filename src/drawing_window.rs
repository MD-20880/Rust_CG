

use sdl2::{self, video::Window, render::Canvas, Sdl};

pub struct DrawingWindow{
    pub width: u32,
    pub height : u32,
    
    context : sdl2::Sdl,
    canvas : Canvas<Window>,
    pixel_buffer:Vec<u32>
}

impl DrawingWindow {


    pub fn new_drawing_window(w: u32, h: u32, fullscreen: bool) -> DrawingWindow{
        let pixel_buffer: Vec<u32> = vec![0; (w * h) as usize];
        assert!(w*h > 100);
        let sdl_context = sdl2::init().expect("Could not initialise SDL");
        let video_subsystem = sdl_context.video().unwrap();
        let window =match fullscreen{
            true => {
                video_subsystem.window("rust-sdl2 demo", w as u32, h as u32)
            .opengl()
            .fullscreen()
            .build()
            .unwrap()
            },
            false => {
                video_subsystem.window("rust-sdl2 demo", w as u32, h as u32)
            .opengl()
            .build()
            .unwrap()
            }
        }; 
        let canvas = window.into_canvas().build().unwrap();
        // println!("The length of var pixel_buffer is {0}",pixel_buffer.len());
        DrawingWindow { width: w, height: h, context: sdl_context, canvas: canvas, pixel_buffer: pixel_buffer }
    }



    pub fn render_frame(&mut self){
        
        self.canvas.present();
    }

    pub fn set_pixel_color(&mut self,x: u32, y:u32, colour:u32){
        match x >= self.width || y >= self.height {
            true => println!("{0},{1} not on visible screen area",x,y),
            _ => {
                // println!("The length of var pixel_buffer is {0}",self.pixel_buffer.len());
                self.pixel_buffer[((y*self.height)+x) as usize] = colour;
            }
        }
    }

    pub fn get_sdl_context(&self) -> &Sdl{
        &self.context
    }
    
}
