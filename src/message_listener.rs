use std::sync::Arc;
use crate::Error;
use crate::message_handler::ActorMessage;
use tokio::sync::mpsc;
use tracing::instrument;
use crate::animation::Animation;

#[derive(Debug)]
pub struct MessageListener {
    receiver: mpsc::Receiver<ActorMessage>,
    animation: Arc<dyn Animation>,
}

impl MessageListener {
    pub fn new(receiver: mpsc::Receiver<ActorMessage>, animation: Arc<dyn Animation>) -> Self {
        Self {
            receiver,
            animation,
        }
    }

    #[instrument]
    async fn handle_message(&mut self, msg: ActorMessage) -> Result<(), Error> {
        match msg {
            ActorMessage::MotivationReceived {} => {
                tracing::info!("Running task ActorMessage::MotivationReceived...");
            }
        }

        self.animation.animate();
        tracing::info!("Finished task ActorMessage::MotivationReceived...");
        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        while let Some(msg) = self.receiver.recv().await {
            tracing::debug!("MessageListener: received {:?}", &msg);
            self.handle_message(msg).await?;
        }

        Ok(())
    }
}
