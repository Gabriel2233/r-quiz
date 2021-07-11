pub mod cli;
pub mod quiz_utils;

mod ui;
use cli::{Cli, QuizSubcommands};
use ui::Ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::generate_args();

    if let Some(subcommand) = args.commands {
        match subcommand {
            QuizSubcommands::Play(opts) => {
                let quiz = quiz_utils::parse_quiz_file(opts)?;
                println!("{:?}", quiz);
                let ui = Ui::new(&quiz);

                ui.play()?
            }
            QuizSubcommands::List(opts) => println!("{:?}", opts),
        };
    }

    Ok(())
}
