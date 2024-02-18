mod sim;
mod dut;
mod spike;

use clap::Parser;
use sim::Sim;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	#[arg(short, long)]
	elf_file: String,

	#[arg(short, long)]
	fst_file: String,
}

fn main() {
	let args = Args::parse();

	let mut sim = Sim::new(1usize << 32, &args.fst_file);

	sim.init(&args.elf_file).unwrap();

	loop {
		sim.execute().unwrap();
	}
}
