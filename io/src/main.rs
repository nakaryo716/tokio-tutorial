use std::{error, str::from_utf8};

use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    sync::mpsc,
    time::{self, sleep},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let (tx, mut rx) = mpsc::channel(20);

    let write_thread = tokio::spawn(async move {
        for i in 0..5 {
            println!("now write thread sleep {}/5", i + 1);
            sleep(time::Duration::from_secs(1)).await;
        }
        let mut file = File::create("text.txt").await.unwrap();
        file.write(b"renew file!").await.unwrap();

        tx.send(true).await.unwrap();
    });

    let read_thread2 = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if message {
                let mut file = File::open("text.txt").await.unwrap();

                let mut buf = Vec::new();
                file.read_to_end(&mut buf).await.unwrap();

                let text = from_utf8(&buf).unwrap().to_string();
                println!("{:?}", text);
            } else {
                continue;
            }
        }
    });

    write_thread.await.unwrap();
    read_thread2.await.unwrap();

    Ok(())
}
