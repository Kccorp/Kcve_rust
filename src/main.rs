mod unit_test;
mod request;
mod function;

use request::request_to_all;
use function::controller_check_and_refactor_date;

use regex::Regex;
use error_chain::error_chain;
use clap::{Parser, Subcommand};
use chrono::{Local, DateTime, TimeZone};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        UrlParseError(url::ParseError);
        JsonError(serde_json::Error);
    }
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    /// The date to start the search
    #[arg(short, long = "start")]
    // #[arg(short, long = "start", default_value = "2023-01-01T00:00:00.000")]
    start_date: String,

    /// The date to end the search
    #[arg(short, long = "end")]
    end_date: String,
}




fn main() {

    // if let Err(ref e) = request_to_all("2023-01-01T00:00:00.000", "2023-02-20T00:00:00.000") {
    //     use error_chain::ChainedError;
    //     println!("Error: {}", e.display_chain());
    //     std::process::exit(1);
    // }
    let args = Args::parse();


    let vec_dates = controller_check_and_refactor_date(args.start_date, args.end_date);

    println!("The start date is :{:?}", vec_dates[0]);
    println!("The end date is :{:?}", vec_dates[1]);

    if let Err(ref e) = request_to_all(vec_dates[0].as_str(), vec_dates[1].as_str()) {
        use error_chain::ChainedError;
        println!("Error: {}", e.display_chain());
        std::process::exit(1);
    }

}

