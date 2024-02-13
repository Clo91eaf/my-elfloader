use clap::Parser;

mod sim;
use sim::Sim;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();

    let mut sim = Sim::new(1usize << 32);

    sim.load_elf(&args.file).unwrap();
}
