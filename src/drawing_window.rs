

use std::{borrow::BorrowMut, cell::{RefCell, Ref}};

use rand::Error;
use sdl2::{self, video::Window, render::{Canvas, TextureCreator, Texture}, Sdl, pixels::{Color,PixelFormatEnum}, rect::Point};

pub struct DrawingWindow{
    pub width: u32,
    pub height : u32,
    
    context : sdl2::Sdl,
    canvas : Canvas<Window>,
    creator : TextureCreator<sdl2::video::WindowContext>,
    texture : RefCell<Texture<'static>>,

    pixel_buffer: Vec<u32>
}

impl DrawingWindow {
    pub fn new_drawing_window(w: u32, h: u32, fullscreen: bool) -> Result<Self,Error>{
        let pixel_buffer: Vec<u32> = vec![0; (w * h) as usize];
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

        let creator = canvas.texture_creator();
        let texture = creator.create_texture_target(
            PixelFormatEnum::RGBA8888,w,h).unwrap();

        let texture = unsafe{
            std::mem::transmute::<_,Texture<'static>>(texture)
        };

        // println!("The length of var pixel_buffer is {0}",pixel_buffer.len());
        Ok({
            DrawingWindow { width: w, height: h, context: sdl_context, canvas: canvas, creator: creator, texture:RefCell::new(texture),pixel_buffer:pixel_buffer}

        })
    }



    pub fn render_frame(&mut self){
        let mut texture = self.texture.borrow_mut();
        texture.update(None, self.data_raw(),
            (self.width*4) as usize).unwrap();
        self.canvas.copy(&texture,None,None).unwrap();
        self.canvas.present();
    }

    pub fn clear_pixels(&mut self){
        self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        self.canvas.clear();
    }

    pub fn set_pixel_color(&mut self,x: u32, y:u32, color:u32){
        match x >= self.width || y >= self.height {
            true => println!("{0},{1} not on visible screen area",x,y),
            _ => {
                // println!("The length of var pixel_buffer is {0}",self.pixel_buffer.len());
                //let color = u32_to_bytes(&[color]);
                self.pixel_buffer[((y*self.width)+x) as usize] = color;
                //self.set_colour(Point::new(x as i32, y as i32),Color::RGBA(color[0], color[1], color[2], color[3]));
            }
        }
    }

    pub fn get_sdl_context(&self) -> &Sdl{
        &self.context
    }
    

    fn data_raw(&self) -> &[u8] {
        unsafe{std::slice::from_raw_parts(
            self.pixel_buffer.as_ptr() as *const u8,
            self.pixel_buffer.len()*4)}
    }
    
}