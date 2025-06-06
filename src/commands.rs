use std::path::PathBuf;

use log::error;

use clap::{Parser, Subcommand, command, crate_authors};
use clap_verbosity_flag::{InfoLevel, Verbosity};

use crate::shadow;

pub mod bug_report;
pub mod build;
pub mod completions;
pub mod generate;
pub mod generate_iso;
pub mod init;
pub mod login;
pub mod prune;
pub mod switch;
pub mod validate;

pub trait BlueBuildCommand {
    /// Runs the command and returns a result
    /// of the execution
    ///
    /// # Errors
    /// Can return an `anyhow` Error
    fn try_run(&mut self) -> miette::Result<()>;

    /// Runs the command and exits if there is an error.
    fn run(&mut self) {
        if let Err(e) = self.try_run() {
            error!("Failed:\n{e:?}");
            std::process::exit(1);
        }
        std::process::exit(0);
    }
}

#[derive(Parser, Debug)]
#[clap(
    name = "BlueBuild",
    about,
    long_about = None,
    author=crate_authors!(),
    version=shadow::PKG_VERSION,
    long_version=shadow::CLAP_LONG_VERSION,
)]
pub struct BlueBuildArgs {
    #[command(subcommand)]
    pub command: CommandArgs,

    /// The directory to output build logs.
    #[arg(long)]
    pub log_out: Option<PathBuf>,

    #[clap(flatten)]
    pub verbosity: Verbosity<InfoLevel>,
}

#[derive(Debug, Subcommand)]
pub enum CommandArgs {
    /// Build an image from a recipe
    Build(build::BuildCommand),

    /// Generate a Containerfile from a recipe
    #[clap(visible_alias = "template")]
    Generate(generate::GenerateCommand),

    /// Generate an ISO for an image or recipe.
    GenerateIso(generate_iso::GenerateIsoCommand),

    /// Switch your current OS onto the image
    /// being built.
    ///
    /// This will create a tarball of your image at
    /// `/etc/bluebuild/` and invoke `rpm-ostree` to
    /// rebase/upgrade onto the image using `oci-archive`.
    ///
    /// NOTE: This can only be used if you have `rpm-ostree`
    /// installed. This image will not be signed.
    #[command(
        visible_alias("update"),
        visible_alias("upgrade"),
        visible_alias("rebase")
    )]
    Switch(switch::SwitchCommand),

    /// Login to all services used for building.
    Login(login::LoginCommand),

    /// Create a new bluebuild project.
    New(init::NewCommand),

    /// Create a new bluebuild project.
    Init(init::InitCommand),

    /// Validate your recipe file and display
    /// errors to help fix problems.
    Validate(Box<validate::ValidateCommand>),

    /// Clean up cache and images for build drivers.
    Prune(prune::PruneCommand),

    /// Create a pre-populated GitHub issue with information about your configuration
    BugReport(bug_report::BugReportCommand),

    /// Generate shell completions for your shell to stdout
    Completions(completions::CompletionsCommand),
}

#[cfg(test)]
mod test {
    use clap::CommandFactory;

    use super::BlueBuildArgs;

    #[test]
    fn test_cli() {
        BlueBuildArgs::command().debug_assert();
    }
}
