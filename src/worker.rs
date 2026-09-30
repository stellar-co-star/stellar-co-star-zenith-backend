#[tokio::main]
async fn main() {
    zenith_stellar-co-star-zenith-stellar-co-star-zenith-backend::init_tracing();
    let state = zenith_stellar-co-star-zenith-stellar-co-star-zenith-backend::init_worker_state().await;
    tracing::info!("Zenith background worker started");
    tokio::select! {
        _ = zenith_stellar-co-star-zenith-stellar-co-star-zenith-backend::jobs::run_loop(state) => {}
        _ = zenith_stellar-co-star-zenith-stellar-co-star-zenith-backend::shutdown_signal() => {
            tracing::info!("Zenith background worker shutting down");
        }
    }
}
