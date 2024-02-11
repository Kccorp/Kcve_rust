use crate::cve::Cve;

/// This function is used to parse the JSON received from the NVD API. It takes the JSON as parameter,
/// and then it parses the JSON to get the CVEs found. It creates a vector of CVEs, and then it calls
/// the function make_choice_to_show_cve to display the CVEs found.
pub fn parse_cve_json(json: serde_json::Value)  {
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
        let url = cve["cve"]["references"][0]["url"].as_str().unwrap().to_string();



        let cve = Cve::new(cve_id, cve_description, cve_published_date, cve_score, cve_vector, exploit_score, cve_last_modified_date, url);
        cve_vec.push(cve);
    }

    make_choice_to_show_cve(cve_vec);
}

/// This function is used to display the CVEs found. It asks the user if he wants to display all the CVEs
/// found or if he wants to choose a specific CVE to display. If the user chooses to display a specific CVE
/// it asks the user to enter the CVE id, and then it displays the CVE with the entered id.
fn make_choice_to_show_cve(cve_vec: Vec<Cve>){
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
