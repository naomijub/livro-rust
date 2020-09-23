use tokio::sync::mpsc;

const EDN: &'static str = "{
    :a #{:otavio :icaro :thais :tanara :aline :vanessa}
    :b #{:vivi :rafa :elisa :cintia #{:tami :tamires}}
    :c #{:juily :camila :isabela #{:carol ricca}}
}";

async fn replace_set(edn_str: &str) -> String {
    edn_str.replace("#{","@")
}

#[tokio::main]
async fn main() {
    let (mut tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
        for e in EDN.split('\n') {
            let res = replace_set(e).await;
            tx.send(res).await.unwrap();
        };
    });

    let mut  formatted_edn = String::new();
    while let Some(res) = rx.recv().await {
        formatted_edn.push_str(&res);
    }
    println!("got = {}", formatted_edn);
}

// #[tokio::main]
// async fn main() {
//     let (mut tx, mut rx) = mpsc::channel(100);

//     for e in EDN.split('\n') {
//         let mut tx = tx.clone();

//         tokio::spawn(async move {
//             let res = replace_set(e).await;
//             tx.send(res).await.unwrap();
//         });
//     };
//     drop(tx);

//     let mut  formatted_edn = String::new();
//     while let Some(res) = rx.recv().await {
//         formatted_edn.push_str(&res);
//     }
//     println!("got = {}", formatted_edn);
// }