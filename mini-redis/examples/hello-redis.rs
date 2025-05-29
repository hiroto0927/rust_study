use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

/// 複数の異なるコマンドは1つのチャネルを通して「多重化 (multiplexed)」される
#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

/// リクエストを送る側が生成する。
/// "マネージャー" タスクがレスポンスをリクエスト側に送り返すために使われる
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    // 最大 32 のキャパシティをもったチャネルを作成
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    // `rx` の所有権をタスクへとムーブするために `move` キーワードを付ける
    let manager = tokio::spawn(async move {
        // サーバーへのコネクションを確立する
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // メッセージの受信を開始
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;
                    // エラーは無視する
                    let _ = resp.send(res);
                }
                Command::Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    // エラーは無視する
                    let _ = resp.send(res);
                }
            }
        }
    });

    // Spawn two tasks, one setting a value and other querying for key that was
    // set.
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "foo".to_string(),
            resp: resp_tx,
        };

        // GET リクエストを送信
        if tx.send(cmd).await.is_err() {
            eprintln!("connection task shutdown");
            return;
        }

        // レスポンスが来るのを待つ
        let res = resp_rx.await;
        println!("GOT (Get) = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx,
        };

        // SET リクエストを送信
        if tx2.send(cmd).await.is_err() {
            eprintln!("connection task shutdown");
            return;
        }

        // レスポンスが来るのを待つ
        let res = resp_rx.await;
        println!("GOT (Set) = {:?}", res);
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
