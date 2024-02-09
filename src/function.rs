use chrono::{DateTime, NaiveDate};
use regex::Regex;

pub fn controller_check_and_reformate_date(start_date : String, end_date: String){

    check_date_format_launcher(&start_date, &end_date);

    let start_date = reformate_date(start_date.as_str());
    let end_date = reformate_date(end_date.as_str());


    println!("The start date is :{:?}", start_date);
    println!("The end date is :{:?}", end_date);

}

fn check_date_format_launcher(start_date : &String, end_date: &String) {
    if !check_date_format(&start_date) || !check_date_format(&end_date) {
        println!("The date is not in the correct format! (YYYY-MM-DD) or (YYYY/MM/DD) or (YYYY.MM.DD)");
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

fn reformate_date(date: &str) -> DateTime<chrono::Utc>  {
    let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
    let date = date.and_hms_opt(0, 0, 0);
    let date = DateTime::from_naive_utc_and_offset(date.unwrap(), chrono::Utc);
    date
}