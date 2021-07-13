use crate::cli::PlayOpts;
use dirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonQuiz {
    pub name: String,
    pub level: String,
    pub number_of_questions: u8,
    pub questions: Vec<Question>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Question {
    pub statement: String,
    pub options: HashMap<String, String>,
    pub answer: String,
}

pub fn parse_quiz_file(opts: &PlayOpts) -> Result<JsonQuiz, Box<dyn std::error::Error>> {
    let home = dirs::home_dir().unwrap();

    let data = match &opts.custom {
        true => fs::read_to_string(&opts.quiz)?,
        false => {
            let file_to_read = Path::new(&home).join(format!(".quiz/{}.json", &opts.quiz));
            fs::read_to_string(&file_to_read)?
        }
    };

    let quiz: JsonQuiz = serde_json::from_str(&data)?;

    Ok(quiz)
}
