use lazy_static::lazy_static;
use std::sync::Mutex;

#[link(name = "spike-interfaces")]
extern "C" {
	pub fn spike_execute(spike: u64) -> i32;
	pub fn spike_get_reg(spike: u64, index: u64, content: *mut u64) -> i32;
	pub fn spike_set_reg(spike: u64, index: u64, content: u64) -> i32;
	pub fn spike_init(spike: u64, entry_addr: u64) -> i32;
}

#[no_mangle]
// read the memory from spike
pub extern "C" fn rs_addr_to_mem(addr: u64) -> *mut u8 {
	let addr = addr as usize;
	let spike = SPIKE.lock().unwrap();
	let spike_ref = spike.as_ref().unwrap();
	if addr < spike_ref.size {
		&mut spike_ref.mem[addr] as *mut u8
	} else {
		panic!("Address out of bounds");
	}
}

pub struct Spike {
	pub size: usize,
	pub mem: Vec<u8>,
}

lazy_static! {
	static ref SPIKE: Mutex<Option<Box<Spike>>> = Mutex::new(None);
}

pub fn spike_new(mem_size: usize) -> anyhow::Result<()> {
	let mut spike_opt = SPIKE.lock().unwrap();
	if spike_opt.is_none() {
		println!("Creating Spike with size: {}", mem_size);
		*spike_opt = Some(Box::new(Spike {
			mem: vec![0; mem_size],
			size: mem_size,
		}));
	}

	Ok(())
}

pub fn spike_size() -> usize {
	let spike = SPIKE.lock().unwrap();
	let spike_ref = spike.as_ref().unwrap();
	spike_ref.size
}

pub fn spike_ld(addr: usize, len: usize, bytes: Vec<u8>) -> anyhow::Result<()> {
	let spike = SPIKE.lock().unwrap();
	let spike_ref = spike.as_ref().unwrap();

	let dst = &mut spike_ref.mem[addr..addr + len];
	for (i, byte) in bytes.iter().enumerate() {
		dst[i] = *byte;
	}

	Ok(())
}
