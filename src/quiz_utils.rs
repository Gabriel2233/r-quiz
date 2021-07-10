use crate::cli::PlayOpts;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

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

pub fn parse_quiz_file(opts: PlayOpts) -> Result<JsonQuiz, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(&opts.file)?;

    let quiz: JsonQuiz = serde_json::from_str(&data)?;

    println!("play quiz {:?}", quiz);

    Ok(quiz)
}
