pub struct Player {
    name: String,
    score: i32
}

impl Player {
    fn add_score(&mut self, additional_score: i32) -> i32 {
        self.score += additional_score;
        self.score
    }
}
