mod dut;
mod difftest;
mod spike;

use clap::Parser;
use difftest::Difftest;
use tracing::{info, trace, Level};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	#[arg(short, long)]
	elf_file: String,

	#[arg(short, long)]
	fst_file: String,

	#[arg(short, long)]
	config: String,
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

	info!("Hello, world!");
	let args = Args::parse();

	let mut diff = Difftest::new(1usize << 32, &args.fst_file);

	diff.init(&args.elf_file).unwrap();

	(1..10).for_each(|_| {
		diff.execute().unwrap();
	});

	diff.test(args.config).unwrap();

	Ok(())
}
