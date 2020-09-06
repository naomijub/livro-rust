use tokio::sync::oneshot;
use std::thread;
use std::time::Duration;

async fn um_calculo() -> String {
    // tarefa longa
    thread::sleep(Duration::new(2, 0));
    "resultado".to_string()
}

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        let res = um_calculo().await;
        tx.send(res).unwrap();
    });

    // calculo 1
    thread::sleep(Duration::new(1, 0));
    println!("calculo 1");

    // calculo 2
    thread::sleep(Duration::new(2, 0));
    println!("calculo 2");

    // Esperar o calculo terminar
    let res = rx.await.unwrap();
    println!("{}", res);
}