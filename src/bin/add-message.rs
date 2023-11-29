use std::time::Duration;

use anyhow::Error;
use chrono::Utc;
use redis::{streams::StreamMaxlen, AsyncCommands};

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

    println!("Running with REDIS_CONNECTION_STRING {}", redis_conn_str);

    let client = redis::Client::open(redis_conn_str).expect("Failed to open redis client");
    let mut connection = client
        .get_multiplexed_tokio_connection()
        .await
        .expect("Failed to get redis connection");

    loop {
        let mut fields = vec![];
        fields.push(("timestamp", Utc::now().timestamp_millis()));

        connection
            .xadd_maxlen(
                "test-connection-stream",
                StreamMaxlen::Approx(100),
                "*",
                &fields,
            )
            .await?;
        println!("msg posted");

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
