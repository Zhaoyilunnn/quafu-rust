use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const CREDENTIAL_PATH: &'static str = "/root/.quafu/api";
const API_BACKENDS: &'static str = "qbackend/get_backends/";

pub struct QuafuClient {
    is_compile: bool,
    tomo: bool,
    priority: u32,
    api_token: String,
    website: String,
}

impl QuafuClient {
    pub fn new() -> QuafuClient {
        QuafuClient {
            is_compile: true,
            tomo: false,
            priority: 2,
            api_token: String::default(),
            website: String::default(),
        }
    }

    pub fn load_credential(&mut self) -> Result<(), std::io::Error> {
        let file = File::open(CREDENTIAL_PATH)?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        // Parse API token
        let api_token = match lines.next() {
            Some(Ok(line)) => line,
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Failed to read API token",
                ))
            }
        };

        // Parse website url
        let website = match lines.next() {
            Some(Ok(line)) => line,
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Failed to read website",
                ))
            }
        };

        // Check unexpected data
        if lines.next().is_some() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Unexpected data in config file",
            ));
        }

        self.api_token = api_token;
        self.website = website;

        // TODO: delete
        println!("API token: {}", self.api_token);
        println!("Website: {}", self.website);
        Ok(())
    }

    pub fn get_backends(&self) -> Result<HashMap<String, Value>, reqwest::Error> {
        let mut headers = HeaderMap::new();
        headers.insert("api_token", HeaderValue::from_str(&self.api_token).unwrap());

        let url = format!("{}{}", self.website, API_BACKENDS);

        let client = Client::new();
        let request = client.post(&url).headers(headers);

        let response = request.send()?;

        // TODO: unified error handling

        let backends_json: Value = response.json()?;

        let mut backends = HashMap::new();
        for b in backends_json["data"].as_array().unwrap_or(&vec![]) {
            if let Some(system_name) = b["system_name"].as_str() {
                backends.insert(system_name.to_string(), b.clone());
            }
        }
        println!("{}", backends_json.to_string());
        Ok(backends)
    }
}
