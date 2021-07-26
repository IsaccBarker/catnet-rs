use super::{Registrar, super::message::Message};

impl Registrar {
    pub fn test_connection(&mut self, message: Message) -> Message {
        let mut snd = Message::new(message.command);
        snd.response.push("received successfully".to_owned());

        snd
    }
}

