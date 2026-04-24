use redis::aio::ConnectionManager;

use crate::config::Redis;

pub async fn create_redis(cfg: &Redis) -> anyhow::Result<ConnectionManager> {
    let client = redis::Client::open(cfg.url.as_str())?;
    let mgr = ConnectionManager::new(client).await?;
    Ok(mgr)
}
