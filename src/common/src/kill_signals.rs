use tokio::signal;
use tracing::info;

pub async fn wait_for_kill_signals() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
                info!("Received ctrl_c!");
        },
        _ = terminate => {
                info!("Received terminate!");
        },
    }
}
