use fake::Fake;
use fake::faker::name::raw::Name;
use fake::locales::JA_JP;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use tokio::task;
use tokio::time::{Duration, sleep, timeout};

#[derive(Eq, PartialEq, Debug, Clone)]
struct User {
    name: String,
    age: u8,
}

impl User {
    fn new(name: String, age: u8) -> Self {
        User { name, age }
    }
}

impl Ord for User {
    fn cmp(&self, other: &Self) -> Ordering {
        self.age.cmp(&other.age)
    }
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<User>(32);
    let (tx2, mut rx2) = mpsc::channel::<User>(1);
    let heap = Arc::new(Mutex::new(BinaryHeap::<User>::new()));

    let _ = task::spawn(async move {
        for i in 0..10 {
            let result = tx.send(User::new(Name(JA_JP).fake(), i)).await;

            match result {
                Ok(_) => (),
                Err(e) => eprintln!("エラー: {}", e),
            }

            // Simulate some delay
            sleep(Duration::from_secs(1)).await;
        }
    });

    // 受信してheapにpushするタスク
    let heap_push = heap.clone();
    let _ = task::spawn(async move {
        while let Some(message) = rx.recv().await {
            println!("メッセージを受信しました。送信をします。: {:?}", message);
            heap_push.lock().await.push(message);
        }
    });

    // popしてtx2に送信するタスク（常に動かす）
    let heap_pop = heap.clone();
    let _ = task::spawn(async move {
        loop {
            let mut heap = heap_pop.lock().await;
            if let Some(item) = heap.pop() {
                let clone_item = item.clone();
                drop(heap); // ロックを早めに解放
                let result = timeout(Duration::from_millis(0), tx2.send(item)).await;

                match result {
                    // Ok(_) => (),
                    // Err(e) => {
                    //     eprintln!("チャネル2への送信に失敗しました。: {}", e);
                    //     heap_pop.lock().await.push(e.0); // 再度push
                    // }
                    Ok(Ok(_)) => (),
                    Ok(Err(e)) => {
                        eprintln!("チャネル2への送信に失敗しました。: {}", e);
                        heap_pop.lock().await.push(e.0); // 再度push
                    }
                    Err(_) => {
                        eprintln!("送信がタイムアウトしました");
                        // 必要ならitemを再度push
                        heap_pop.lock().await.push(clone_item);
                        sleep(Duration::from_secs(1)).await; // 再試行までの待機
                    }
                }
            }
        }
    });

    while let Some(message) = rx2.recv().await {
        println!("Received user from second channel: {:?}", message);
        sleep(Duration::from_secs(10)).await; // Simulate processing delay
    }
}
