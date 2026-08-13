#[tokio::main]
async fn main() {
    zenith_stellar-co-star-zenith-stellar-co-star-zenith-backend::init_tracing();
    let state = zenith_stellar-co-star-zenith-stellar-co-star-zenith-backend::init_state().await;
    let app = zenith_stellar-co-star-zenith-stellar-co-star-zenith-backend::build_router(state);

    let addr = "0.0.0.0:8081";
    println!("Zenith stellar-co-star-zenith-stellar-co-star-zenith-backend listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(zenith_stellar-co-star-zenith-stellar-co-star-zenith-backend::shutdown_signal())
        .await
        .unwrap();
}
