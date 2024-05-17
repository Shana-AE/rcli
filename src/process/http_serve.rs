use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};

use tokio::fs;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on port {}", path, port);
    let shared_state = Arc::new(HttpServeState { path });
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/*path", get(file_handler))
        .with_state(shared_state);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index_handler(State(state): State<Arc<HttpServeState>>) -> String {
    format!("{:?}", state)
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(p): Path<PathBuf>,
) -> (StatusCode, String) {
    format!("visit state: {:?} path: {:?} ", state.path, p);
    let path = state.path.join(p);
    info!("Client Requesting {:?}", path);
    if !path.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("directory not found: {}", path.display()),
        )
    } else {
        match fs::read_to_string(&path).await {
            Ok(content) => (StatusCode::OK, content),
            Err(e) => {
                warn!("error reading file {}", &path.display());
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}
