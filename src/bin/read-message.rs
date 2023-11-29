use std::time::Duration;

use anyhow::Error;
use redis::{streams::StreamReadOptions, AsyncCommands};
use test_redis_connections::STREAM_KEY;

const GROUP: &str = "test-consumers";

fn main() {
    let tokio_rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to initialize tokio runtime");

    let future = tokio_rt.block_on(async move { main_async().await });

    if let Err(error) = future {
        eprintln!("{:?}", error);
    }
}

async fn main_async() -> anyhow::Result<(), Error> {
    dotenv::dotenv().ok();

    let redis_conn_str =
        std::env::var("REDIS_CONNECTION_STRING").expect("Missing REDIS_CONNECTION_STRING");

    println!("REDIS_CONNECTION_STRING {}", redis_conn_str);

    let client = redis::Client::open(redis_conn_str).expect("Failed to open redis client");
    let mut connection = client
        .get_multiplexed_tokio_connection()
        .await
        .expect("Failed to get redis connection");

    let read_options = StreamReadOptions::default().group(GROUP, "consumer1");

    println!("Starting loop");
    loop {
        connection
            .xread_options(&[STREAM_KEY], &[">"], &read_options)
            .await?;

        println!("read message");

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
