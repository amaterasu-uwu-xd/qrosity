mod cli;
mod commands;
mod shared;

use clap::Parser;
use cli::Cli;
use commands::handle_commands;

fn main() {
    let cli = Cli::parse();
    handle_commands(cli);
}
