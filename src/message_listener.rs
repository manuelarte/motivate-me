use crate::Error;
use crate::message_handler::ActorMessage;
use tokio::sync::mpsc;
use tracing::instrument;
use crate::raspberrypi_animation::RaspberryPiAnimation;

#[derive(Debug)]
pub struct MessageListener {
    receiver: mpsc::Receiver<ActorMessage>,
    animation: RaspberryPiAnimation
}

impl MessageListener {
    pub fn new(receiver: mpsc::Receiver<ActorMessage>) -> Self {
        Self { receiver, animation: RaspberryPiAnimation::new() }
    }

    #[instrument]
    async fn handle_message(&mut self, msg: ActorMessage) -> Result<(), Error> {
        match msg {
            ActorMessage::MotivationReceived {} => {
                tracing::info!("Running task ActorMessage::MotivationReceived...");
            }
        }

        self.animation.animate("".to_string());
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
