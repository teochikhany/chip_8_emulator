pub struct Ram
{
    memory: [u8; 4096],
}

impl Ram
{
    pub fn new() -> Ram
    {
        let sprites: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
            [0x20, 0x60, 0x20, 0x20, 0x70], // 1
            [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
            [0xF0, 0x10, 0xF0, 0x10, 0xF0],
            [0x90, 0x90, 0xF0, 0x10, 0x10],
            [0xF0, 0x80, 0xF0, 0x10, 0xF0],
            [0xF0, 0x80, 0xF0, 0x90, 0xF0],
            [0xF0, 0x10, 0x20, 0x40, 0x40],
            [0xF0, 0x90, 0xF0, 0x90, 0xF0],
            [0xF0, 0x90, 0xF0, 0x10, 0xF0],
            [0xF0, 0x90, 0xF0, 0x90, 0x90],
            [0xE0, 0x90, 0xE0, 0x90, 0xE0],
            [0xF0, 0x80, 0x80, 0x80, 0xF0],
            [0xE0, 0x90, 0x90, 0x90, 0xE0],
            [0xF0, 0x80, 0xF0, 0x80, 0xF0],
            [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
        ];

        let mut mem : [u8; 4096] = [0; 4096];

        let mut i = 0;
        for sprite in sprites
        {
            for ch in sprite
            {
                mem[i] = ch;
                i += 1;
            }
        }

        return Ram{memory : mem};
    }

    pub fn read(&self, start: u16, size: u16) -> Vec<u8>
    {
        let mut result: Vec<u8> = Vec::new();

        for i in start .. start + size
        {
            let i = i as usize;
            result.push(self.memory[i]);
        }

        return result;
    }

    pub fn read_byte(&self, start: u16) -> u8
    {
        return self.memory[start as usize];
    }

    pub fn write(& mut self, start: u16, data: &[u8])
    {
        let mut i = start;
        for byte in data
        {
            self.memory[i as usize] = *byte;
            i += 1;
        }
    }

    pub fn write_byte(& mut self, start: u16, data: u8)
    {
        self.memory[start as usize] = data;
    }
}