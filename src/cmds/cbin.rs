#![cfg(feature = "cargo-run-bin")]

use super::bin::BinCmd;

#[derive(Debug)]
pub struct CBinCmd {
    pub cbin: String,
    pub args: Vec<String>,
}
impl CBinCmd {
    pub(crate) fn run(self) -> anyhow::Result<()> {
        BinCmd {
            args: [format!("cargo-{}", self.cbin)]
                .into_iter()
                .chain(self.args)
                .collect(),
        }
        .run()
    }
}
#[cfg(feature = "argh")]
impl argh::SubCommand for CBinCmd {
    const COMMAND: &'static argh::CommandInfo = &argh::CommandInfo {
        name: "cbin",
        description: "XTask bin shortcut for executing cargo-* commands",
    };
}
#[cfg(feature = "argh")]
impl argh::FromArgs for CBinCmd {
    fn from_args(_command_name: &[&str], args: &[&str]) -> Result<Self, argh::EarlyExit> {
        if args.is_empty() {
            return Err(argh::EarlyExit {
                output: "Expected a cargo subcommand.".to_string(),
                status: Err(()),
            });
        }
        Ok(Self {
            cbin: args[0].to_string(),
            args: args[1..].iter().map(|&a| a.into()).collect(),
        })
    }
}
