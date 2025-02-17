use futures::future::join_all;
use reqwest;
use std::time::Instant;
use tokio::{task, time};

async fn fetch_url(id: usize, url: &str, client: reqwest::Client) -> Result<(), reqwest::Error> {
    let start = Instant::now();
    println!("[{id}] Sending request to {url}");

    let response = client.get(url).send().await?;

    let duration = start.elapsed();
    println!(
        "[{id}] Response from {url}: {} (Time: {:.2?})",
        response.status(),
        duration
    );
    Ok(())
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::builder()
        .pool_max_idle_per_host(100) // Increase idle connections per host
        .build()
        .unwrap();
    let urls = vec![
        "https://example.com",
        "https://httpbin.org/get",
        "https://jsonplaceholder.typicode.com/todos/1",
        "https://jsonplaceholder.typicode.com/posts/1",
        "https://jsonplaceholder.typicode.com/users/1",
        "https://jsonplaceholder.typicode.com/comments/1",
        "https://jsonplaceholder.typicode.com/albums/1",
        "https://jsonplaceholder.typicode.com/photos/1",
        "https://jsonplaceholder.typicode.com/todos/2",
        "https://jsonplaceholder.typicode.com/posts/2",
        "https://jsonplaceholder.typicode.com/users/2",
        "https://jsonplaceholder.typicode.com/comments/2",
        "https://jsonplaceholder.typicode.com/albums/2",
        "https://jsonplaceholder.typicode.com/photos/2",
        "https://jsonplaceholder.typicode.com/todos/3",
        "https://jsonplaceholder.typicode.com/posts/3",
        "https://jsonplaceholder.typicode.com/users/3",
        "https://jsonplaceholder.typicode.com/comments/3",
        "https://jsonplaceholder.typicode.com/albums/3",
        "https://jsonplaceholder.typicode.com/photos/3",
    ];

    println!("\n=== Running with join_all (Concurrent) ===");
    let start = Instant::now();
    let handles: Vec<_> = urls
        .iter()
        .enumerate()
        .map(|(i, &url)| {
            let client = client.clone();
            task::spawn(fetch_url(i, url, client))
        })
        .collect();

    let _results: Vec<_> = join_all(handles).await;
    println!("Total time taken (join_all): {:.2?}", start.elapsed());

    println!("\n=== Running with Sequential Await ===");
    let start = Instant::now();
    let client = reqwest::Client::new();
    for (i, url) in urls.iter().enumerate() {
        let _ = fetch_url(i, url, client.clone()).await;
    }
    println!("Total time taken (sequential): {:.2?}", start.elapsed());
}
