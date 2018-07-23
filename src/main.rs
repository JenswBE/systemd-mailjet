use std::env;
use std::process::Command;

#[macro_use]
extern crate serde_derive;

extern crate reqwest;
use reqwest::header::ContentType;
use reqwest::mime::APPLICATION_JSON;

// Structures
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct Address {
    name: String,
    email: String,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct Message {
    from: Address,
    to: Vec<Address>,
	subject: String,
	text_part: String,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct Request {
    messages: Vec<Message>
}

fn main() {
    // Constants
    const API_URL: &'static str = "https://api.mailjet.com/v3.1/send";

    // Get env variables
    let from_name = env::var("FROM_NAME").expect("From name missing");
    let from_email = env::var("FROM_EMAIL").expect("From email missing");
    let to_name = env::var("TO_NAME").expect("To name missing");
    let to_email = env::var("TO_EMAIL").expect("To email missing");
    let api_key_public = env::var("API_KEY_PUBLIC").expect("Public API key missing");
    let api_key_private = env::var("API_KEY_PRIVATE").expect("Private API key missing");

    // Get service name
    let service = env::args().nth(1).expect("No service name given");

    // Get service status
    let status = Command::new("systemctl").arg("status").arg(&service).output().expect("Fetch status command failed (systemctl status <service name>)");
    let status = String::from_utf8_lossy(&status.stdout);

    // Build request object
    let request = Request {
        messages: vec!(
            Message {
                from: Address {
                    name: from_name,
                    email: from_email,
                },
                to: vec!(
                    Address {
                        name: to_name,
                        email: to_email,
                    }
                ),
                subject: format!("Service {} failed", service),
                text_part: status.to_owned().to_string()
            }
        )
    };

    // Send request
    let client = reqwest::Client::new();
    let res = client.post(API_URL)
    .header(ContentType(APPLICATION_JSON))
    .basic_auth(api_key_public, Some(api_key_private))
    .json(&request)
    .send()
    .expect("Request to Mailjet failed");
    if !res.status().is_success() {
        println!("Received error response: {}", res.status());
        std::process::exit(1);
    }
}
