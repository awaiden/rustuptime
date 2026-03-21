use colored::*;
use reqwest::{Client, header}; // Header desteği eklendi
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::ACCEPT,
        "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"
            .parse()
            .unwrap(),
    );
    headers.insert(header::ACCEPT_LANGUAGE, "en-US,en;q=0.5".parse().unwrap());

    let client = Client::builder()
        .user_agent(
            "Mozilla/5.0 (X11; Arch Linux; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0",
        )
        .default_headers(headers)
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(15))
        .build()
        .unwrap();

    let content = fs::read_to_string("sites.txt").expect("sites.txt not found!");
    let sites: Vec<String> = content
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let up_sites = Arc::new(AtomicUsize::new(0));
    let down_sites = Arc::new(AtomicUsize::new(0));

    let _ = fs::remove_file("errors.txt");

    println!(
        "{}",
        "--- 🛡️  WEB HEALTH CHECK PRO (Final) ---".bold().cyan()
    );

    let semaphore = Arc::new(Semaphore::new(25));
    let mut handles = vec![];

    for site_url in sites {
        let client_clone = client.clone();
        let up = Arc::clone(&up_sites);
        let down = Arc::clone(&down_sites);
        let permit = Arc::clone(&semaphore).acquire_owned().await.unwrap();

        let handle = tokio::spawn(async move {
            let _permit = permit;
            let start = Instant::now();

            match client_clone.get(&site_url).send().await {
                Ok(response) => {
                    let duration = start.elapsed().as_millis();
                    if response.status().is_success() {
                        up.fetch_add(1, Ordering::Relaxed);
                        println!(
                            "[{}] {} - {}ms",
                            "UP".green().bold(),
                            site_url,
                            duration.to_string().yellow()
                        );
                    } else {
                        down.fetch_add(1, Ordering::Relaxed);
                        println!(
                            "[{}] {} - Status: {}",
                            "DOWN".red().bold(),
                            site_url,
                            response.status()
                        );
                    }
                }
                Err(e) => {
                    down.fetch_add(1, Ordering::Relaxed);
                    if let Ok(mut file) = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open("errors.txt")
                    {
                        let _ = writeln!(file, "{} | Error: {}", site_url, e);
                    }
                    println!(
                        "[{}] {} - {}",
                        "ERROR".red().bold(),
                        site_url,
                        "FAILED".dimmed()
                    );
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    println!(
        "\nSummary: UP: {}, DOWN: {}",
        up_sites.load(Ordering::Relaxed).to_string().green(),
        down_sites.load(Ordering::Relaxed).to_string().red()
    );
}
