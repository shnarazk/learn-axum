use {
    bytes::{buf::Buf, Bytes},
    mini_redis::{Connection, Frame, Result},
    std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    },
    tokio::net::{TcpListener, TcpStream},
};

#[derive(Clone, Debug, Default)]
struct SharedDb {
    entry: Arc<Vec<Mutex<HashMap<String, u8>>>>,
}

impl SharedDb {
    fn new(num_shards: usize) -> SharedDb {
        let mut db = Vec::with_capacity(num_shards);
        for _ in 0..num_shards {
            db.push(Mutex::new(HashMap::new()));
        }
        Self {
            entry: Arc::new(db),
        }
    }
    #[allow(dead_code)]
    fn get_shard(&self, key: &str) -> &Mutex<HashMap<String, u8>> {
        fn hash(s: &str) -> usize {
            s.len()
        }
        &self.entry[hash(key) % self.entry.len()] // .lock().unwrap()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening...");

    // On using std::sync::Mutex
    //
    // Note, std::sync::Mutex and not tokio::sync::Mutex is used to guard the HashMap. A common
    // error is to unconditionally use tokio::sync::Mutex from within async code. An async mutex is
    // a mutex that is locked across calls to .await.
    //
    // A synchronous mutex will block the current thread when waiting to acquire the lock. This, in
    // turn, will block other tasks from processing. However, switching to tokio::sync::Mutex
    // usually does not help as the asynchronous mutex uses a synchronous mutex internally.
    //
    // As a rule of thumb, using a synchronous mutex from within asynchronous code is fine as long
    // as contention remains low and the lock is not held across calls to .await. Additionally,
    // consider using parking_lot::Mutex as a faster alternative to std::sync::Mutex.
    let db: SharedDb = SharedDb::new(5);
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        print!("Accepted");
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: SharedDb) {
    use mini_redis::Command::{self, Get, Set};
    // let mut db = HashMap::new();
    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let shard = db.get_shard(cmd.key());
                let mut db = shard.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone().get_u8());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let shard = db.get_shard(cmd.key());
                let db = shard.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(Bytes::from(format!("{value}")))
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }
}

#[allow(dead_code)]
async fn process_old(socket: TcpStream) {
    let mut connection = Connection::new(socket);
    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}
