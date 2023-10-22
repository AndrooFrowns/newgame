use tracing::{trace, info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::event::event::{Event, EventDispatcher, KeyEvent};

pub trait App {
    fn run(&self) {
        init_logging();

        let e = Event::Key(KeyEvent::Pressed { keycode: 3, repeat_count: 1 });

        let ed = EventDispatcher::new(e);

        trace!("event trace: {e:?}");
        trace!("event trace: {ed:?}");


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
