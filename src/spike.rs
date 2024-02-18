use lazy_static::lazy_static;
use std::sync::Mutex;

#[link(name = "spike-interfaces")]
extern "C" {
	pub fn spike_new() -> u64;
	pub fn spike_execute(spike: u64) -> i32;
	pub fn spike_init(spike: u64, entry_addr: u64) -> i32;
}

#[no_mangle]
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
	pub fn new(mem_size: usize) -> Self {
		let mut spike_opt = SPIKE.lock().unwrap();
		if spike_opt.is_none() {
			println!("Creating Spike with size: {}", mem_size);
			*spike_opt = Some(Box::new(Spike {
				mem: vec![0; mem_size],
			}));
		}

		SpikeHandle {
			size: mem_size,
			id: unsafe { spike_new() },
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
		let spike = SPIKE.lock().unwrap();
		let spike_ref = spike.as_ref().unwrap();

		unsafe { Ok(spike_execute(self.id)) }
	}

	pub fn init(&self, entry_addr: u64) -> anyhow::Result<i32> {
		let spike = SPIKE.lock().unwrap();
		let spike_ref = spike.as_ref().unwrap();

		unsafe { Ok(spike_init(self.id, entry_addr)) }
	}
}
