use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

pub trait App {
    fn run(&self) {
        init_logging();

        loop {
            info!("loop start");
            todo!("implement primary loop");
        }
    }
}

fn init_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default logging subscriber failed");
}
