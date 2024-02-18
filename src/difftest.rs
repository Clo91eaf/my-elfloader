use std::fs::File;
use std::io::Read;
use xmas_elf::{
	header,
	program::{ProgramHeader, Type},
	ElfFile,
};

use crate::spike::*;
use crate::dut::Dut;
use crate::{info, trace};

pub struct Difftest {
	dut: Dut,
}

impl Difftest {
	pub fn new(size: usize, fst_file: &str) -> Self {
		spike_new(size).unwrap();
		Self {
			dut: Dut::new(fst_file.to_string()),
		}
	}

	fn load_elf(&mut self, fname: &str) -> Result<u64, Box<dyn std::error::Error>> {
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
						assert!(addr + size < spike_size());
						info!("addr: {addr}, size: 0x{:x}", size);
						spike_ld(addr, size, slice.to_vec()).unwrap();
					}
				}
				_ => (),
			}
		}

		Ok(header.pt2.entry_point())
	}

	pub fn init(&mut self, elf_file: &str) -> Result<(), Box<dyn std::error::Error>> {
		let entry_addr = self.load_elf(elf_file).unwrap();
		self.spike.init(entry_addr).unwrap();

		Ok(())
	}

	pub fn execute(&mut self) -> Result<(), Box<dyn std::error::Error>> {
		self.spike.execute().unwrap();

		Ok(())
	}

	pub fn test(&mut self, config: String) -> Result<(), Box<dyn std::error::Error>> {
		self.dut.test(config).unwrap();

		Ok(())
	}
}
