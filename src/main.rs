use clap::Parser;
use std::fs::File;
use std::io::Read;
use xmas_elf::{
    header,
    program::{ProgramHeader, Type},
    ElfFile,
};

extern crate my_elfloader;
use my_elfloader::Spike;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    file: String,
}

pub struct LoadElfResult {
    pub entry_addr: u64,
}

pub struct Sim {
    spike: Spike,
}

impl Sim {
    pub fn new(size: usize) -> Self {
        Self {
            spike: Spike::new(size as u64),
        }
    }

    pub fn load_elf(&mut self, fname: &str) -> Result<LoadElfResult, Box<dyn std::error::Error>> {
        let mut file = File::open(fname)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

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
                        self.spike.ld_elf(addr as u64, size as u64, slice.as_ptr() as *mut u8)?;
                    }
                }
                _ => (),
            }
        }

        Ok(LoadElfResult {
            entry_addr: header.pt2.entry_point() as u64,
        })
    }
}

fn main() {
    let args = Args::parse();

    let mut sim = Sim::new(1usize << 32);

    sim.load_elf(&args.file).unwrap();
}
