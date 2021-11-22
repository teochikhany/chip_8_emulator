extern crate minifb;

use minifb::{Key, Window, WindowOptions};

pub struct Display
{
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

impl Display
{
    pub fn new() -> Display
    {
        let w = 640;
        let h = 320;
        return Display { width: w, height: h, buffer: vec![0; w * h] };
    }

    pub fn create_window(&self) -> minifb::Window
    {
        let mut window = Window::new(
            "Rust Chip8 emulator",
            self.width,
            self.height,
            WindowOptions::default(),
        ).expect("could not create windows");

        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        return window;
    }

    pub fn draw_pixel(&self, window: &mut minifb::Window)
    {
        window.update_with_buffer(&self.buffer, self.width, self.height).expect("cannot draw pixel");
    }
}