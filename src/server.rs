use serde::Serialize;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use warp::Filter;

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub addr: SocketAddr,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080),
        }
    }
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let root = warp::path::end().and(warp::get()).map(|| "Latios HTTP server");
    let health = warp::path("health")
        .and(warp::get())
        .map(|| warp::reply::json(&HealthResponse { status: "ok" }));

    root.or(health)
}

pub async fn run(config: ServerConfig) {
    let routes = routes();
    warp::serve(routes).run(config.addr).await;
}
