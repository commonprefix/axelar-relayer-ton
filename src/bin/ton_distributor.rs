use dotenv::dotenv;
use std::sync::Arc;
use tokio::signal::unix::{signal, SignalKind};

use relayer_base::{
    config::Config,
    database::PostgresDB,
    distributor::Distributor,
    gmp_api,
    queue::Queue,
    utils::{setup_heartbeat, setup_logging},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // TODO: Consider refactoring to reuse boilerplate

    dotenv().ok();
    let network = std::env::var("NETWORK").expect("NETWORK must be set");
    let config = Config::from_yaml(&format!("config.{}.yaml", network)).unwrap();

    let _guard = setup_logging(&config);

    let tasks_queue = Queue::new(&config.queue_address, "tasks").await;
    let gmp_api = Arc::new(gmp_api::GmpApi::new(&config, true).unwrap());
    let postgres_db = PostgresDB::new(&config.postgres_url).await.unwrap();

    let mut distributor = Distributor::new(
        postgres_db,
        "default".to_string(),
        gmp_api,
        config.refunds_enabled,
    )
    .await;

    let redis_client = redis::Client::open(config.redis_server.clone()).unwrap();
    let redis_pool = r2d2::Pool::builder().build(redis_client).unwrap();

    setup_heartbeat("heartbeat:distributor".to_owned(), redis_pool);

    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;

    tokio::select! {
        _ = sigint.recv()  => {},
        _ = sigterm.recv() => {},
        _ = distributor.run(tasks_queue.clone()) => {},
    }

    tasks_queue.close().await;

    Ok(())
}
