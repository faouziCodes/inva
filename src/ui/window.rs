use anyhow::anyhow;
use std::{
    io::{stdout, Stdout},
    sync::mpsc::{self, Sender},
};

use super::component::Component;

pub enum WindowEvent {
    DrawComponent {
        component: Box<(dyn Component + Send)>,
    },
}

pub struct Window {
    sender: Sender<WindowEvent>,
}

pub enum DrawText {}

impl Window {
    pub fn new() -> Self {
        let (sender, recv) = mpsc::channel();
        let mut stdout = stdout();

        tokio::spawn(async move {
            while let Ok(event) = recv.recv() {
                Self::handle_event(&mut stdout, event)
            }
        });

        Self { sender }
    }

    pub fn send_event(&self, event: WindowEvent) -> anyhow::Result<()> {
        let send_event = self.sender.send(event);

        if send_event.is_err() {
            return Err(anyhow!(
                "Event could not be sent. Window has stoped listening."
            ));
        }

        Ok(())
    }

    fn handle_event(stdout: &mut Stdout, event: WindowEvent) {
        match event {
            WindowEvent::DrawComponent { component } => {
                component.draw(stdout);
            }
        }
    }
}
