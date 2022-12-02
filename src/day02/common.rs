#[derive(Copy, Clone)]
pub enum RPSOption {
    Rock,
    Paper,
    Scissors,
}

pub enum MatchOutcome {
    Win,
    Draw,
    Lose,
}

pub fn option_to_score(option: RPSOption) -> i32 {
    match option {
        RPSOption::Rock => 1,
        RPSOption::Paper => 2,
        RPSOption::Scissors => 3,
    }
}

pub fn match_outcome_to_score(outcome: MatchOutcome) -> i32 {
    match outcome {
        MatchOutcome::Win => 6,
        MatchOutcome::Draw => 3,
        MatchOutcome::Lose => 0,
    }
}
