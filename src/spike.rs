use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::{info, trace};

#[link(name = "spike-interfaces")]
extern "C" {
	pub fn spike_new() -> u64;
	pub fn spike_execute() -> i32;
	pub fn spike_init(entry_addr: u64) -> i32;
	pub fn spike_register_callback(cb: extern fn(u64) -> *mut u8) -> i32;
}

// read the addr from spike memory
// caller should make sure the address is valid
pub extern "C" fn rs_addr_to_mem(addr: u64) -> *mut u8 {
	let addr = addr as usize;
	let mut spike = SPIKE.lock().unwrap();
	let spike_mut = spike.as_mut().unwrap();
	&mut spike_mut.mem[addr] as *mut u8
}

pub struct Spike {
	pub mem: Vec<u8>,
}

lazy_static! {
	static ref SPIKE: Mutex<Option<Box<Spike>>> = Mutex::new(None);
}

pub struct SpikeHandle {
	pub size: usize,
	pub id: u64,
}

impl SpikeHandle {
	pub fn new(size: usize) -> Self {
		// register the callback function
		let r = unsafe { spike_register_callback(rs_addr_to_mem) };

		// create a new spike instance
		let mut spike_opt = SPIKE.lock().unwrap();
		if spike_opt.is_none() {
			info!("Creating Spike with size: {}", size);
			*spike_opt = Some(Box::new(Spike {
				mem: vec![0; size],
			}));
		}

		// get the spike id
		let id = unsafe { spike_new() };
		info!("Spike id: {}", id);
		SpikeHandle {
			size,
			id,
		}
	}

	pub fn ld(&self, addr: usize, len: usize, bytes: Vec<u8>) -> anyhow::Result<()> {
		let mut spike = SPIKE.lock().unwrap();
		let spike_ref = spike.as_mut().unwrap();

		let dst = &mut spike_ref.mem[addr..addr + len];
		for (i, byte) in bytes.iter().enumerate() {
			dst[i] = *byte;
		}

		Ok(())
	}

	pub fn exec(&self) -> anyhow::Result<i32> {
		unsafe { Ok(spike_execute()) }
	}

	pub fn init(&self, entry_addr: u64) -> anyhow::Result<i32> {
		unsafe { Ok(spike_init(entry_addr)) }
	}
}
