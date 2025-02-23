use std::{clone, default, sync::{mpsc::Sender, Arc}, time::Duration};

use crossterm::event::{Event as CrosstermEvent, KeyEvent, MouseEvent};
use futures::{FutureExt, StreamExt};
use tokio::sync::{broadcast, mpsc::{self, Receiver}, watch::{self, }};

use crate::{application::AppResult};

/// Terminal events.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),

  
}
// #[derive(Default, Clone)]
// pub struct AppState {
//   cpu_state: CpuState
// }

/// Terminal event handler.
#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    sender: mpsc::UnboundedSender<Event>,
    /// Event receiver channel.
    receiver: mpsc::UnboundedReceiver<Event>,
    // /// Appstate receiver
    // appstate_receiver:  mpsc::UnboundedReceiver<AppState>,
    /// Event handler thread.
    handler: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
  
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::unbounded_channel();
        // let (appstate_sender, appstate_receiver) = mpsc::unbounded_channel();
        // let _appsatate_sender = appstate_sender.clone();
        let _sender = sender.clone();
        let handler = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut tick = tokio::time::interval(tick_rate);
            loop {
                let tick_delay = tick.tick();
                let crossterm_event = reader.next().fuse();
                // needs refactoring
                // let recv = rx.recv();
                // let mut _recv = tx.subscribe();

                tokio::select! {
                  // val = app.cpu_state.send("value") => {
              
                  // }

                  _ = _sender.closed() => { // close sender to avoid hanging 
                    break;
                  }
                  // _ = recv => {
            
                  //   appstate_sender.send(_recv.recv().await.unwrap_or(AppState::default())).unwrap()
                  // }
                  _ = tick_delay => {
                    _sender.send(Event::Tick).unwrap(); // send tick event for update
                  }
                  Some(Ok(evt)) = crossterm_event => { // send events receive
                    match evt {
                      CrosstermEvent::Key(key) => {
                        if key.kind == crossterm::event::KeyEventKind::Press {
                          _sender.send(Event::Key(key)).unwrap();
                        }
                      },
                      CrosstermEvent::Mouse(mouse) => {
                        _sender.send(Event::Mouse(mouse)).unwrap();
                      },
                      CrosstermEvent::Resize(x, y) => {
                        _sender.send(Event::Resize(x, y)).unwrap();
                      },
                      CrosstermEvent::FocusLost => {
                      },
                      CrosstermEvent::FocusGained => {
                      },
                      CrosstermEvent::Paste(_) => {
                      },
                    }
                  }
                };
            }
        });
        Self {
            sender,
            receiver,
            handler,
            // appstate_receiver,
        }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub async fn next(&mut self) -> AppResult<Event> {
        self.receiver
            .recv()
            .await
            .ok_or(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "This is an IO error",
            )))
    }
}