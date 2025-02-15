use futures::future::join_all;
use reqwest::Client;
use std::time::Instant;
use tokio::task;

use std::vec::Vec;
#[tokio::main]
async fn main() {
    let urls = [
        "http://torlinkv7cft5zhegrokjrxj2st4hcimgidaxdmcmdpcrnwfxrr2zxqd.onion/",
        "http://fvrifdnu75abxcoegldwea6ke7tnb3fxwupedavf5m3yg3y2xqyvi5qd.onion/",
        "http://zqktlwiuavvvqqt4ybvgvi7tyo4hjl5xgfuvpdf6otjiycgwqbym2qad.onion/wiki/index.php/Main_Page",
    ];

    let client = Client::builder()
        .proxy(reqwest::Proxy::all("socks5h://127.0.0.1:9050").unwrap())
        .build()
        .unwrap();

    let start = Instant::now();

    // Create a vector to hold the JoinHandles of the spawned tasks
    let mut handles = Vec::new();

    for url in urls {
        let client = client.clone();
        let handle = task::spawn(async move {
            let request_start = Instant::now();
            println!(
                "[{:?}] Sending request to {}",
                request_start.duration_since(start),
                url
            );
            match client.get(&*url).send().await {
                Ok(resp) => match resp.text().await {
                    Ok(text) => println!(
                        "[{:?}] Response from {}: {}",
                        Instant::now().duration_since(start),
                        url,
                        &text.to_string()[..20]
                    ),
                    Err(err) => eprintln!(
                        "[{:?}] Error reading response from {}: {}",
                        Instant::now().duration_since(start),
                        url,
                        err
                    ),
                },
                Err(err) => eprintln!(
                    "[{:?}] Error fetching {}: {}",
                    Instant::now().duration_since(start),
                    url,
                    err
                ),
            }
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    let mut results = join_all(handles).await;

    // let mut results = Vec::new(); // Declare results before the loop
    // for handle in handles {
    //     let result = handle.await;
    //     results.push(result);
    // }
    //
    // Handle any errors that occurred during task execution
    for result in results {
        if let Err(err) = result {
            eprintln!("Task failed: {}", &err.to_string()[..20]);
        }
    }
}
