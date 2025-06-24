use tokio::sync::mpsc;

#[derive(Clone, Debug)]
pub enum ActorMessage {
    MotivationReceived {},
}

#[derive(Clone, Debug)]
pub struct MessageHandler {
    sender: mpsc::Sender<ActorMessage>,
}

impl MessageHandler {
    pub fn new(sender: mpsc::Sender<ActorMessage>) -> Self {
        Self { sender }
    }
    pub async fn motivation_received(&self) {
        if (self.sender.send(ActorMessage::MotivationReceived {}).await).is_err() {
            tracing::info!("receiver dropped");
            assert!(self.sender.is_closed());
        }
    }
}
