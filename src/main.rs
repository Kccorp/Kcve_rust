mod request;
use request::request_to_all;
use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        UrlParseError(url::ParseError);
        JsonError(serde_json::Error);
    }
}

fn main() {

    if let Err(ref e) = request_to_all("2023-01-01T00:00:00.000", "2023-02-20T00:00:00.000") {
        use error_chain::ChainedError;
        println!("Error: {}", e.display_chain());
        std::process::exit(1);
    }


}

