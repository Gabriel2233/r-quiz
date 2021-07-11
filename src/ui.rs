use crate::quiz_utils::{JsonQuiz, Question};
use std::io::{self, Error as IoError, Stdout};
use std::{thread, time};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{self, color, style};

pub struct Ui<'a> {
    quiz: &'a JsonQuiz,
    _stdout: RawTerminal<Stdout>,
}

enum Signal {
    Quit,
    CorrectAnswer,
    WrongAnswer,
    WrongKey,
}

impl<'a> Ui<'a> {
    pub fn new(quiz: &'a JsonQuiz) -> Self {
        Self {
            quiz,
            _stdout: io::stdout().into_raw_mode().unwrap(),
        }
    }

    pub fn play(&self) -> Result<(), IoError> {
        let mut right_answers = 0;
        let mut wrong_answers = 0;

        for question in self.quiz.questions.iter() {
            self.clear_screen();
            print!("{}{}{}\r\n", style::Bold, question.statement, style::Reset);

            for (key, value) in question.options.iter() {
                print!("{}) {}\r\n", key, value);
            }

            loop {
                match self.process_keystroke(&question) {
                    Ok(result) => match result {
                        Signal::CorrectAnswer => right_answers += 1,
                        Signal::WrongAnswer => wrong_answers += 1,
                        Signal::WrongKey => continue,
                        Signal::Quit => {
                            print!("You cancelled the quiz\r\n");
                            return Ok(());
                        }
                    },
                    Err(e) => return Err(e),
                };
                break;
            }
            self.clear_screen();
        }

        print!("Yay! You've reached the end! Here's the result: \r\n");
        print!(
            "{}{} correct answers{}\r\n",
            color::Fg(color::Green),
            right_answers,
            style::Reset
        );
        print!(
            "{}{} wrong answers{}\r\n",
            color::Fg(color::Red),
            wrong_answers,
            style::Reset
        );

        Ok(())
    }

    fn clear_screen(&self) {
        print!("{}", termion::clear::All);
        print!("{}", termion::cursor::Goto(1, 1))
    }

    fn process_keystroke(&self, q: &Question) -> Result<Signal, IoError> {
        let key = self.read_key()?;

        match key {
            Key::Ctrl('q') => Ok(Signal::Quit),
            Key::Char(c) => {
                let contains_typed_key = q.options.contains_key(&c.to_string());

                if !contains_typed_key {
                    print!("This is not a valid alternative!\r\n");
                    return Ok(Signal::WrongKey);
                }

                if c.to_string() == q.answer {
                    print!("Correct!\r\n");
                    thread::sleep(time::Duration::from_millis(500));
                    Ok(Signal::CorrectAnswer)
                } else {
                    print!("Wrong!\r\n");
                    thread::sleep(time::Duration::from_millis(500));
                    Ok(Signal::WrongAnswer)
                }
            }
            _ => {
                print!("This is not a valid alternative!\r\n");
                Ok(Signal::WrongKey)
            }
        }
    }

    fn read_key(&self) -> Result<Key, IoError> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}
