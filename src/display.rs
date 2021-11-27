extern crate minifb;
use minifb::{Window, WindowOptions, Key, KeyRepeat};

const ORIGINAL_WIDTH: usize = 64;
const ORIGINAL_HEIGHT: usize = 32;

pub const SCALE: usize = 10;
const WIDTH: usize = ORIGINAL_WIDTH * SCALE;
const HEIGHT: usize = ORIGINAL_HEIGHT * SCALE;

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

    pub fn get_width(&self) -> u32 { return self.width as u32; }
    pub fn is_open(&self) -> bool { return self.window.is_open(); }

    pub fn clear_display(&mut self)
    {
        for i in 0 .. self.width * self.height
        {
            self.write_buffer(i as u32, 0);
        }
        self.update_pixel();
    }

    pub fn update_pixel(&mut self)
    {
        self.window.update_with_buffer(&self.buffer, self.width, self.height).expect("cannot draw pixel");
    }

    pub fn write_buffer(&mut self, index: u32, color: u32)
    {
        self.buffer[index as usize] = color;
    }

    pub fn is_pixel(&self, index: u32) -> u8
    {
        let pixel = self.buffer[index as usize];

        match pixel
        {
            0x000000 => 0,
            0xffffff => 1,
            _ => 2,
        }
    }

    pub fn is_pressed(&self, int_key: u8) -> bool
    {
        let mut result = false;
        let keys = self.window.get_keys_pressed(KeyRepeat::Yes).unwrap();

        for key in keys
        {
            let true_key = from_keyboard_to_chip(key);
            if true_key == int_key
            {
                result = true;
                break;
            }
        }

        return result;
    }

    // FIXME:
    pub fn wait_key(&self) -> u8
    {
        // loop
        // {
        //     let keys = self.window.get_keys_pressed(KeyRepeat::Yes).unwrap();

        //     if keys.len() != 0
        //     {
        //         return from_keyboard_to_chip(keys[0]);
        //     }

        //     thread::sleep(TEN_MILIS);
        // }

        return 0;
    }
}


fn from_keyboard_to_chip(key: Key) -> u8
{
    match key {
        Key::Key1 => 0x1,
        Key::Key2 => 0x2,
        Key::Key3 => 0x3,
        Key::Key4 => 0xC,

        Key::Q => 0x4,
        Key::W => 0x5,
        Key::E => 0x6,
        Key::R => 0xD,

        Key::A => 0x7,
        Key::S => 0x8,
        Key::D => 0x9,
        Key::F => 0xE,

        Key::Z => 0xA,
        Key::X => 0x0,
        Key::C => 0xB,
        Key::V => 0xF,
        _ => 0x0,
    }
}