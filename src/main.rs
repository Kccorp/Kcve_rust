mod unit_test;
mod request;
mod function;
mod parse;
mod cve;

use request::request_controller;
use function::controller_check_and_refactor_date;

use error_chain::error_chain;
use clap::{Parser};
use crate::cve::Cve;


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

    // if let Err(ref e) = request_to_all("2023-01-01T00:00:00.000", "2023-02-20T00:00:00.000") {
    //     use error_chain::ChainedError;
    //     println!("Error: {}", e.display_chain());
    //     std::process::exit(1);
    // }
    let args = Args::parse();


    let vec_dates = controller_check_and_refactor_date(args.start_date, args.end_date);

    let response = request_controller(&vec_dates[0], &vec_dates[1], args.keyword);


    let response_json : serde_json::Value = response.unwrap();
    // println!("The response is :{}", response_json);

    let cve_vec = parse::parse_cve_json(response_json);

    if cve_vec.len() == 0 {
        println!("No CVEs found!");
    } else {
        println!("{:?} CVE found !", cve_vec.len());
        println!("Display all cve found: 1");
        println!("Choose the cve to display: 2");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse::<i32>().unwrap();
        match choice {
            1 => {
                for cve in &cve_vec {
                    Cve::show_cve(&cve);
                }
            },
            2 => {
                for cve in &cve_vec {
                    Cve::show_cve_id(&cve);
                }
                println!("Enter the cve id to display: ");
                let mut cve_id = String::new();
                std::io::stdin().read_line(&mut cve_id).unwrap();
                let cve_id = cve_id.trim().to_string();
                for cve in &cve_vec {
                    if cve.cve_id == cve_id {
                        Cve::show_cve(&cve);
                    }
                }
            },
            _ => {
                println!("Invalid choice");
            }
        }
    }
}

