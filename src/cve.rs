use colored::Colorize;

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

    pub fn show_cve_id(&self) {
        println!("CVE ID: {} ", self.cve_id.red());
    }

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