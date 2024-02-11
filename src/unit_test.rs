use std::io::Read;
use chrono::{DateTime, Local, NaiveDate, TimeZone};
use error_chain::error_chain;
use serde_json::json;
use crate::cve::Cve;

/// This function makes a request to the NVD API to get all the CVEs that were published between the start and end date


#[test]
pub fn parse_cve_json()  {
    let json = json!({
  "format": "NVD_CVE",
  "resultsPerPage": 5,
  "startIndex": 0,
  "timestamp": "2024-02-11T14:26:59.290",
  "totalResults": 5,
  "version": "2.0",
  "vulnerabilities": [
    {
      "cve": {
        "configurations": [
          {
            "nodes": [
              {
                "cpeMatch": [
                  {
                    "criteria": "cpe:2.3:a:pwsdashboard:personal_weather_station_dashboard:-:*:*:*:*:*:*:*",
                    "matchCriteriaId": "3F21F28D-A86E-40D6-BAED-1A5D8AA88CEE",
                    "vulnerable": true
                  }
                ],
                "negate": false,
                "operator": "OR"
              }
            ]
          }
        ],
        "descriptions": [
          {
            "lang": "en",
            "value": "PWS Personal Weather Station Dashboard (PWS_Dashboard) LTS December 2020 (2012_lts) allows remote code execution by injecting PHP code into settings.php. Attacks can use the PWS_printfile.php, PWS_frame_text.php, PWS_listfile.php, PWS_winter.php, and PWS_easyweathersetup.php endpoints. A contributing factor is a hardcoded login password of support, which is not documented. (This is not the same as the documented setup password, which is 12345.) The issue was fixed in late 2022."
          }
        ],
        "id": "CVE-2022-45291",
        "lastModified": "2023-05-04T19:57:26.577",
        "metrics": {
          "cvssMetricV31": [
            {
              "cvssData": {
                "attackComplexity": "LOW",
                "attackVector": "NETWORK",
                "availabilityImpact": "HIGH",
                "baseScore": 7.2,
                "baseSeverity": "HIGH",
                "confidentialityImpact": "HIGH",
                "integrityImpact": "HIGH",
                "privilegesRequired": "HIGH",
                "scope": "UNCHANGED",
                "userInteraction": "NONE",
                "vectorString": "CVSS:3.1/AV:N/AC:L/PR:H/UI:N/S:U/C:H/I:H/A:H",
                "version": "3.1"
              },
              "exploitabilityScore": 1.2,
              "impactScore": 5.9,
              "source": "nvd@nist.gov",
              "type": "Primary"
            }
          ]
        },
        "published": "2023-04-25T19:15:10.520",
        "references": [
          {
            "source": "cve@mitre.org",
            "tags": [
              "Exploit",
              "Technical Description",
              "Third Party Advisory"
            ],
            "url": "https://cavefxa.com/posts/cve-2022-45291/"
          },
          {
            "source": "cve@mitre.org",
            "tags": [
              "Product"
            ],
            "url": "https://pwsdashboard.com/"
          }
        ],
        "sourceIdentifier": "cve@mitre.org",
        "vulnStatus": "Analyzed",
        "weaknesses": [
          {
            "description": [
              {
                "lang": "en",
                "value": "CWE-798"
              }
            ],
            "source": "nvd@nist.gov",
            "type": "Primary"
          }
        ]
      }
    }
  ]
});

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
}

#[test]
fn refactor_date_str_to_utc()  {
    let date = "2023-01-01";
    let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
    let date = date.and_hms_opt(0, 0, 0);
    let date: DateTime<chrono::Utc> = DateTime::from_naive_utc_and_offset(date.unwrap(), chrono::Utc);
    assert_eq!(date.to_rfc3339(), "2023-01-01T00:00:00+00:00");
}

#[test]
fn refactor_date_unify_separator()  {
    let date = "2023/01/01";
    let re = regex::Regex::new(r"[- /.]").unwrap();
    let date = re.replace_all(date, "-");
    assert_eq!(date, "2023-01-01");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_get_request() {
        // Your test logic here
        if let Err(ref e) = http_get_request() {
            use error_chain::ChainedError;
            println!("Error: {}", e.display_chain());
            std::process::exit(1);
        }
    }
}

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        UrlParseError(url::ParseError);
        JsonError(serde_json::Error);
    }
}

fn http_get_request() -> crate::Result<String> {
    let params = vec![
        ("pubStartDate", "2023-01-01T00:00:00.000"),
        ("pubEndDate", "2023-01-05T00:00:00.000"),
        ("sourceIdentifier", "nvd@nist.gov"),
    ];

    let url = "https://services.nvd.nist.gov/rest/json/cves/2.0";
    let url_test = reqwest::Url::parse_with_params(url, &params)?;
    let mut res = reqwest::blocking::get(url_test)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    if res.status() != 200 {
        println!("Error: {:#?}", res.headers());
        std::process::exit(1);
    }

    Ok(body)
}