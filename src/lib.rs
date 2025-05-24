#[cfg(feature = "build-web")]
pub use cmds::build_web;
#[cfg(feature = "run-web")]
pub use cmds::run_web;
#[cfg(feature = "cargo-run-bin")]
pub use cmds::{bin, cbin};

#[cfg(feature = "cargo-run-bin")]
use self::{bin::BinCmd, cbin::CBinCmd};
#[cfg(feature = "build-web")]
use build_web::BuildWeb;
#[cfg(feature = "run-web")]
use run_web::RunWeb;

mod cargo_metadata;
mod cmds;
mod command_ext;
mod fs;

#[derive(Debug)]
#[cfg_attr(feature = "argh", derive(argh::FromArgs))]
#[cfg_attr(feature = "argh", argh(subcommand))]
#[non_exhaustive]
pub enum SubCmd {
    #[cfg(feature = "cargo-run-bin")]
    Bin(BinCmd),
    #[cfg(feature = "cargo-run-bin")]
    CBin(CBinCmd),
    #[cfg(feature = "build-web")]
    BuildWeb(BuildWeb),
    #[cfg(feature = "run-web")]
    RunWeb(RunWeb),
}

/// Cargo XTask
#[derive(Debug)]
#[cfg_attr(feature = "argh", derive(argh::FromArgs))]
pub struct Cli {
    #[cfg_attr(feature = "argh", argh(subcommand))]
    pub cmd: SubCmd,
}

#[cfg(feature = "argh")]
pub fn run() -> anyhow::Result<()> {
    run_with_args(argh::from_env())
}

pub fn run_with_args(args: Cli) -> anyhow::Result<()> {
    match args.cmd {
        #[cfg(feature = "cargo-run-bin")]
        SubCmd::Bin(bincmd) => bincmd.run(),
        #[cfg(feature = "cargo-run-bin")]
        SubCmd::CBin(cbincmd) => cbincmd.run(),
        #[cfg(feature = "build-web")]
        SubCmd::BuildWeb(build_web_cmd) => build_web_cmd.run(),
        #[cfg(feature = "run-web")]
        SubCmd::RunWeb(run_web_cmd) => run_web_cmd.run(),
    }
}
