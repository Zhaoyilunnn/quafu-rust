use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Serialize;
use serde_json::Value;
use serde_urlencoded;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const CREDENTIAL_PATH: &'static str = "/root/.quafu/api";
const API_BACKENDS: &'static str = "qbackend/get_backends/";
const API_EXEC: &'static str = "qbackend/scq_kit/";
const API_EXEC_ASYNC: &'static str = "qbackend/scq_kit_asyc/";
const QUAFU_VERSION: &'static str = "0.4.0";

pub struct QRes {
    text: String,
}

// References:
//  https://serde.rs/derive.html
#[derive(Serialize)] // Generate serialized code using serde
struct QPayload {
    qtasm: String,
    shots: String,
    qubits: String,
    scan: String,
    tomo: i32,
    selected_server: String,
    compile: i32,
    priority: String,
    task_name: String,
    pyquafu_version: String,
    runtime_job_id: String,
}

pub struct QClient {
    is_compile: bool,
    tomo: bool,
    priority: u32,
    api_token: String,
    website: String,
    backends: HashMap<String, serde_json::Value>,
    backend_name: String,
    shots: u32,
}

impl QClient {
    pub fn new() -> QClient {
        QClient {
            is_compile: true,
            tomo: false,
            priority: 2,
            api_token: String::default(),
            website: String::default(),
            backends: HashMap::default(),
            backend_name: String::from("Baiwang"),
            shots: 1024,
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
        Ok(())
    }

    pub fn get_backends(&mut self) -> Result<(), reqwest::Error> {
        let mut headers = HeaderMap::new();
        headers.insert("api_token", HeaderValue::from_str(&self.api_token).unwrap());

        let url = format!("{}{}", self.website, API_BACKENDS);

        let client = Client::new();
        let request = client.post(&url).headers(headers);

        let response = request.send()?;

        // TODO: unified error handling

        let backends_json: Value = response.json()?;

        for b in backends_json["data"].as_array().unwrap_or(&vec![]) {
            if let Some(system_name) = b["system_name"].as_str() {
                self.backends.insert(system_name.to_string(), b.clone());
            }
        }
        println!("{}", backends_json.to_string());
        Ok(())
    }

    pub fn execute(&self, qasm: &str, name: &str, async_flag: bool) -> QRes {
        let backend = self.backends.get(&self.backend_name).unwrap(); // get backend

        // Construct payload
        let payload = serde_urlencoded::to_string(&QPayload {
            qtasm: qasm.to_string(),
            shots: self.shots.to_string(),
            qubits: "1".to_string(), // TODO: extract from qasm?
            scan: "0".to_string(),
            tomo: self.tomo as i32,
            selected_server: backend["system_id"].as_i64().unwrap().to_string(),
            compile: self.is_compile as i32,
            priority: self.priority.to_string(),
            task_name: name.to_string(),
            pyquafu_version: QUAFU_VERSION.to_string(),
            runtime_job_id: "".to_string(),
        })
        .unwrap();

        // select API based on async_flag
        let url = if async_flag {
            format!("{}{}", &self.website, API_EXEC_ASYNC)
        } else {
            format!("{}{}", &self.website, API_EXEC)
        };

        // set header
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded;charset=UTF-8"
                .parse()
                .unwrap(),
        );
        headers.insert("api_token", self.api_token.parse().unwrap());

        // Send to website
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(&url)
            .headers(headers)
            .body(payload)
            .send()
            .unwrap(); // TODO: handle all possible scenarios

        // TODO: Check error
        if !response.status().is_success() {
            panic!("Website error"); //
        }

        // Get result
        let text = response.text().unwrap();

        println!("Execution result: \n{}", text);

        QRes { text }
    }

    pub fn info(&self) {
        println!("Website: {}", self.website);
        println!("API Token: {}", self.api_token);
    }
}
