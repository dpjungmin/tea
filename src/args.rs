use std::path::PathBuf;

use anyhow::{bail, Result};

#[derive(Debug, Default)]
pub struct Args {
    pub display_help: bool,
    pub display_version: bool,
    pub files: Vec<PathBuf>,
}

impl Args {
    pub fn parse() -> Result<Self> {
        let mut args = Self::default();
        let mut argv = std::env::args().skip(1).peekable();

        for arg in argv.by_ref() {
            match arg.as_str() {
                "-h" | "--help" => args.display_help = true,
                "-v" | "-V" | "--version" => args.display_version = true,
                arg if arg.starts_with("--") => bail!("unexpected argument: {}", arg),
                arg => args.files.push(PathBuf::from(arg)),
            }
        }

        for arg in argv {
            args.files.push(PathBuf::from(arg));
        }

        Ok(args)
    }
}
