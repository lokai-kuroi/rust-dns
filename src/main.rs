use std::process;
use clap::Parser;
use rust_dns::packet_buffer::BytePacketBuffer;

#[derive(Parser, Debug)]
struct CliArgs {
    #[arg(short, long, default_value_t = String::new())]
    custom_hostnames_path: String
}

fn main() {
    human_panic::setup_panic!();
    let args: CliArgs = CliArgs::parse();

    eprintln!("Booting the system!"); // TODO Into Logging Crate

    process::exit(exitcode::OK);
}
