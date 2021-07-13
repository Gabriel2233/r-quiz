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
    #[structopt(name = "init")]
    Init(InitOpts),
    #[structopt(name = "play")]
    Play(PlayOpts),
}

#[derive(StructOpt, Debug)]
pub struct InitOpts {}

#[derive(StructOpt, Debug)]
pub struct PlayOpts {
    #[structopt(long, short)]
    pub custom: bool,
    #[structopt(name = "quiz-name")]
    pub quiz: String,
}
