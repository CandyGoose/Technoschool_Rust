use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::time::Duration;
use url::{Url, ParseError};
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("rust-wget")
        .arg(Arg::new("url")
            .short('u')
            .long("url")
            .value_name("URL")
            .help("Specifies the root URL to download")
            .required(true))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_name("DIRECTORY")
            .help("Specifies the directory where to save the downloaded content")
            .required(true))
        .get_matches();

    let root_url = matches.get_one::<String>("url").unwrap();
    let output_dir = matches.get_one::<String>("output").unwrap();


    let mut visited_urls = HashSet::new();

    if let Err(e) = download_site(root_url, output_dir, &mut visited_urls) {
        eprintln!("Error downloading site: {}", e);
    } else {
        println!("Site downloaded successfully!");
    }
}

fn download_site(root_url: &str, output_dir: &str, visited_urls: &mut HashSet<String>) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    download_page(&client, root_url, output_dir, visited_urls)?;

    Ok(())
}

fn download_page(client: &Client, url: &str, output_dir: &str, visited_urls: &mut HashSet<String>) -> Result<(), Box<dyn std::error::Error>> {
    if visited_urls.contains(url) {
        return Ok(());
    }

    visited_urls.insert(url.to_string());

    // Загружаем страницу
    println!("Downloading: {}", url);
    let response = client.get(url).send()?;

    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");

    if content_type.contains("text/html") {
        let html_text = response.text()?;
        let document = Document::from(html_text.as_str());

        let parsed_url = Url::parse(url)?;
        let local_path = url_to_local_path(&parsed_url, output_dir)?;
        save_file(&local_path, html_text.as_bytes())?;

        for node in document.find(Name("a")).filter_map(|n| n.attr("href")) {
            if let Ok(new_url) = resolve_url(&parsed_url, node) {
                download_page(client, new_url.as_str(), output_dir, visited_urls)?;
            }
        }

        for node in document.find(Name("img")).filter_map(|n| n.attr("src")) {
            if let Ok(new_url) = resolve_url(&parsed_url, node) {
                download_resource(client, new_url.as_str(), output_dir, visited_urls)?;
            }
        }

        for node in document.find(Name("link")).filter_map(|n| n.attr("href")) {
            if let Ok(new_url) = resolve_url(&parsed_url, node) {
                download_resource(client, new_url.as_str(), output_dir, visited_urls)?;
            }
        }

        for node in document.find(Name("script")).filter_map(|n| n.attr("src")) {
            if let Ok(new_url) = resolve_url(&parsed_url, node) {
                download_resource(client, new_url.as_str(), output_dir, visited_urls)?;
            }
        }
    } else {
        download_resource(client, url, output_dir, visited_urls)?;
    }

    Ok(())
}

fn download_resource(client: &Client, url: &str, output_dir: &str, visited_urls: &mut HashSet<String>) -> Result<(), Box<dyn std::error::Error>> {
    if visited_urls.contains(url) {
        return Ok(());
    }

    visited_urls.insert(url.to_string());

    println!("Downloading resource: {}", url);
    let response = client.get(url).send()?.bytes()?;

    let parsed_url = Url::parse(url)?;
    let local_path = url_to_local_path(&parsed_url, output_dir)?;
    save_file(&local_path, &response)?;

    Ok(())
}

fn url_to_local_path(url: &Url, output_dir: &str) -> Result<std::path::PathBuf, ParseError> {
    let mut path = Path::new(output_dir).join(url.host_str().unwrap_or("unknown"));

    for segment in url.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap_or_else(Vec::new) {
        path = path.join(segment);
    }

    if path.extension().is_none() {
        path = path.with_extension("html");
    }

    Ok(path)
}

fn save_file(path: &Path, content: &[u8]) -> Result<(), std::io::Error> {
    if path.exists() {
        println!("File already exists, skipping: {:?}", path);
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, content)?;
    Ok(())
}

fn resolve_url(base_url: &Url, href: &str) -> Result<Url, ParseError> {
    base_url.join(href)
}
