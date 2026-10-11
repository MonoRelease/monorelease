#[derive(clap::Parser, Debug, Clone)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum Command {
    /// Prints the version of MonoRelease.
    Version,
    /// Initialise MonoRelease in the current directory.
    Init,
    /// Train commands.
    Trains,
    /// Change commands.
    Changes,
}

pub struct Cli;

impl Cli {
    pub fn run(args: Args) {
        todo!()
    }
}
