use std::{sync::Arc, time::Duration};

use tokio::{
    sync::{mpsc::{self, Receiver}, Mutex},
    time::sleep,
};

type SharedReceiver = Arc<Mutex<Receiver<String>>>;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<String>(32);
    let tx2 = tx.clone();
    let arx: SharedReceiver = Arc::new(Mutex::new(rx));
    let arx2: SharedReceiver = arx.clone();

    let s1 = tokio::spawn(async move {
        loop {
            tx.send("sending from prod 1".to_string()).await.unwrap();
            sleep(Duration::from_millis(100)).await;
        }
    });

    let s2 = tokio::spawn(async move {
        loop {
            tx2.send("sending from prod 2".to_string()).await.unwrap();
            sleep(Duration::from_millis(100)).await;
        }
    });

    let r1 = tokio::spawn(async move {
        loop {
            let mut lrx = arx.lock().await;
            let message = lrx.recv().await;
            match message {
                Some(m) => println!("one: {}", m),
                None => continue,
            }
            sleep(Duration::from_millis(100)).await;
        }
    });

    let r2 = tokio::spawn(async move {
        loop {
            let mut lrx = arx2.lock().await;
            let message = lrx.recv().await;
            match message {
                Some(m) => println!("two: {}", m),
                None => continue,
            }
            sleep(Duration::from_millis(100)).await;
        }
    });

    let (_, _, _, _) = tokio::join!(s1, s2, r1, r2);
}
