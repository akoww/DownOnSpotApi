use axum::{
	Router, response::IntoResponse,    middleware::map_response,response::Response,
};
use axum::http::HeaderMap;
use tower::ServiceExt;
use std::net::SocketAddr;
use tower_http::{
	services::{ServeDir},
	trace::TraceLayer,
};

pub async fn start_static_server() {
	tokio::join!(serve(using_serve_dir(), 3001));
}

async fn set_header<B>(mut response: Response<B>) -> Response<B> {
	response.headers_mut().insert("Access-Control-Allow-Origin", "*".parse().unwrap());
	response.headers_mut().insert("Access-Control-Allow-Methods", "OPTIONS, DELETE, POST, GET, PATCH, PUT".parse().unwrap());
	response.headers_mut().insert("Access-Control-Allow-Headers", "Content-Type".parse().unwrap());
	response
}

fn using_serve_dir() -> Router {
	// serve the file in the "assets" directory under `/assets`
	Router::new()
		.nest_service("/static", ServeDir::new("static"))
		.nest_service("/downloads", ServeDir::new("downloads"))
		.layer(map_response(set_header))
}

async fn serve(app: Router, port: u16) {
	let addr = SocketAddr::from(([127, 0, 0, 1], port));
	let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
	tracing::debug!("listening on {}", listener.local_addr().unwrap());
	axum::serve(listener, app.layer(TraceLayer::new_for_http()))
		.await
		.unwrap();
}
