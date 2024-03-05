use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, RwLock},
};

use tokio::{
    sync::{mpsc, oneshot},
    time,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx, mut rx) = mpsc::channel::<Task>(20);
    let tx2 = tx.clone();
    let tx3 = tx.clone();

    let mut repository: HashMap<i32, String> = HashMap::new();

    let maneger = tokio::spawn(async move {
        let mut count = 0;
        while let Some(tasks) = rx.recv().await {
            match tasks {
                Task::Post { text, res } => {
                    count += 1;
                    repository.insert(count, text);
                    

                    res.send("StatusCode::Created".to_string()).unwrap();
                }
                Task::Get { id, res } => {
                    let respose = repository.get(&id);
                    let ans = match respose {
                        Some(ele) => ele.clone(),
                        None => "NotFound".to_string(),
                    };
                    res.send(ans).unwrap();
                }
            }
        }
    });

    let task1 = tokio::spawn(async move {
        let (send, recive) = oneshot::channel();

        let request = Task::Post {
            text: "Hello Tokio".to_string(),
            res: send,
        };
        tx.send(request).await.unwrap();

        let response = recive.await.unwrap();
        println!("{}", response);
    });

    let task2 = tokio::spawn(async move {
        let (send, recive) = oneshot::channel();

        let request = Task::Get { id: 2, res: send };
        tx2.send(request).await.unwrap();

        let response = recive.await.unwrap();
        println!("{}", response);
    });

    let task3 = tokio::spawn(async move {
        tokio::time::sleep(time::Duration::from_secs(5)).await;

        let (send, recive) = oneshot::channel();

        let request = Task::Post {
            text: "Hello Rust".to_string(),
            res: send,
        };
        tx3.send(request).await.unwrap();

        let response = recive.await.unwrap();
        println!("{}", response);
    });

    task1.await.unwrap();
    task2.await.unwrap();
    task3.await.unwrap();
    maneger.await.unwrap();

    Ok(())
}

enum Task {
    Post {
        text: String,
        res: oneshot::Sender<String>,
    },
    Get {
        id: i32,
        res: oneshot::Sender<String>,
    },
}
