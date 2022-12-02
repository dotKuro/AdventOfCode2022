use crate::day02::common;
use std::collections::HashMap;
use std::fs;
use thiserror::Error;

#[derive(Copy, Clone)]
struct Strategy {
    opponents_choice: common::RPSOption,
    own_choice: common::RPSOption,
}

#[derive(Error, Debug)]
enum ParseStrategyError {
    #[error("strategy line has the wrong amount of characters: {0}")]
    InvalidLineLength(String),
    #[error("strategy line has invalid character for opponents choice: {0}")]
    InvalidOpponentsChoice(String),
    #[error("strategy line has invalid character for own choice: {0}")]
    InvalidOwnChoice(String),
}

fn parse_strategy(strategy: &str) -> Result<Strategy, ParseStrategyError> {
    let opponents_choices = HashMap::from([
        ('A', common::RPSOption::Rock),
        ('B', common::RPSOption::Paper),
        ('C', common::RPSOption::Scissors),
    ]);
    let own_choices = HashMap::from([
        ('X', common::RPSOption::Rock),
        ('Y', common::RPSOption::Paper),
        ('Z', common::RPSOption::Scissors),
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
    let own_choice = chars.get(2).and_then(|char| own_choices.get(char));
    if own_choice.is_none() {
        return Err(ParseStrategyError::InvalidOwnChoice(strategy.to_string()));
    }

    return Ok(Strategy {
        opponents_choice: opponents_choice.unwrap().clone(),
        own_choice: own_choice.unwrap().clone(),
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

fn determine_match_outcome(strategy: &Strategy) -> common::MatchOutcome {
    match (strategy.own_choice, strategy.opponents_choice) {
        (common::RPSOption::Rock, common::RPSOption::Rock) => common::MatchOutcome::Draw,
        (common::RPSOption::Rock, common::RPSOption::Paper) => common::MatchOutcome::Lose,
        (common::RPSOption::Rock, common::RPSOption::Scissors) => common::MatchOutcome::Win,
        (common::RPSOption::Paper, common::RPSOption::Rock) => common::MatchOutcome::Win,
        (common::RPSOption::Paper, common::RPSOption::Paper) => common::MatchOutcome::Draw,
        (common::RPSOption::Paper, common::RPSOption::Scissors) => common::MatchOutcome::Lose,
        (common::RPSOption::Scissors, common::RPSOption::Rock) => common::MatchOutcome::Lose,
        (common::RPSOption::Scissors, common::RPSOption::Paper) => common::MatchOutcome::Win,
        (common::RPSOption::Scissors, common::RPSOption::Scissors) => common::MatchOutcome::Draw,
    }
}

pub fn task1(input_file_path: &str) -> i32 {
    let strategies = get_strategies_from_input(input_file_path);

    strategies
        .iter()
        .map(|strategy| {
            common::option_to_score(strategy.own_choice)
                + common::match_outcome_to_score(determine_match_outcome(strategy))
        })
        .sum()
}
