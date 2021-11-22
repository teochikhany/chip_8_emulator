extern crate minifb;

use minifb::{Window, WindowOptions};

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
        let w = 640;
        let h = 320;

        let mut win = Window::new(
            "Rust Chip8 emulator",
            w,
            h,
            WindowOptions::default(),
        ).expect("could not create windows");

        win.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        return Display { window: win, width: w, height: h, buffer: vec![0; w * h], };
    }

    pub fn clear_display(&mut self)
    {
        for i in 0 .. self.width * self.height
        {
            self.write_buffer(i, 0);
        }
        self.update_pixel();
    }


    pub fn update_pixel(&mut self)
    {
        self.window.update_with_buffer(&self.buffer, self.width, self.height).expect("cannot draw pixel");
    }

    pub fn write_buffer(&mut self, index: usize, data: u32)
    {
        self.buffer[index] = data;
    }
}