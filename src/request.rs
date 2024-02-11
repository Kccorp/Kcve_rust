use std::io::Read;
use crate::parse;

/// This struct is used to store the parameters to make the request to the NVD API
struct Params {
    pub_start_date: String,
    pub_end_date: String,
    source_identifier: String,
    keyword_search: Option<String>,
}

impl Params {
    /// This function creates a new Params struct with the required parameters to make the request to the NVD API
    fn new(pub_start_date: String, pub_end_date: String, source_identifier: String, keyword_search: Option<String>) -> Params {
        Params {
            pub_start_date,
            pub_end_date,
            source_identifier,
            keyword_search,
        }
    }
}


/// This trait is used to iterate over the Params struct and return a tuple with the key and value of the struct
/// It's used to prepare the request to the NVD API and to make the GET request. It necessary to use iter() and into_iter()
/// to iterate over the struct required by parse_with_params from reqwest
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

/// This function is the controller for the request to the NVD API. It calls the request_to_all function if the keyword is None
/// and the request_to_keyword function if the keyword is Some. It then calls the parse_cve_json function from the parse module
/// to parse the response from the NVD API
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

/// This function prepares the request to the NVD API to get all the CVEs that were published between
/// the start and end date. It calls the http_get_request function to make the request and returns the
/// body of the response
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

/// This function prepares the request to the NVD API to get all the CVEs that were published between the start and end date
/// and that contains the keyword provided. It calls the http_get_request function to make the request and returns the body of the response
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

/// This function makes a GET request to the NVD API and returns the body of the response if the request is successful
/// If the request is not successful, the function prints the error and exits the program.
/// The function take the Params struct as a parameter and returns a Result with the body of the response as a String
/// or an error
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



