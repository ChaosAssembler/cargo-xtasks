#![cfg(feature = "build-web")]

use std::{fs, path::Path, process::Command};

use anyhow::Context;
use cargo_toml::Manifest;
use wasm_bindgen_cli_support::Bindgen;

use crate::{cargo_metadata::CargoMetadata, command_ext::CommandExt, fs::copy_dir_files_filtered};

const STATIC_WEB_FILE_EXTENSIONS: &[&str] = &["html"];

/// Build for the Web
#[derive(Debug)]
#[cfg_attr(feature = "argh", derive(argh::FromArgs))]
#[cfg_attr(feature = "argh", argh(subcommand, name = "build-web"))]
#[non_exhaustive]
pub struct BuildWeb {
    /// build in release mode
    #[cfg_attr(feature = "argh", argh(switch))]
    pub release: bool,
}
impl BuildWeb {
    pub(crate) fn run(self) -> anyhow::Result<()> {
        // CARGO_MANIFEST_PATH can't be used here, since it always contains the path of the xtask manifest
        let manifest_path = Command::new_cargo()
            .args(["locate-project", "--message-format", "plain"])
            .string_output()?;
        let manifest_path = Path::new(
            manifest_path
                .strip_suffix('\n')
                .context("expected cargo to generate a newline at the end")?,
        );

        let manifest = Manifest::from_path(manifest_path).with_context(|| {
            format!("failed to parse manifest at `{}`", manifest_path.display())
        })?;
        let package = manifest.package.with_context(|| {
            format!(
                "`{}` does not describe a package, this is not supported",
                manifest_path.display()
            )
        })?;
        let bin_name = package.default_run.unwrap_or(package.name);

        let cargo_metadata: CargoMetadata = serde_json::from_slice(
            &Command::new_cargo()
                .args([
                    "metadata",
                    "--no-deps",
                    "-q",
                    "--color",
                    "never",
                    "--format-version",
                    "1",
                ])
                .bytes_output()?,
        )?;
        let target_dir = Path::new(&*cargo_metadata.target_directory);

        let mut build_cmd = Command::new_cargo();
        build_cmd.args([
            "build",
            "--target",
            "wasm32-unknown-unknown",
            "--bin",
            &bin_name,
        ]);
        if self.release {
            build_cmd.arg("--release");
        }
        build_cmd.execute()?;

        let bin_path = target_dir
            .join("wasm32-unknown-unknown")
            .join(if self.release { "release" } else { "debug" })
            .join(&bin_name)
            .with_extension("wasm");
        let target_web_path = target_dir.join("web");

        fs::remove_dir_all(&target_web_path)?;

        Bindgen::new()
            .input_path(bin_path)
            .web(true)?
            .typescript(false)
            .out_name(&bin_name)
            .generate(&target_web_path)?;

        let static_files_path = manifest_path
            .ancestors()
            .nth(1)
            .unwrap()
            .join("src")
            .join("public");

        if static_files_path.exists() {
            copy_dir_files_filtered(static_files_path, target_web_path, |fp| {
                if let Some(ext) = fp.extension() {
                    STATIC_WEB_FILE_EXTENSIONS.iter().any(|&e| e == ext)
                } else {
                    false
                }
            })?;
        } else {
            eprintln!(
                "Warning: directory `{}` does not exist",
                static_files_path.display()
            )
        }

        Ok(())
    }
}
