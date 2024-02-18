mod dut;
mod sim;
mod spike;

use clap::Parser;
use sim::Sim;
use tracing::{info, trace, Level};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	#[arg(short, long)]
	elf_file: String,

	#[arg(short, long)]
	fst_file: String,
}

fn main() -> anyhow::Result<()> {
	let global_logger = FmtSubscriber::builder()
		.with_env_filter(EnvFilter::from_default_env())
		.with_max_level(Level::TRACE)
		.without_time()
		.with_target(false)
		.compact()
		.finish();
	tracing::subscriber::set_global_default(global_logger)
		.expect("internal error: fail to setup log subscriber");

	let args = Args::parse();

	let mut sim = Sim::new(1usize << 32, &args.fst_file);

	sim.init(&args.elf_file).unwrap();

	(1..10).for_each(|_| {
		sim.execute().unwrap();
	});

	sim.test().unwrap();

	Ok(())
}
