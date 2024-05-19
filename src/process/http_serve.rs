use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};

use tokio::fs;
use tower_http::{compression::CompressionLayer, services::ServeDir};
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on port {}", path, port);
    let shared_state = Arc::new(HttpServeState { path: path.clone() });
    let dir_service = ServeDir::new(path)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_zstd()
        .append_index_html_on_directories(true);
    let app = Router::new()
        .route("/", get(index_handler))
        .nest_service("/tower", dir_service)
        .route("/*path", get(file_handler))
        .layer(
            CompressionLayer::new()
                .br(true)
                .deflate(true)
                .gzip(true)
                .zstd(true),
        )
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
    // TODO: test p is a directory
    // if it is a directory, list all files/subdirectories
    // as <li><a href="/path/to/file">file name</a></li>
    // <html><body><ul>...</ul></body></html>
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });

        let (status, response) =
            file_handler(State(state), Path(PathBuf::from("Cargo.toml"))).await;

        println!("response: {}", response);
        assert_eq!(status, StatusCode::OK);
        assert!(response.contains("[package]"))
    }
}
