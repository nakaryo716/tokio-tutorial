use tokio::{time::{sleep, self}, sync::mpsc};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(20);
    let tx2 =tx.clone();

    let t1 = tokio::spawn(async move {
        let mut count = 0;
        loop {
            count += 1;
            if count == 5 {
                break;
            }

            sleep(time::Duration::from_secs(10)).await;
            let _ = tx.send("input data at t1").await;
        }
    });

    let t2 = tokio::spawn(async move {
        sleep(time::Duration::from_secs(10)).await;
        20
    });

    
    let t3 = tokio::spawn(async move {
        let mut count = 0;
        loop {
            count += 1;
            if count == 5 {
                break;
            }
            
            sleep(time::Duration::from_secs(5)).await;
            let _ = tx2.send("input data at t2").await;
        }
    });
    
    let t4 = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            println!("i got {}", message);
        }
    });
    
    let data = t2.await.unwrap();
    println!("{}",data);
    
    t1.await.unwrap();
    t4.await.unwrap();
    t3.await.unwrap();
    
}