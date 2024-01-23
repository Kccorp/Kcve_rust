use error_chain::error_chain;
use std::io::Read;



error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        UrlParseError(url::ParseError);
        JsonError(serde_json::Error);
    }
}

fn request_to_all() -> Result<()> {
    let url_test = "https://services.nvd.nist.gov/rest/json/cves/2.0";

    let params =[
        ("pubStartDate", "2023-01-01T00:00:00.000"),
        ("pubEndDate", "2023-02-20T00:00:00.000"),
        ("sourceIdentifier", "nvd@nist.gov"),
        ("keywordSearch", "php")
    ];

    let url_test = reqwest::Url::parse_with_params(url_test, &params)?;
    let mut res = reqwest::blocking::get(url_test)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    // let body_json: serde_json::Value = serde_json::from_str(&body)?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{:#?}", body);

    Ok(())
}

fn main() {
    if let Err(ref e) = request_to_all() {
        use error_chain::ChainedError;
        println!("Error: {}", e.display_chain());
    }
}

