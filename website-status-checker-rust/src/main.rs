use std::time::{Duration, Instant};
use std::env;
use std::num::ParseIntError;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread;
use std::sync::{Arc, Mutex};
use reqwest::blocking::{Client};

pub struct Config {
    pub urls: Vec<String>,
    pub workers: usize,
    pub timeout: Duration,
    pub retries: u32,
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
            return Err("No URLs provided. Use --file or provide URLs as arguments.".to_string());
        }

        Ok(Config {
            urls,
            workers,
            timeout,
            retries,
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

fn process_url(url: String, retries: u32, timeout: Duration) -> Result<String, String> {
    let client = Client::new();
    let mut attempts = 0;
    
    while attempts <= retries {
        attempts += 1;
        
        let res = client.get(&url)
            .timeout(timeout)
            .send();
        
        match res {
            Ok(response) => {
                if response.status().is_success() {
                    return Ok(format!("Successfully processed: {}", url));
                } else {
                    return Err(format!("Error processing {}: Status: {}", url, response.status()));
                }
            }
            Err(e) => {
                if attempts == retries {
                    return Err(format!("Failed to process {} after {} retries. Error: {}", url, retries, e));
                }
            }
        }

        thread::sleep(Duration::from_secs(1)); // Simulate retry delay
    }

    Err(format!("Failed to process {} after {} retries.", url, retries))
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
                        results_lock.push(result.unwrap_or_else(|e| e));
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

    // After all threads finish, print the results
    let results_lock = results.lock().unwrap();
    for result in results_lock.iter() {
        println!("{}", result);
    }
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
