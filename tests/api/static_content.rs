use crate::helpers;

use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Read};

#[tokio::test]
async fn favicon_works() {
    // Arrange
    let test_app = helpers::spawn_app();
    let client = reqwest::Client::new();
    let file = File::open("src/vue-client/public/favicon.ico")
        .expect("Should be able to open 'favicon.ico'");
    let mut file_reader = BufReader::new(file);
    let mut file_buffer = Vec::new();
    file_reader
        .read_to_end(&mut file_buffer)
        .expect("Should be able to read 'favicon.ico'");
    // Act
    let response = client
        .get(format!("{}/favicon.ico", test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    let response_bytes = response.bytes().await;
    let response_buffer = response_bytes.expect("Should be able to open response answer");
    assert_eq!(file_buffer, response_buffer);
}

#[tokio::test]
async fn url_root_routes_to_index() {
    // Arrange
    let test_app = helpers::spawn_app();
    let client = reqwest::Client::new();
    let re = get_index_matching_reg_ex();
    // Act
    let urls = vec!["", "/"];
    for url in urls {
        let response = client
            .get(test_app.address.to_owned() + url)
            .send()
            .await
            .expect("Failed to execute request.");
        //Assert
        assert!(response.status().is_success());
        assert_eq!(
            re.captures_iter(
                response
                    .text()
                    .await
                    .expect("Failed to get request body")
                    .as_str()
            )
            .collect::<Vec<_>>()
            .len(),
            1
        );
    }
}

#[tokio::test]
async fn random_url_routes_to_index() {
    // Arrange
    let test_app = helpers::spawn_app();
    let client = reqwest::Client::new();
    let re = get_index_matching_reg_ex();
    let urls = helpers::get_random_urls(10, 15);
    // Act
    for url in urls {
        let response = client
            .get(test_app.address.to_owned() + "/" + url.as_str())
            .send()
            .await
            .expect("Failed to execute request.");
        //Assert
        assert!(response.status().is_success());
        let response = response
            .text()
            .await
            .expect("Failed to get response body")
            .as_str()
            .to_owned();
        let matches = re.captures_iter(&response).collect::<Vec<_>>();
        assert_eq!(matches.len(), 1);
    }
}

fn get_index_matching_reg_ex() -> Regex {
    Regex::new(
        r#"(?m)^<!DOCTYPE html>\r?$
^<html lang="en">\r?$
^ {2}<head>\r?$
^ {4}<meta charset="UTF-8">\r?$
^ {4}<link rel="icon" href="/favicon\.ico">\r?$
^ {4}<meta name="viewport" content="width=device-width, initial-scale=1\.0">\r?$
^ {4}<title>Vite App</title>\r?$
^ {4}<script type="module" crossorigin src="/assets/index-.+\.js"></script>\r?$
^ {4}<link rel="stylesheet" crossorigin href="/assets/index-.+\.css">\r?$
^ {2}</head>\r?$
^ {2}<body>\r?$
^ {4}<div id="app"></div>\r?$
^ {4}<!--suppress HtmlUnknownTarget -->\r?$
^ {2}</body>\r?$
</html>\r?$
"#,
    )
    .expect("Could not parse RegEx.")
}
