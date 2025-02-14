use reqwest::Client;
use std::error::Error;
use tokio::{
    task,
    time::{sleep, Duration},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let seedlist = [
        "http://torlinkv7cft5zhegrokjrxj2st4hcimgidaxdmcmdpcrnwfxrr2zxqd.onion/",
        "http://fvrifdnu75abxcoegldwea6ke7tnb3fxwupedavf5m3yg3y2xqyvi5qd.onion/",
        "http://zqktlwiuavvvqqt4ybvgvi7tyo4hjl5xgfuvpdf6otjiycgwqbym2qad.onion/wiki/index.php/Main_Page",
    ];

    let client = Client::builder()
        .proxy(reqwest::Proxy::all("socks5h://127.0.0.1:9050")?)
        .build()?;

    let mut handles = vec![];

    for url in seedlist.iter() {
        let client = client.clone();
        let url = url.to_string();
        handles.push(task::spawn(async move {
            println!("Fetching: {}", url);
            match client.get(&url).send().await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        match resp.text().await {
                            Ok(body) => println!("Success: {}", &body[..50]), // Print only a snippet
                            Err(e) => println!("Failed to read response body: {}", e),
                        }
                    } else {
                        println!("Failed: {} - Status: {}", url, resp.status());
                    }
                }
                Err(e) => println!("Error fetching {}: {}", url, e),
            }
        }));
    }

    for handle in handles {
        handle.await?;
    }

    Ok(())
}
