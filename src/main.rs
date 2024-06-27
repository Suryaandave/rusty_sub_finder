use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use reqwest::Error;
use tokio::task;

async fn request(url: &str) -> Result<reqwest::Response, Error> {
    let full_url = format!("https://{}", url);
    reqwest::get(&full_url).await
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut input = String::new();
    println!("Enter website name [in form of google.com]: ");
    io::stdin().read_line(&mut input)?;
    let target_url = input.trim();

    let file_path = "rusty_sub_finder\src\subdomains-top1million-5000.txt";
    if let Ok(lines) = read_lines(file_path) {
        let mut handles = vec![];
        for line in lines {
            if let Ok(word) = line {
                let test_url = format!("{}.{}", word, target_url).to_string();
                let handle = task::spawn(async move {
                    match request(&test_url).await {
                        Ok(_) => println!("Subdomain exists {}", test_url),
                        Err(_) => (),
                    }
                });
                handles.push(handle);
            }
        }
        for handle in handles {
            handle.await.unwrap();
        }
    }

    Ok(())
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
