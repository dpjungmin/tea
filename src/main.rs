use anyhow::Result;
use crossterm::event::EventStream;
use tea::App;

type ExitCode = i32;

#[tokio::main]
async fn tea_main() -> Result<ExitCode> {
    let mut app = App::new(tea::EXAMPLE_TEXT)?;

    let exit_code = app.run(&mut EventStream::new()).await?;

    Ok(exit_code)
}

fn main() -> Result<()> {
    let exit_code = tea_main()?;
    std::process::exit(exit_code);
}
