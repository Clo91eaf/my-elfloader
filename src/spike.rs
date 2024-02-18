#[link(name = "spike-interfaces")]
extern "C" {
	pub fn spike_new(mem_size: u64) -> u64;
	pub fn spike_delete(spike: u64) -> i32;
	pub fn spike_execute(spike: u64) -> i32;
	pub fn spike_get_reg(spike: u64, index: u64, content: *mut u64) -> i32;
	pub fn spike_set_reg(spike: u64, index: u64, content: u64) -> i32;
	pub fn spike_ld(spike: u64, addr: u64, len: u64, bytes: *mut u8) -> i32;
	pub fn spike_sd(spike: u64, addr: u64, len: u64, bytes: *mut u8) -> i32;
	pub fn spike_ld_elf(spike: u64, addr: u64, len: u64, bytes: *mut u8) -> i32;
	pub fn spike_init(spike: u64, entry_addr: u64) -> i32;
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
		println!("Creating Spike with size: {}", mem_size);
		unsafe {
			Self {
				addr: spike_new(mem_size),
				size: mem_size as usize,
			}
		}
	}

	pub fn execute(&self) -> Result<(), Error> {
		let r = unsafe { spike_execute(self.addr) };
		match r {
			0 => Ok(()),
			_ => Err(Error(r)),
		}
	}

	pub fn get_reg(&self, index: u64) -> Result<u64, Error> {
		let mut x = 0;
		let r = unsafe { spike_get_reg(self.addr, index, &mut x) };
		match r {
			0 => Ok(x),
			_ => Err(Error(r)),
		}
	}

	pub fn set_reg(&self, index: u64, content: u64) -> Result<(), Error> {
		let r = unsafe { spike_set_reg(self.addr, index, content) };
		match r {
			0 => Ok(()),
			_ => Err(Error(r)),
		}
	}

	pub fn ld(&self, addr: u64, len: u64, bytes: *mut u8) -> Result<(), Error> {
		let r = unsafe { spike_ld(self.addr, addr, len, bytes) };
		match r {
			0 => Ok(()),
			_ => Err(Error(r)),
		}
	}

	pub fn sd(&self, addr: u64, len: u64, bytes: *mut u8) -> Result<(), Error> {
		let r = unsafe { spike_sd(self.addr, addr, len, bytes) };
		match r {
			0 => Ok(()),
			_ => Err(Error(r)),
		}
	}

	pub fn ld_elf(&self, addr: u64, len: u64, bytes: *mut u8) -> Result<(), Error> {
		let r = unsafe { spike_ld_elf(self.addr, addr, len, bytes) };
		match r {
			0 => Ok(()),
			_ => Err(Error(r)),
		}
	}

	pub fn init(&self, entry_addr: u64) -> Result<(), Error> {
		let r = unsafe { spike_init(self.addr, entry_addr) };
		match r {
			0 => Ok(()),
			_ => Err(Error(r)),
		}
	}
}

impl Drop for Spike {
	fn drop(&mut self) {
		println!("Dropping Spike");
		let r = unsafe { spike_delete(self.addr) };
		match r {
			0 => (),
			_ => panic!("spike_delete failed"),
		}
	}
}
