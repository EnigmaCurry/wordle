use clap::{Parser, Subcommand};
use time::macros::format_description;

#[derive(Parser)]
#[clap(name = "wordle", author, version)]
pub struct App {
    #[clap(subcommand)]
    pub game_mode: Option<GameMode>,
}

#[derive(Subcommand)]
pub enum GameMode {
    /// Play with a custom word
    Custom(Custom),
    /// Play the specified day's wordle
    Day(Day),
    /// Play the specified date's wordle
    Date(Date),
    /// Play a random day
    Random,
}

#[derive(Parser)]
pub struct Custom {
    pub word: String,
}

#[derive(Parser)]
pub struct Day {
    pub day: usize,
}

#[derive(Parser)]
pub struct Date {
    #[clap(parse(try_from_str = parse_date))]
    /// year-month-day
    pub date: time::Date,
}

fn parse_date(input: &str) -> Result<time::Date, time::error::Parse> {
    let description = format_description!("[year]-[month]-[day]");
    time::Date::parse(input, description)
}
