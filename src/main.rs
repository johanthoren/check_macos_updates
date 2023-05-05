use check_macos_updates::{check_updates, get_output};
use clap::Parser;
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // #[arg(short, long)]
    // critical: u8,
    // #[arg(short, long)]
    // warning: u8,
}

fn main() {
    let _args = Args::parse();

    if !cfg!(target_os = "macos") {
        println!("UNKNOWN - Not running on MacOS");
        process::exit(3);
    }

    let (message, exit_code) = check_updates(get_output());

    println!("{}", message);
    process::exit(exit_code);
}
