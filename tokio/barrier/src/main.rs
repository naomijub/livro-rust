use tokio::sync::Barrier;

use futures::future::join_all;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let mut handles = Vec::with_capacity(10);
    let barrier = Arc::new(Barrier::new(10));
    for i in 0..10 {
        let c = barrier.clone();

        handles.push(async move {
            println!("antes do wait {}", i);
            let wr = c.wait().await;
            println!("depois do wait {}", i);
            wr
        });
    }

    let wrs = join_all(handles).await;
    assert_eq!(wrs.into_iter().filter(|wr| wr.is_leader()).count(), 1);
}