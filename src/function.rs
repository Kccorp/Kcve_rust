use chrono::{DateTime, NaiveDate};
use regex::Regex;

pub fn controller_check_and_refactor_date(start_date : String, end_date: String) -> Vec<String> {

    check_date_format_launcher(&start_date, &end_date);

    let start_date = refactor_date_unify_separator(&start_date);
    let end_date = refactor_date_unify_separator(&end_date);

    let start_date = refactor_date_str_to_utc(start_date.as_str());
    let end_date = refactor_date_str_to_utc(end_date.as_str());

    check_order_date(start_date, end_date);

    let vec_of_dates = vec![start_date.to_rfc3339().to_string(), end_date.to_rfc3339().to_string()];
    vec_of_dates
}

fn check_date_format_launcher(start_date : &String, end_date: &String) {
    if !check_date_format(&start_date) || !check_date_format(&end_date) {
        println!("The date is not in the correct format! (YYYY-MM-DD) or (YYYY/MM/DD) or (YYYY.MM.DD)");
        std::process::exit(1);
    }
}

fn check_order_date(start_date: DateTime<chrono::Utc>, end_date: DateTime<chrono::Utc>) {
    if start_date > end_date {
        println!("The start date is greater than the end date!");
        std::process::exit(1);
    }
}

/// This function checks if the date is in the correct format that means YYYY-MM-DD or YYYY/MM/DD
/// or YYYY.MM.DD. It uses the regex crate to check the date format.
fn check_date_format(date: &String) -> bool {
    let re = Regex::new(r"^(19|20)\d\d([- /.])(0[1-9]|1[012])([- /.])(0[1-9]|[12][0-9]|3[01])$").unwrap();
    let checked_date = re.is_match(date.as_str());
    checked_date
}

fn refactor_date_unify_separator(date: &String) -> String {
    let re = Regex::new(r"[- /.]").unwrap();
    let date = re.replace_all(date.as_str(), "-");
    date.to_string()
}

fn refactor_date_str_to_utc(date: &str) -> DateTime<chrono::Utc>  {
    let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
    let date = date.and_hms_opt(0, 0, 0);
    let date = DateTime::from_naive_utc_and_offset(date.unwrap(), chrono::Utc);
    date
}

