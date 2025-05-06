#![cfg(feature = "build-web")]

use std::{env, ffi::OsString, iter, process::Command};

use anyhow::Context;

pub(crate) trait CommandExt {
    fn new_cargo() -> Self;
    fn args_os_str(&self) -> OsString;
    fn execute(&mut self) -> anyhow::Result<()>;
    fn bytes_output(&mut self) -> anyhow::Result<Vec<u8>>;
    fn string_output(&mut self) -> anyhow::Result<String>;
}
impl CommandExt for Command {
    fn new_cargo() -> Self {
        Self::new(env::var_os("CARGO").as_deref().unwrap_or("cargo".as_ref()))
    }
    fn args_os_str(&self) -> OsString {
        iter::once(self.get_program())
            .chain(self.get_args())
            .collect::<Vec<_>>()
            .join(" ".as_ref())
    }
    fn execute(&mut self) -> anyhow::Result<()> {
        let args = self.args_os_str();
        self.status()
            .with_context(|| format!("failed to execute command `{}`", args.display()))?
            .success()
            .then_some(())
            .with_context(|| {
                format!(
                    "command `{}` terminated with non-zero exit code",
                    args.display()
                )
            })
    }
    fn bytes_output(&mut self) -> anyhow::Result<Vec<u8>> {
        let output = self
            .output()
            .with_context(|| format!("failed to execute `{}`", self.args_os_str().display()))?;
        output.status.success().then_some(()).with_context(|| {
            format!(
                "command exited with non-zero: `{}`",
                self.args_os_str().display()
            )
        })?;
        Ok(output.stdout)
    }
    fn string_output(&mut self) -> anyhow::Result<String> {
        String::from_utf8(self.bytes_output()?).with_context(|| {
            format!(
                "unexpected non-utf8 output in stdout of `{}`",
                self.args_os_str().display()
            )
        })
    }
}
