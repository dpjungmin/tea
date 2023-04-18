use tea::{App, Args};

use anyhow::Result;
use crossterm::event::EventStream;

fn print_help() {
    println!(
        "{}",
        format_args!(
            "\
{name} {version}
{description}

USAGE:
    {bin} [OPTIONS] [files]...

ARGS:
    <files>...    The input files

OPTIONS:
    -h, --help                   Prints help information
    -v, -V, --version            Prints version information",
            name = env!("CARGO_PKG_NAME"),
            version = env!("CARGO_PKG_VERSION"),
            description = env!("CARGO_PKG_DESCRIPTION"),
            bin = env!("CARGO_BIN_NAME"),
        )
    );
}

fn print_version() {
    println!("{} {}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"));
}

type ExitCode = i32;

#[tokio::main]
async fn tea_main() -> Result<ExitCode> {
    let args = Args::parse()?;

    if args.display_help {
        print_help();
        std::process::exit(0);
    }

    if args.display_version {
        print_version();
        std::process::exit(0);
    }

    let mut app = App::new(args.files.as_ref())?;

    let exit_code = app.run(&mut EventStream::new()).await?;

    Ok(exit_code)
}

fn main() -> Result<()> {
    let exit_code = tea_main()?;
    std::process::exit(exit_code);
}
