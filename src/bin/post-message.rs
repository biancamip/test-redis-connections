use anyhow::Error;

fn main() {
    let tokio_rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to initialize tokio runtime");

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

    Ok(())
}
