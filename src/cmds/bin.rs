#![cfg(feature = "cargo-run-bin")]

use anyhow::Context;

#[derive(Debug)]
pub struct BinCmd {
    pub args: Vec<String>,
}
impl BinCmd {
    pub(crate) fn run(self) -> anyhow::Result<()> {
        let cargo_bin_pkg = cargo_run_bin::metadata::get_binary_packages()?
            .iter()
            .find(|bp| {
                bp.package == "cargo-run-bin" && bp.bin_target.as_deref() == Some("cargo-bin")
            })
            .context("couldn't find cargo-run-bin with cargo-bin binary in workspace.metadata.bin")?
            .to_owned();
        let cargo_bin_path = cargo_run_bin::binary::install(cargo_bin_pkg)?;
        cargo_run_bin::binary::run(cargo_bin_path, self.args)
    }
}
#[cfg(feature = "argh")]
impl argh::SubCommand for BinCmd {
    const COMMAND: &'static argh::CommandInfo = &argh::CommandInfo {
        name: "bin",
        description: "XTask interface for cargo-bin",
    };
}
#[cfg(feature = "argh")]
impl argh::FromArgs for BinCmd {
    fn from_args(_command_name: &[&str], args: &[&str]) -> Result<Self, argh::EarlyExit> {
        Ok(Self {
            args: args.iter().map(|&a| a.into()).collect(),
        })
    }
}
