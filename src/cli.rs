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
    /// Change commands.
    Change,
    /// Wave commands.
    Wave,
    /// Project commands.
    Project {
        #[command(subcommand)]
        command: ProjectCommand,
    },
    /// Generate changelogs.
    Changelog {
        #[command(subcommand)]
        command: ChangelogCommand,
    },
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum ProjectCommand {
    Bump {
        project: String,

        /// Performs a dry-run.
        #[arg(long)]
        dry: bool,
    },
    /// Revert the last bump of a project.
    Revert { project: String },
}

pub type Wave = u32;

#[derive(clap::Subcommand, Debug, Clone)]
pub enum ChangelogCommand {
    /// Generate the changelog for a single exact version of a project.
    Exact {
        /// The project ID.
        project: String,
        /// The project version to generate the changelog for.
        version: String,
    },
    /// Generate the changelog across a range of versions of a project.
    Range {
        project: String,
        /// The starting point of the changelog (inclusive).
        /// Can either be a version of the project or a wave.
        ///
        /// Examples:
        /// - version:1.2.3
        /// - wave:42
        from: String,
        /// The end point of the changelog (inclusive).
        /// Can either be a version of the project or a wave.
        ///
        /// Examples:
        /// - version:1.2.3
        /// - wave:42
        to: String,
    },
    /// Generate the full changelog for a project.
    Full { project: String },
}

pub struct Cli;

impl Cli {
    pub fn run(args: Args) {
        todo!()
    }
}
