mod request;
extern crate argparse_rs;


use std::env::args;
use argparse_rs::{ArgParser, ArgType, hashmap_parser, vec_parser};
use std::collections::HashMap;
use request::request_to_all;
use error_chain::error_chain;


error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        UrlParseError(url::ParseError);
        JsonError(serde_json::Error);
    }
}

fn main() {

    // if let Err(ref e) = request_to_all("2023-01-01T00:00:00.000", "2023-02-20T00:00:00.000") {
    //     use error_chain::ChainedError;
    //     println!("Error: {}", e.display_chain());
    //     std::process::exit(1);
    // }

    let mut parser = ArgParser::new("argparse".into());

    parser.add_opt("Name", None, 'n', true, "provide name", ArgType::Option);
    parser.add_opt("Age", None, 'a', true, "provide age", ArgType::Option);

    // read the arguments and print arguments
    let args: Vec<String> = args().collect();

    let args = args.iter().skip(1);

    let mut args = parser.parse(args).unwrap();

    // only the first argument is the name of the program

    println!("{:?}", args);



    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");

    // println!("pattern: {:?}, path: {:?}", pattern, path)


}

