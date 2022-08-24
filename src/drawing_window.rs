

use sdl2::{self, video::Window, render::Canvas, Sdl, pixels::Color, rect::Point};

pub struct DrawingWindow{
    pub width: u32,
    pub height : u32,
    
    context : sdl2::Sdl,
    canvas : Canvas<Window>,
    pixel_buffer:Vec<(Point,Color)>
}

impl DrawingWindow {
    pub fn new_drawing_window(w: u32, h: u32, fullscreen: bool) -> DrawingWindow{
        let pixel_buffer: Vec<(Point,Color)> = vec![];
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
        //self.clear_pixels();
        self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        self.canvas.clear();
        'settle: loop{
            let item = match self.pixel_buffer.pop(){
                Some(T) => T,
                None => break 'settle
            };
            
            self.set_colour(item.0, item.1);
        }
        
        self.canvas.present();
    }

    pub fn clear_pixels(&mut self){
        self.pixel_buffer = vec![];
    }

    pub fn set_pixel_color(&mut self,x: u32, y:u32, color:u32){
        match x >= self.width || y >= self.height {
            true => println!("{0},{1} not on visible screen area",x,y),
            _ => {
                // println!("The length of var pixel_buffer is {0}",self.pixel_buffer.len());
                let color = to_bytes(&[color]);
                self.pixel_buffer.push((Point::new(x as i32, y as i32),Color::RGBA(color[0], color[1], color[2], color[3])));
            }
        }
    }

    

    pub fn get_sdl_context(&self) -> &Sdl{
        &self.context
    }


    fn set_colour(&mut self,point:Point, color:Color){
        self.canvas.set_draw_color(color);
        self.canvas.draw_point(point).expect("Draw Pixel Failed")
    }
    
}

fn to_bytes(input: &[u32]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(4 * input.len());

    for value in input {
        bytes.extend(&value.to_be_bytes());
    }

    bytes
}