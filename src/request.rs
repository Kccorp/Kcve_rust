use std::io::Read;
use crate::parse;

#[derive(Debug)]
struct Params {
    pub_start_date: String,
    pub_end_date: String,
    source_identifier: String,
    keyword_search: Option<String>,
}

impl Params {
    fn new(pub_start_date: String, pub_end_date: String, source_identifier: String, keyword_search: Option<String>) -> Params {
        Params {
            pub_start_date,
            pub_end_date,
            source_identifier,
            keyword_search,
        }
    }
}


// Implémentez IntoIterator pour votre struct
impl IntoIterator for &Params {
    type Item = (&'static str, String);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut params_vec = vec![
            ("pubStartDate", self.pub_start_date.clone()),
            ("pubEndDate", self.pub_end_date.clone()),
            ("sourceIdentifier", self.source_identifier.clone()),
        ];

        if let Some(keywordSearch) = &self.keyword_search {
            params_vec.push(("keywordSearch", keywordSearch.clone()));
        }

        params_vec.into_iter()
    }
}


pub fn request_controller(start_date: &str, end_date: &str, keyword: Option<String>)  {
    return if keyword == None {
        let response = request_to_all(start_date, end_date);
        let response_json : serde_json::Value = response.unwrap();
        parse::parse_cve_json(response_json);
    } else {
        let response = request_to_keyword(start_date, end_date, keyword.unwrap().as_str());
        let response_json : serde_json::Value = response.unwrap();
        parse::parse_cve_json(response_json);
    }


}

/// This function makes a request to the NVD API to get all the CVEs that were published between the start and end date
fn request_to_all(start_date: &str, end_date: &str) -> crate::Result<serde_json::Value> {
    let url = "https://services.nvd.nist.gov/rest/json/cves/2.0";
    let params = Params::new(start_date.to_string(), end_date.to_string(), "nvd@nist.gov".to_string(), None);

    match http_get_request(url, params) {
        Ok(body) => {
            // Handle success with the body
            let body_json: serde_json::Value = serde_json::from_str(&body)?;
            return Ok(body_json);

        }
        Err(e) => {
            // Handle error
            use error_chain::ChainedError;
            println!("Error: {}", e.display_chain());
            std::process::exit(1);
        }
    }
}

fn request_to_keyword(start_date: &str, end_date: &str, keyword: &str) -> crate::Result<serde_json::Value> {
    let url = "https://services.nvd.nist.gov/rest/json/cves/2.0";
    let params = Params::new(start_date.to_string(), end_date.to_string(), "nvd@nist.gov".to_string(), Some(keyword.to_string()));

    match http_get_request(url, params) {
        Ok(body) => {
            // Handle success with the body
            let body_json: serde_json::Value = serde_json::from_str(&body)?;
            return Ok(body_json);

        }
        Err(e) => {
            // Handle error
            use error_chain::ChainedError;
            println!("Error: {}", e.display_chain());
            std::process::exit(1);
        }
    }
}

fn http_get_request(url: &str, params: Params) -> crate::Result<String> {

    let url_test = reqwest::Url::parse_with_params(url, &params)?;
    let mut res = reqwest::blocking::get(url_test)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    if res.status() != 200 {
        println!("Error: {:#?}", res.headers());
        std::process::exit(1);
    }

    // return Ok(body_json) if the function returns a json and no error
    Ok(body)
}



