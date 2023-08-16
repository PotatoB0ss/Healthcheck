use std::{env};
use std::time::Duration;
use url::Url;

#[tokio::main]
async fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Error: incorrect arguments");
        return;
    }

    let number: u64 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Number parsing error");
            return;
        }
    };

    let url = match Url::parse(&args[2]) {
        Ok(url) => url,
        Err(_) => {
            println!("URL parsing error");
            return
        }
    };

    let interval = Duration::from_secs(number);

    loop {
        let response = reqwest::get(url.clone()).await;
        print!("Checking '{url}'. ");
        match response {
            Ok(resp) => {
                println!("Result: Ok({})", resp.status().as_u16());
            }
            Err(err) => {
                if let Some(status_code) = err.status() {
                    println!("Result: Ok({})", status_code.as_u16());
                } else {
                    println!("HTTP request error: {}", err);
                    return;
                }
            }
        }

        tokio::time::sleep(interval).await;

    }
}
