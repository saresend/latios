use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::process::Command;
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

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum SessionType {
    #[default]
    OpenCode,
}

#[derive(Serialize, Deserialize)]
pub struct SessionInput {
    #[serde(default = "SessionType::default")]
    session_type: SessionType,
    title: String,
    spec_file: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionHandle {
    session_id: String,
}

impl SessionInput {
    pub fn new(title: String, spec_file: String) -> Self {
        Self {
            session_type: SessionType::OpenCode,
            title,
            spec_file,
        }
    }
    fn instantiate(&self) -> SessionHandle {
        let output = Command::new("wezterm")
            .args(["cli", "spawn"])
            .output()
            .expect("failed to execute wezterm cli spawn");

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            panic!("wezterm cli spawn failed: {}", stderr.trim());
        }

        let pane_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if pane_id.is_empty() {
            panic!("wezterm cli spawn returned empty pane id");
        }

        SessionHandle {
            session_id: pane_id,
        }
    }
}

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let root = warp::path::end()
        .and(warp::get())
        .map(|| "Latios HTTP server");

    let health = warp::path("health")
        .and(warp::get())
        .map(|| warp::reply::json(&HealthResponse { status: "ok" }));

    let new_session = warp::path("new")
        .and(warp::body::json())
        .map(|session_input: SessionInput| warp::reply::json(&session_input.instantiate()));

    root.or(health).or(new_session)
}

pub async fn run(config: ServerConfig) {
    let routes = routes();
    warp::serve(routes).run(config.addr).await;
}
