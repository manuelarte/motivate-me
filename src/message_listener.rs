use crate::Error;
use crate::message_handler::ActorMessage;
use tokio::sync::mpsc;

pub struct MessageListener {
    receiver: mpsc::Receiver<ActorMessage>,
}

impl MessageListener {
    pub fn new(receiver: mpsc::Receiver<ActorMessage>) -> Self {
        Self { receiver }
    }

    async fn handle_message(&mut self, msg: ActorMessage) -> Result<(), Error> {
        match msg {
            ActorMessage::MotivationReceived {} => {
                tracing::info!("Running task ActorMessage::MotivationReceived...");
            }
        }

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
