use anyhow::Error;

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

    let result: String = redis::cmd("PING")
        .query_async(&mut connection)
        .await
        .expect("Failed to ping redis server");

    println!("{} should be PONG", result);

    Ok(())
}
