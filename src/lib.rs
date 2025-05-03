#[cfg(feature = "cargo-run-bin")]
use bin::BinCmd;

mod bin;

#[derive(Debug)]
#[cfg_attr(feature = "argh", derive(argh::FromArgs))]
#[cfg_attr(feature = "argh", argh(subcommand))]
#[non_exhaustive]
pub enum SubCmd {
    #[cfg(feature = "cargo-run-bin")]
    Bin(BinCmd),
}

#[derive(Debug)]
#[cfg_attr(feature = "argh", derive(argh::FromArgs))]
/// Cargo XTask
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
    }
}
