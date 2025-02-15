use futures::future::join_all;
use reqwest::Client;
use std::time::Instant;
use tokio::task;

use std::vec::Vec;
#[tokio::main]
async fn main() {
    let client = Client::new();
    let urls = vec![
        "https://example.com",
        "https://httpbin.org/get",
        "https://jsonplaceholder.typicode.com/todos/1",
    ];

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
