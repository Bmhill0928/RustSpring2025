use std::time::{Duration, Instant};
use std::env;
use std::num::ParseIntError;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread;
use std::sync::{Arc, Mutex};
use reqwest::blocking::{Client};
use std::fs::OpenOptions;
use std::io::Write;
use std::time::SystemTime;

#[derive(Clone)]
pub struct WebsiteStatus {
    pub url: String,                       // original URL
    pub action_status: Result<u16, String>, // HTTP status code or error message
    pub response_time: Duration,           // how long the request took
    pub timestamp: SystemTime,             // when the attempt completed
}

pub struct Config {
    pub urls: Vec<String>,
    pub workers: usize,
    pub timeout: Duration,
    pub retries: u32,
    pub status_file: String,
}

impl Config {
    pub fn from_args() -> Result<Self, String> {
        let mut args = env::args().skip(1);

        let mut urls = Vec::new();
        let mut file_path: Option<String> = None;
        let mut workers = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        let mut timeout = Duration::from_secs(5);
        let mut retries = 0;
        let mut status_file = String::from("status.json");

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--file" => {
                    file_path = Some(args.next().ok_or("Missing value after --file.")?);
                }
                "--workers" => {
                    let val = args.next().ok_or("Missing value after --workers.")?;
                    workers = parse_usize(&val).map_err(|_| "Invalid value for workers.")?;
                }
                "--timeout" => {
                    let val = args.next().ok_or("Missing value after --timeout.")?;
                    let secs = val.parse::<u64>().map_err(|_| "Invalid timeout value.")?;
                    timeout = Duration::from_secs(secs);
                }
                "--retries" => {
                    let val = args.next().ok_or("Missing value after --retries")?;
                    retries = val.parse::<u32>().map_err(|_| "Invalid retries value.")?;
                }
                "--status-file" => {
                    status_file = args.next().ok_or("Missing value after --status-file")?;
                }
                _ => {
                    if arg.starts_with("--") {
                        return Err(format!("Unknown flag: {}", arg));
                    } else {
                        urls.push(arg);
                    }
                }
            }
        }

        if let Some(path) = file_path {
            let file_urls = read_urls_from_file(&path)?;
            urls.extend(file_urls);
        }

        if urls.is_empty() {
            eprintln!("Usage: website_checker [URL | --file FILE] [--workers N] [--timeout S] [--retries N] --status-file FILE");
            std::process::exit(2);
        }

        Ok(Config {
            urls,
            workers,
            timeout,
            retries,
            status_file,
        })
    }
}

fn parse_usize(s: &str) -> Result<usize, ParseIntError> {
    s.parse::<usize>()
}

fn read_urls_from_file(path: &str) -> Result<Vec<String>, String> {
    let file = File::open(Path::new(path)).map_err(|e| format!("Failed to open file: {}", e))?;
    let reader = io::BufReader::new(file);
    let mut urls = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
        if !line.trim().is_empty() {
            urls.push(line.trim().to_string());
        }
    }

    Ok(urls)
}

fn process_url(url: String, retries: u32, timeout: Duration) -> WebsiteStatus {
    let client = Client::new();
    let start_time = Instant::now();
    let mut attempts = 0;

    while attempts <= retries {
        attempts += 1;

        let res = client.get(&url)
            .timeout(timeout)
            .send();

        let response_time = start_time.elapsed();

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    // Successfully processed, print and return status
                    println!("Successfully processed: {}", url);
                    return WebsiteStatus {
                        url: url.clone(),
                        action_status: Ok(response.status().as_u16()),
                        response_time,
                        timestamp: SystemTime::now(),
                    };
                } else {
                    // Failed with status code
                    let error_msg = format!("Error processing {}: Status: {}", url, response.status());
                    println!("{}", error_msg);
                    return WebsiteStatus {
                        url: url.clone(),
                        action_status: Err(error_msg),
                        response_time,
                        timestamp: SystemTime::now(),
                    };
                }
            }
            Err(e) => {
                // Failure due to request error
                let error_msg = format!("Failed to process {}: Error: {}", url, e);
                if attempts == retries {
                    println!("{}", error_msg);
                    return WebsiteStatus {
                        url: url.clone(),
                        action_status: Err(error_msg),
                        response_time,
                        timestamp: SystemTime::now(),
                    };
                }
            }
        }

        thread::sleep(Duration::from_secs(1)); // Simulate retry delay
    }

    WebsiteStatus {
        url,
        action_status: Err("Max retries reached.".to_string()),
        response_time: start_time.elapsed(),
        timestamp: SystemTime::now(),
    }
}

fn process_urls_concurrently(config: &Config) {
    let urls = Arc::new(Mutex::new(config.urls.clone()));
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for _ in 0..config.workers {
        let urls = Arc::clone(&urls);
        let retries = config.retries;
        let timeout = config.timeout;
        let results = Arc::clone(&results);

        let handle = thread::spawn(move || {
            loop {
                let url = {
                    let mut urls = urls.lock().unwrap();
                    urls.pop()
                };

                match url {
                    Some(url) => {
                        let result = process_url(url, retries, timeout);
                        let mut results_lock = results.lock().unwrap();
                        results_lock.push(result);
                    }
                    None => break,
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // After all threads finish, write the results to the status file
    let results_lock = results.lock().unwrap();
    write_status_to_file(config.status_file.clone(), results_lock.clone());
}

fn write_status_to_file(file_path: String, website_statuses: Vec<WebsiteStatus>) {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)
        .unwrap();

    let mut file = std::io::BufWriter::new(file);

    file.write_all(b"[\n").unwrap();
    for (i, status) in website_statuses.iter().enumerate() {
        let timestamp = format!("{:?}", status.timestamp);
        let millis = status.response_time.as_millis(); // just a number

        match &status.action_status {
            Ok(code) => {
                write!(
                    file,
                    r#"{{"url": "{}", "action_status": {}, "response_time": {}, "timestamp": "{}"}}"#,
                    status.url, code, millis, timestamp
                )
                .unwrap();
            }
            Err(error) => {
                write!(
                    file,
                    r#"{{"url": "{}", "action_status": "{}", "response_time": {}, "timestamp": "{}"}}"#,
                    status.url, error, millis, timestamp
                )
                .unwrap();
            }
        }

        if i != website_statuses.len() - 1 {
            write!(file, ",\n").unwrap();
        } else {
            write!(file, "\n").unwrap();
        }
    }
    file.write_all(b"]\n").unwrap();
}

fn main() {
    let config = match Config::from_args() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    println!("Starting to process URLs with {} workers...", config.workers);
    let start_time = Instant::now();
    process_urls_concurrently(&config);
    let duration = start_time.elapsed();
    println!("Processing completed in {:?}", duration);
}
