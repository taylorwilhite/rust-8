use sdl2;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;


const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const SCALE: u32 = 10;
const SCREEN_WIDTH: u32 = (WIDTH as u32) * SCALE;
const SCREEN_HEIGHT: u32 = (HEIGHT as u32) * SCALE;

pub struct Display {
  canvas: Canvas<Window>
}

impl Display {
  pub fn new(sdl_context: &sdl2::Sdl) -> Self {
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-8 emulator", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    
    Display { canvas: canvas }
  }

  pub fn draw(&mut self, pixels: &[[u8; WIDTH]; HEIGHT]) {
    for (y, row) in pixels.iter().enumerate() {
      for (x, &col) in row.iter().enumerate() {
        let x = (x as u32) * SCALE;
        let y = (y as u32) * SCALE;

        self.canvas.set_draw_color(color(col));
        self.canvas
          .fill_rect(Rect::new(x as i32, y as i32, SCALE, SCALE)).expect("something went wrong");
      }
    }
    self.canvas.present();
  }
}

fn color(value: u8) -> Color {
    if value == 0 {
        Color::RGB(0, 0, 0)
    } else {
        Color::RGB(0, 0, 250)
    }
}