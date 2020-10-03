use tokio::sync::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let count = Arc::new(Mutex::new(0));

    for _ in 0..5 {
        let my_count = Arc::clone(&count);
        tokio::spawn(async move {
            for _ in 0..10 {
                let mut lock = my_count.lock().await;
                *lock += 1;
                println!("{}", lock);
            }
        });
    }

    loop {
        if *count.lock().await >= 50 {
            break;
        }
    }
    println!("Contagem atingiu 50.");
}