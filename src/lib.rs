#[link(name = "spike-interfaces")]
extern "C" {
    pub fn spike_new(mem_size: u64) -> u64;
    pub fn spike_delete(spike: u64);
    pub fn spike_execute(spike: u64, instruction: u64) -> i32;
    pub fn spike_get_reg(spike: u64, index: u64, content: *mut u64) -> i32;
    pub fn spike_set_reg(spike: u64, index: u64, content: u64) -> i32;
    pub fn spike_ld(spike: u64, addr: u64, len: u64, bytes: *mut u8) -> i32;
    pub fn spike_sd(spike: u64, addr: u64, len: u64, bytes: *mut u8) -> i32;
}

#[derive(Debug)]
pub struct Error(i32);
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error({})", self.0)
    }
}
impl std::error::Error for Error {}

pub struct Spike {
    pub addr: u64,
    pub size: usize,
}

impl Spike {
    pub fn new(mem_size: u64) -> Self {
        unsafe {
            Self {
                addr: spike_new(mem_size),
                size: mem_size as usize,
            }
        }
    }

    pub fn execute(&self, instruction: u64) -> Result<(), Error> {
        let r = unsafe { spike_execute(self.addr, instruction) };
        if r != 0 {
            Err(Error(r))
        } else {
            Ok(())
        }
    }

    pub fn get_reg(&self, index: u64) -> Result<u64, Error> {
        let mut x = 0;
        let r = unsafe { spike_get_reg(self.addr, index, &mut x) };
        if r != 0 {
            Err(Error(r))
        } else {
            Ok(x)
        }
    }

    pub fn set_reg(&self, index: u64, content: u64) -> Result<(), Error> {
        let r = unsafe { spike_set_reg(self.addr, index, content) };
        if r != 0 {
            Err(Error(r))
        } else {
            Ok(())
        }
    }

    pub fn ld(&self, addr: u64, len: u64, bytes: *mut u8) -> Result<(), Error> {
        let r = unsafe { spike_ld(self.addr, addr, len, bytes) };
        if r != 0 {
            Err(Error(r))
        } else {
            Ok(())
        }
    }

    pub fn sd(&self, addr: u64, len: u64, bytes: *mut u8) -> Result<(), Error> {
        let r = unsafe { spike_sd(self.addr, addr, len, bytes) };
        if r != 0 {
            Err(Error(r))
        } else {
            Ok(())
        }
    }
}

impl Drop for Spike {
    fn drop(&mut self) {
        unsafe { spike_delete(self.addr) }
    }
}
