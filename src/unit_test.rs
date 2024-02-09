use std::io::Read;

/// This function makes a request to the NVD API to get all the CVEs that were published between the start and end date
#[test]
pub fn request_to_all_test() -> crate::Result<()> {

    let start_date = "2023-01-01T00:00:00.000";
    let end_date = "2023-02-20T00:00:00.000";

    let url_test = "https://services.nvd.nist.gov/rest/json/cves/2.0";

    let params =[
        ("pubStartDate", start_date),
        ("pubEndDate", end_date),
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

    // return OK if everything is fine
    Ok(())
}
