use futures::{FutureExt, StreamExt};
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};

use crossterm::event::{Event as CrosstermEvent, KeyEvent, KeyEventKind};

pub enum Event {
    Init,
    // Quit,
    Error,
    Render,
    Tick,
    Timer,
    Key(KeyEvent),
    Resize(u16, u16),
}

pub struct Tui {
    frame_rate: f64,
    tick_rate: f64,
    event_rx: UnboundedReceiver<Event>,
    event_tx: UnboundedSender<Event>,
    task: Option<JoinHandle<()>>,
}
impl Tui {
    pub fn new(frame_rate: f64, tick_rate: f64) -> Self {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let task = None;

        Self {
            task,
            frame_rate,
            tick_rate,
            event_rx,
            event_tx,
        }
    }

    pub fn waiting_time_to_sync() {
        use std::{
            thread::sleep,
            time::{Duration, SystemTime, UNIX_EPOCH},
        };
        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

        let millis = since_the_epoch.as_millis();
        let millis_until_next_second = 1000 - (millis % 1000);
        sleep(Duration::from_millis(millis_until_next_second as u64));
    }

    pub fn run(&mut self) {
        let render_delay = std::time::Duration::from_secs_f64(1.0 / self.frame_rate);
        let tick_delay = std::time::Duration::from_secs_f64(1.0 / self.tick_rate);
        let timer_delay = std::time::Duration::from_secs_f64(1.0);
        let event_tx = self.event_tx.clone();
        let do_tick = self.frame_rate != self.tick_rate;

        let task = tokio::spawn(async move {
            macro_rules! tired {
                ($send:expr) => {
                    if event_tx.send($send).is_err() {
                        break;
                    }
                };
            }
            event_tx.send(Event::Init).unwrap();
            Self::waiting_time_to_sync();

            let mut reader = crossterm::event::EventStream::new();
            let mut render_interval = tokio::time::interval(render_delay);
            let mut tick_interval = tokio::time::interval(tick_delay);
            let mut timer_interval = tokio::time::interval(timer_delay);

            loop {
                let tick_delay = tick_interval.tick();
                let render_delay = render_interval.tick();
                let timer_delay = timer_interval.tick();
                let crossterm_event = reader.next().fuse();

                tokio::select! {
                    maybe_event = crossterm_event => {
                        match maybe_event {
                            Some(Ok(evt)) => {
                                match evt {
                                    CrosstermEvent::Key(key) => {
                                            if key.kind == KeyEventKind::Press {
                                                // event_tx.send(Event::Key(key)).unwrap();
                                                tired!(Event::Key(key));
                                            }
                                    },
                                    CrosstermEvent::Resize(w, h) => {
                                        tired!(Event::Resize(w, h));
                                    },
                                    _ => ()
                                }
                            },
                            Some(Err(_)) => {
                                tired!(Event::Error);
                            },
                            _ => (),
                        }
                    },
                    _ = tick_delay => {
                        if do_tick {
                            tired!(Event::Tick);
                        }
                    },
                    _ = render_delay => {
                        tired!(Event::Render);
                    },
                    _ = timer_delay => {
                        tired!(Event::Timer);
                    }
                }
            }
        });

        self.task = Some(task);
    }

    pub async fn next(&mut self) -> Option<Event> {
        self.event_rx.recv().await
    }
}
