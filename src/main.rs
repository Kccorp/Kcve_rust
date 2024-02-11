mod unit_test;
mod request;
mod function;
mod parse;
mod cve;

use request::request_controller;
use function::controller_check_and_refactor_date;

use error_chain::error_chain;
use clap::{Parser};



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

    /// The keyword to search
    #[arg(short, long = "keyword")]
    keyword: Option<String>,
}




fn main() {

    // parse args
    let args = Args::parse();

    // check and refactor date
    let vec_dates = controller_check_and_refactor_date(args.start_date, args.end_date);

    // request to the NVD API to get the CVEs between the two dates and with the keyword if it's provided
    request_controller(&vec_dates[0], &vec_dates[1], args.keyword);

}

