#![cfg(feature = "run-web")]

use std::path::Path;
#[cfg(feature = "webbrowser")]
use std::{io, net::TcpStream, thread, time::Duration};

use super::{bin::BinCmd, build_web::BuildWeb};

/// Build and Run Web-App
#[derive(Debug)]
#[cfg_attr(feature = "argh", derive(argh::FromArgs))]
#[cfg_attr(feature = "argh", argh(subcommand, name = "run-web"))]
#[non_exhaustive]
pub struct RunWeb {
    /// build in release mode
    #[cfg_attr(feature = "argh", argh(switch))]
    pub release: bool,
    /// open webbrowser
    #[cfg(feature = "webbrowser")]
    #[cfg_attr(feature = "argh", argh(switch))]
    pub open: bool,
}
impl RunWeb {
    pub(crate) fn run(self) -> anyhow::Result<()> {
        BuildWeb {
            release: self.release,
        }
        .run()?;
        #[cfg(feature = "webbrowser")]
        if self.open {
            thread::spawn(|| {
                while let Err(err) = TcpStream::connect("127.0.0.1:8000") {
                    if err.kind() != io::ErrorKind::ConnectionRefused {
                        break;
                    }
                    thread::sleep(Duration::from_secs(1));
                }

                webbrowser::open("http://127.0.0.1:8000/")
            });
        }
        BinCmd {
            args: [
                "simple-http-server",
                Path::new("target").join("web").to_str().unwrap(),
                "-c",
                "wasm,html,js",
                "-i",
                "--coep",
                "--coop",
                "--nocache",
                "--ip",
                "127.0.0.1",
                "--port",
                "8000",
            ]
            .map(&str::to_string)
            .to_vec(),
        }
        .run()?;
        Ok(())
    }
}
