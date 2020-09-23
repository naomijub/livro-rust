use tokio::sync::{oneshot, mpsc};

enum Command {
    Increment,
}

#[tokio::main]
async fn main() {
    let (cmd_tx, mut cmd_rx) = mpsc::channel::<(Command, oneshot::Sender<u64>)>(100);

    tokio::spawn(async move {
        let mut counter: u64 = 0;

        while let Some((cmd, response)) = cmd_rx.recv().await {
            match cmd {
                Command::Increment => {
                    let prev = counter;
                    counter += 1;
                    response.send(prev).unwrap();
                }
            }
        }
    });

    let mut join_handles = vec![];

    (0..10).for_each(|_| {
        let mut cmd_tx = cmd_tx.clone();

        join_handles.push(tokio::spawn(async move {
            let (resp_tx, resp_rx) = oneshot::channel();

            cmd_tx.send((Command::Increment, resp_tx)).await.ok().unwrap();
            let res = resp_rx.await.unwrap();

            println!("Valor antes de incrementar = {}", res);
        }));
    });

    for join_handle in join_handles.drain(..) {
        join_handle.await.unwrap();
    }
}