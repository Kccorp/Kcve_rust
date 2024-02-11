//import cve.rs file
use crate::cve::Cve;

pub fn parse_cve_json(json: serde_json::Value) -> Vec<Cve> {
    let cve_json = json["vulnerabilities"].as_array().unwrap();
    let mut cve_vec = Vec::new();

    for cve in cve_json {
        let cve_id = cve["cve"]["id"].as_str().unwrap().to_string();
        let cve_description = cve["cve"]["descriptions"][0]["value"].as_str().unwrap().to_string();
        let cve_published_date = cve["cve"]["published"].as_str().unwrap().to_string();
        let cve_last_modified_date = cve["cve"]["lastModified"].as_str().unwrap().to_string();
        let cve_score = cve["cve"]["metrics"]["cvssMetricV31"][0]["cvssData"]["baseScore"].as_number().unwrap().to_string();
        let cve_vector = cve["cve"]["metrics"]["cvssMetricV31"][0]["cvssData"]["vectorString"].as_str().unwrap().to_string();
        let exploit_score = cve["cve"]["metrics"]["cvssMetricV31"][0]["exploitabilityScore"].as_number().unwrap().to_string();



        let cve = Cve::new(cve_id, cve_description, cve_published_date, cve_score, cve_vector, exploit_score, cve_last_modified_date);
        cve_vec.push(cve);
        // println!("cve: {:?} Description: {:?} Published date: {:?} Last_modified_date: {:?} exploit_score: {:?} BaseScore: {:?} Vector: {:?}",
        //          cve_id, cve_description, cve_published_date, cve_last_modified_date, exploit_score, cve_score, cve_vector);

    }

    cve_vec
}
