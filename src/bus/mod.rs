extern crate nalgebra as na;
#[derive(Copy, Clone)]
pub struct Bus{
    addr_space: [u32; 0xffff]
}

impl Bus{
    pub fn init() -> Bus{
        Bus{addr_space: [0; 0xffff]}
    }

    pub fn write(&mut self, addr: u32, val: u32) {
        // addr is DWORD (4 bytes)
        self.addr_space[(addr as i32 /4) as usize] = val;
    }

    pub fn read(self, addr: u32) -> u32{
        // addr is DWORD (4 bytes)
        self.addr_space[(addr as i32/4) as usize]
    }
}
