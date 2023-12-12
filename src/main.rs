pub mod cli;
pub mod editor;
pub mod ui;

use clap::Parser;
use cli::Args;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args {
        Args { filename } if filename.is_some() => {
            let filename = filename.unwrap();
            editor::Editor::new(filename)?.run();
            Ok(())
        }
        _ => Ok(()),
    }
}
