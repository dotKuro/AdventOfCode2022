use crate::day02::common;
use std::collections::HashMap;
use std::fs;
use thiserror::Error;

#[derive(Copy, Clone)]
struct Strategy {
    opponents_choice: common::RPSOption,
    match_outcome: common::MatchOutcome,
}

#[derive(Error, Debug)]
enum ParseStrategyError {
    #[error("strategy line has the wrong amount of characters: {0}")]
    InvalidLineLength(String),
    #[error("strategy line has invalid character for opponents choice: {0}")]
    InvalidOpponentsChoice(String),
    #[error("strategy line has invalid character for match outcome: {0}")]
    InvalidMatchOutcome(String),
}

fn parse_strategy(strategy: &str) -> Result<Strategy, ParseStrategyError> {
    let opponents_choices = HashMap::from([
        ('A', common::RPSOption::Rock),
        ('B', common::RPSOption::Paper),
        ('C', common::RPSOption::Scissors),
    ]);
    let match_outcomes = HashMap::from([
        ('X', common::MatchOutcome::Lose),
        ('Y', common::MatchOutcome::Draw),
        ('Z', common::MatchOutcome::Win),
    ]);

    let chars: Vec<char> = strategy.chars().collect();

    if chars.len() != 3 {
        return Err(ParseStrategyError::InvalidLineLength(strategy.to_string()));
    }

    let opponents_choice = chars.get(0).and_then(|char| opponents_choices.get(char));
    if opponents_choice.is_none() {
        return Err(ParseStrategyError::InvalidOpponentsChoice(
            strategy.to_string(),
        ));
    }
    let match_outcome = chars.get(2).and_then(|char| match_outcomes.get(char));
    if match_outcome.is_none() {
        return Err(ParseStrategyError::InvalidMatchOutcome(
            strategy.to_string(),
        ));
    }

    return Ok(Strategy {
        opponents_choice: opponents_choice.unwrap().clone(),
        match_outcome: match_outcome.unwrap().clone(),
    });
}

fn get_strategies_from_input(input_file_path: &str) -> Vec<Strategy> {
    let content = fs::read_to_string(input_file_path).expect("failed to read input file");

    let lines: Vec<&str> = content.split("\n").collect();
    let mut strategies: Vec<Strategy> = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let strategy = parse_strategy(line).unwrap();
        strategies.push(strategy);
    }

    return strategies;
}

fn determine_own_choice(strategy: &Strategy) -> common::RPSOption {
    match (strategy.match_outcome, strategy.opponents_choice) {
        (common::MatchOutcome::Lose, common::RPSOption::Rock) => common::RPSOption::Scissors,
        (common::MatchOutcome::Lose, common::RPSOption::Paper) => common::RPSOption::Rock,
        (common::MatchOutcome::Lose, common::RPSOption::Scissors) => common::RPSOption::Paper,
        (common::MatchOutcome::Draw, common::RPSOption::Rock) => common::RPSOption::Rock,
        (common::MatchOutcome::Draw, common::RPSOption::Paper) => common::RPSOption::Paper,
        (common::MatchOutcome::Draw, common::RPSOption::Scissors) => common::RPSOption::Scissors,
        (common::MatchOutcome::Win, common::RPSOption::Rock) => common::RPSOption::Paper,
        (common::MatchOutcome::Win, common::RPSOption::Paper) => common::RPSOption::Scissors,
        (common::MatchOutcome::Win, common::RPSOption::Scissors) => common::RPSOption::Rock,
    }
}

pub fn task2(input_file_path: &str) -> i32 {
    let strategies = get_strategies_from_input(input_file_path);

    strategies
        .iter()
        .map(|strategy| {
            common::option_to_score(determine_own_choice(strategy))
                + common::match_outcome_to_score(strategy.match_outcome)
        })
        .sum()
}
