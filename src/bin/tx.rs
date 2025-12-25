use tokio::sync::mpsc::channel;
use tokio::time;

struct Message {
    value: i32,
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = channel::<Message>(32);

    tokio::spawn(async move {
        tx.send(Message { value: 42 }).await.unwrap();
        tx.send(Message { value: 213 }).await.unwrap();
        time::sleep(time::Duration::from_secs(1)).await;
        tx.send(Message { value: 4322 }).await.unwrap();
        tx.send(Message { value: 3 }).await.unwrap();
        tx.send(Message { value: 32 }).await.unwrap();
    });

    while let Some(message) = rx.recv().await {
        println!("{}", message.value);
    }
    println!("\nEND");
}
