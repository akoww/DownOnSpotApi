use axum::{
	extract::Request, handler::HandlerWithoutStateExt, http::StatusCode, routing::get, Router,
};
use std::net::SocketAddr;
use tower_http::{
	services::{ServeDir, ServeFile},
	trace::TraceLayer,
};

pub async fn start_static_server() {
	tokio::join!(serve(using_serve_dir(), 3001));
}

fn using_serve_dir() -> Router {
	// serve the file in the "assets" directory under `/assets`
	Router::new()
		.nest_service("/static", ServeDir::new("static"))
		.nest_service("/downloads", ServeDir::new("downloads"))
}

async fn serve(app: Router, port: u16) {
	let addr = SocketAddr::from(([127, 0, 0, 1], port));
	let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
	tracing::debug!("listening on {}", listener.local_addr().unwrap());
	axum::serve(listener, app.layer(TraceLayer::new_for_http()))
		.await
		.unwrap();
}
