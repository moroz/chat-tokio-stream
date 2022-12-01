use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (tx, _rx) = broadcast::channel::<String>(10);

    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (read, mut write) = socket.split();

            let mut reader = BufReader::new(read);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send(line.clone()).unwrap();
                        line.clear();
                    }

                    result = rx.recv() => {
                        let msg = result.unwrap();

                        write.write_all(msg.as_bytes()).await.unwrap();
                    }
                }
            }
        });
    }
}
