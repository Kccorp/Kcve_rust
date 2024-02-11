use colored::Colorize;

/// This struct represents a CVE (Common Vulnerabilities and Exposures) with its attributes. It can 
/// be used to create a CVE object and to display the CVE information.
pub struct Cve {
    pub cve_id: String,
    pub cve_description: String,
    pub cve_published_date: String,
    pub cve_score: String,
    pub cve_vector: String,
    pub exploit_score: String,
    pub cve_last_modified_date: String,
    pub url: String,
}

impl Cve {
    
    /// This function creates a new CVE object with the given attributes. Create a new CVE object with
    /// the keyword NEW and the attributes of the CVE.
    pub fn new(cve_id: String, cve_description: String, cve_published_date: String, cve_score: String,
               cve_vector: String, exploit_score: String, cve_last_modified_date: String, url: String) -> Cve {
        Cve {
            cve_id,
            cve_description,
            cve_published_date,
            cve_score,
            cve_vector,
            exploit_score,
            cve_last_modified_date,
            url,
        }
    }

    /// This function displays the CVE ID of the CVE object. It uses the colored crate to display the
    pub fn show_cve_id(&self) {
        println!("CVE ID: {} ", self.cve_id.red());
    }

    /// This function displays the CVE information of the CVE object. It uses the colored crate to display the
    /// CVE ID in red, the CVE Score in green if it's less than 5, in yellow if it's less than 8 and in red
    /// if it's greater than 8. It uses the blue color to display the CVE Last Modified Date and the CVE Published
    /// Date.
    pub fn show_cve(&self) {
        println!("----------## CVE ID: {} ##----------", self.cve_id.red());
        println!("## CVE Description: {} ", self.cve_description);
        println!("## CVE Last Modified Date: {} ", self.cve_last_modified_date.blue());
        println!("## CVE Published Date: {} ", self.cve_published_date.blue());
        let cve_score = self.cve_score.parse::<f32>().unwrap() as i32;
        if cve_score < 5 {
            println!("## CVE Score: {} ", self.cve_score.green());
        } else if cve_score < 8 {
            println!("## CVE Score: {} ", self.cve_score.yellow());
        } else {
            println!("## CVE Score: {} ", self.cve_score.red());
        }
        println!("## CVE Vector: {} ", self.cve_vector);
        let cve_exploit_score = self.exploit_score.parse::<f32>().unwrap() as i32;
        if cve_exploit_score < 4 {
            println!("## CVE Score: {} ", self.exploit_score.green());
        } else if cve_exploit_score < 5 {
            println!("## CVE Score: {} ", self.exploit_score.yellow());
        } else {
            println!("## CVE Score: {} ", self.exploit_score.red());
        }
        println!("## URL: {} ", self.url);
        println!("------------------------------------------------\n");
    }
}