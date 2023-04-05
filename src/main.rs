use anyhow::{Context, Result};
use tea::App;

type ExitCode = i32;

#[tokio::main]
async fn tea_main() -> Result<ExitCode> {
    // Let's just make it display something for now!
    App::default().run().context("Noooo!!")?;

    Ok(0)
}

fn main() -> Result<()> {
    std::process::exit(tea_main()?);
}
