// src/event_queue.rs
use tokio::sync::mpsc;
use crate::event::{Event, EventType};
use tokio::time::{self, Duration};

pub struct EventQueue {
    pub sender: mpsc::Sender<Event>,
    receiver: mpsc::Receiver<Event>,
}

impl EventQueue {
    pub fn new(buffer_size: usize) -> Self {
        let (sender, receiver) = mpsc::channel(buffer_size);
        EventQueue { sender, receiver }
    }

    pub async fn add_event(&self, event: Event) {
        self.sender.send(event).await.unwrap();
    }

    pub async fn process_events(&mut self) {
        while let Some(event) = self.receiver.recv().await {
            self.handle_event(event).await;
        }
    }

    async fn handle_event(&self, event: Event) {
        match event.event_type {
            EventType::MouseMove { x, y } => {
                println!("Mouse moved to: ({}, {})", x, y);
                // Here, you would send the event to the connected systems
            }
            EventType::KeyPress { key_code } => {
                println!("Key pressed: {}", key_code);
                // Here, you would send the event to the connected systems
            }
            EventType::Quit => {
                println!("Received Quit Event: {:?}", event);
                // Handle quit event
            }
        }
    }

    pub async fn init_timer(&self, duration: Duration, event_type: EventType) {
        let sender = self.sender.clone();
        tokio::spawn(async move {
            time::sleep(duration).await;
            sender.send(Event { event_type }).await.unwrap();
        });
    }
}
