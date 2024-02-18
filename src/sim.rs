use std::fs::File;
use std::io::Read;
use xmas_elf::{
	header,
	program::{ProgramHeader, Type},
	ElfFile,
};

use crate::spike::Spike;

pub struct Sim {
	spike: Spike,
}

impl Sim {
	pub fn new(size: usize) -> Self {
		Self {
			spike: Spike::new(size as u64),
		}
	}

	pub fn load_elf(&mut self, fname: &str) -> Result<(), Box<dyn std::error::Error>> {
		let mut file = File::open(fname).unwrap();
		let mut buffer = Vec::new();
		file.read_to_end(&mut buffer).unwrap();

		let elf_file = ElfFile::new(&buffer).unwrap();

		let header = elf_file.header;
		assert_eq!(header.pt2.machine().as_machine(), header::Machine::RISC_V);
		assert_eq!(header.pt1.class(), header::Class::ThirtyTwo);

		for ph in elf_file.program_iter() {
			match ph {
				ProgramHeader::Ph32(ph) => {
					if ph.get_type() == Ok(Type::Load) {
						let offset = ph.offset as usize;
						let size = ph.file_size as usize;
						let addr = ph.virtual_addr as usize;

						let slice = &buffer[offset..offset + size];
						assert!(addr + size < self.spike.size);
						println!("addr: {addr}, size: 0x{:x}", size);
						self
							.spike
							.ld_elf(addr as u64, size as u64, slice.as_ptr() as *mut u8).unwrap();
					}
				}
				_ => (),
			}
		}

		// init the spike with the entry point.
		self.spike.init(header.pt2.entry_point()).unwrap();

		Ok(())
	}

	pub fn execute(&mut self) -> Result<(), Box<dyn std::error::Error>> {
		self.spike.execute().unwrap();

		Ok(())
	}
}
