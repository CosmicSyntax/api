use tokio::spawn;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::task::JoinHandle;

use crate::error::{self, ApiErrors};

pub mod models;

#[allow(dead_code)]
pub struct Manager<T>
where
    T: DbExec<T> + Send + 'static,
{
    ingress: Option<Receiver<T>>,
    handlers: Option<Vec<JoinHandle<()>>>,
    egress: Option<Vec<Sender<T>>>,
}

impl<T> Manager<T>
where
    T: DbExec<T> + Send + 'static,
{
    pub fn new(recv: Receiver<T>, pool_size: usize) -> Self {
        let mut egress = Vec::with_capacity(pool_size);
        let mut handles = Vec::with_capacity(pool_size);
        for _ in 0..pool_size {
            // create the channels between manager the its workers
            let (s, mut r) = channel::<T>(1000);
            // spawn a green thread and send off with r and rkill
            let handle = spawn(async move {
                // when the receiver is dropped, break out of loop
                while let Some(i) = r.recv().await {
                    // placeholder
                    let _ = i.set();
                }
            });
            // store the sender into Vec
            egress.push(s);
            handles.push(handle);
        }
        Manager {
            ingress: Some(recv),
            handlers: Some(handles),
            egress: Some(egress),
        }
    }

    pub fn start(&mut self) -> Result<JoinHandle<()>, ApiErrors> {
        if let (Some(mut ingress), Some(handles), Some(egress)) = (
            self.ingress.take(),
            self.handlers.take(),
            self.egress.take(),
        ) {
            return Ok(spawn(async move {
                // Listen for instructions from the sender from main thread
                let mut index = 0;
                let pool_size = egress.len();
                while let Some(i) = ingress.recv().await {
                    // Send the instructions to each worker in a roundrobin fashion
                    if Self::roundrobin(&egress, &mut index, pool_size, i).await.is_err() {
                        eprintln!("Could not send instruction to worker.");
                    }
                }
                drop(handles);
            }));
        }
        Err(error::MANAGER_START_ERROR)
    }

    #[inline]
    async fn roundrobin(
        pool: &[Sender<T>],
        index: &mut usize,
        pool_size: usize,
        instruct: T,
    ) -> Result<(), SendError<T>> {
        let r = pool[*index].send(instruct).await;
        if *index == pool_size {
            *index = 0;
        } else {
            *index += 1;
        }
        r
    }
}

// Trait for executing a SQL commands to postgres
pub trait DbExec<T> {
    fn set(&self) -> T;
}

pub struct Kill;

impl DbExec<()> for Kill {
    // This is the function to kill all the workers
    fn set(&self) {}
}

#[cfg(test)]
mod test {

    // Testing if we are able to make the connections to the data and pool them
    #[tokio::test]
    async fn test_run() {}
}

/* let mut pool = VecDeque::with_capacity(pool_size);
let config_tls = tls_config();
let tls = tokio_postgres_rustls::MakeRustlsConnect::new(config_tls);
for _ in 0..pool_size {
    let (client, conn) = tokio_postgres::connect(url, tls.clone()).await?;
    pool.push_back(client);
    // If the client is dropped, conn will go too
    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("connection error: {}", e);
        }
    });
} */
