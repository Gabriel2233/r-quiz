use dirs;

use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

pub fn init() -> Result<(), std::io::Error> {
    let path = setup_quiz_dir()?;
    let mut filename = String::new();
    io::stdin().read_line(&mut filename)?;
    let filename = filename.trim();
    let file_path = format!("{}/{}.json", path, filename);
    let mut file = fs::File::create(file_path)?;

    let json = json!({
        "name": "",
        "level": "",
        "number_of_questions": 0,
        "questions": []
    });

    file.write_all(serde_json::to_string_pretty(&json).unwrap().as_bytes())?;

    Ok(())
}

fn setup_quiz_dir() -> Result<String, std::io::Error> {
    let home = dirs::home_dir().unwrap();
    let path = format!("{}/{}", home.to_str().unwrap(), ".quiz");

    if Path::new(&path).exists() {
        return Ok(path);
    }

    fs::create_dir_all(&path)?;

    Ok(path)
}
