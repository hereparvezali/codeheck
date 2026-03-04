use tokio::signal::{self, unix::SignalKind};

pub async fn shutdown_signal() {
    let ctrl_c = signal::ctrl_c();
    let mut terminate = signal::unix::signal(SignalKind::terminate())
        .expect("Failed to install TERMINATE signal handler");
    let mut interrupt = signal::unix::signal(SignalKind::interrupt())
        .expect("Failed to install INTERRUPT signal handler");

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate.recv() => {},
        _ = interrupt.recv() => {},
    }
}
