extern crate minifb;

use minifb::{Window, WindowOptions};

const ORIGINAL_WIDTH: usize = 64;
const ORIGINAL_HEIGHT: usize = 32;

pub const SCALE: usize = 5;
pub const WIDTH: usize = ORIGINAL_WIDTH * SCALE;
pub const HEIGHT: usize = ORIGINAL_HEIGHT * SCALE;

pub struct Display
{
    window: minifb::Window,
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

impl Display
{
    pub fn new() -> Display
    {
        let win = Window::new(
            "Rust Chip8 emulator",
            WIDTH,
            HEIGHT,
            WindowOptions::default(),
        ).expect("could not create windows");

        // win.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        let mut dis = Display { window: win, width: WIDTH, height: HEIGHT, buffer: vec![0x000000; WIDTH * HEIGHT], };
        dis.update_pixel();

        return dis;
    }

    pub fn get_width(&self) -> u16 { return self.width as u16; }
    // pub fn get_height(&self) -> usize { return self.height; }
    pub fn is_open(&self) -> bool { return self.window.is_open(); }

    pub fn clear_display(&mut self)
    {
        for i in 0 .. self.width * self.height
        {
            self.write_buffer(i as u16, 0);
        }
        self.update_pixel();
    }

    pub fn update_pixel(&mut self)
    {
        self.window.update_with_buffer(&self.buffer, self.width, self.height).expect("cannot draw pixel");
    }

    pub fn write_buffer(&mut self, index: u16, color: u32)
    {
        self.buffer[index as usize] = color;
    }

    pub fn is_pixel(&self, index: u16) -> u8
    {
        let pixel = self.buffer[index as usize];

        match pixel
        {
            0x000000 => 0,
            0xffffff => 1,
            _ => 2,
        }
    }
}