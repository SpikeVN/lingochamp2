use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};
use std::fs;

use crate::logging;

#[derive(Serialize, Deserialize, Clone)]
pub struct Question {
    prompt: String,
    key: String,
    score: i32,
    choices: Option<Vec<String>>,
    score_false: Option<i32>,
    explaination: Option<String>,
}

pub struct QuestionBank {
    question_storage: Option<Vec<Question>>,
}

impl QuestionBank {
    pub const fn new() -> QuestionBank {
        QuestionBank {
            question_storage: None,
        }
    }

    /// Gets the question with the specified ID.
    /// Will panic if no questions with the specified ID exists.
    pub fn get_question(&self, question_id: i32) -> &Question {
        match &self.question_storage {
            Some(qbank) => qbank
                .get(question_id as usize)
                .expect(logging::error_str("No question was found under the given ID.").as_str()),
            None => {
                panic!("{}", logging::error_str("Uninitialized question bank."));
            }
        }
    }

    /// Gets a random question.
    /// Panics if there are no questions in the bank.
    pub fn random_question(&self) -> &Question {
        match &self.question_storage {
            Some(qbank) => {
                if qbank.len() == 0 {
                    panic!(
                        "{}",
                        logging::error_str("Cannot get random question in an empty question bank.")
                    )
                }
                let mut rng = thread_rng();
                qbank
                    .choose(&mut rng)
                    .expect(logging::error_str("Cannot choose random question.").as_str())
            }
            None => panic!("{}", logging::error_str("Uninitialized question bank.")),
        }
    }

    /// Gets a specified number of unique random questions.
    /// Panics if there are no questions in the bank.
    pub fn random_n_questions(&self, n: usize) -> Vec<&Question> {
        match &self.question_storage {
            Some(qbank) => {
                if qbank.len() == 0 {
                    panic!(
                        "{}",
                        logging::error_str(
                            "Cannot get random n question in an empty question bank."
                        )
                    )
                }
                let mut rng = thread_rng();
                qbank.choose_multiple(&mut rng, n).collect()
            }
            None => {
                panic!("{}", logging::error_str("Uninitialized question bank."));
            }
        }
    }

    /// Load questions from a JSON file.
    /// It should have the following structure:
    /// ```json
    /// [
    ///   {
    ///     "prompt": "What is one plus one?",
    ///     "key": "Two",
    ///     "score": 100,
    ///     // Optional properties
    ///     "choices": ["One", "Two", "Three", "One Million Five Hundred and Fifty Two Thousand Seven Hundred and Three"],
    ///     "score_false": -5,
    ///     "explaination": "bruh"
    ///   },
    ///   ...
    /// ]
    /// ```
    pub fn load_questions(&mut self, filepath: &str) {
        self.question_storage = serde_json::from_str(
            fs::read_to_string(filepath)
                .expect(logging::error_str("Unable to read file").as_str())
                .as_str(),
        )
        .expect(logging::error_str("Invalid JSON structure.").as_str());
    }
}
