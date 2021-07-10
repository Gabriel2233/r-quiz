use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "quiz", about = "cool")]
pub struct Cli {
    #[structopt(subcommand)]
    pub commands: Option<QuizSubcommands>,
}

impl Cli {
    pub fn generate_args() -> Cli {
        let args = Cli::from_args();
        args
    }
}

#[derive(StructOpt, Debug)]
pub enum QuizSubcommands {
    #[structopt(name = "list")]
    List(ListOpts),
    #[structopt(name = "play")]
    Play(PlayOpts),
}

#[derive(StructOpt, Debug)]
pub struct ListOpts {}

#[derive(StructOpt, Debug)]
pub struct PlayOpts {
    #[structopt(name = "file", parse(from_os_str))]
    pub file: PathBuf,
}
