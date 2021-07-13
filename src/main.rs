#[macro_use]
extern crate serde_json;

mod cli;
mod init_quiz;
mod quiz_utils;
mod ui;

use cli::{Cli, QuizSubcommands};
use ui::Ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::generate_args();

    if let Some(subcommand) = args.commands {
        match subcommand {
            QuizSubcommands::Play(opts) => {
                let quiz = quiz_utils::parse_quiz_file(&opts)?;
                let ui = Ui::new(&quiz);
                ui.play()?
            }
            QuizSubcommands::Init(_) => init_quiz::init()?,
        };
    }

    Ok(())
}
