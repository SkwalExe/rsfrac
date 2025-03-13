use std::sync::mpsc::Sender;

use crate::app::SlaveMessage;

/// This trait will only be implemented for Option<&Sender<SlaveMessage>>
/// .send() is given a SlaveMessage and sends it only if the option actually contains a sender.
pub(crate) trait SendSlaveMessage {
    fn send(&self, msg: SlaveMessage) -> Result<(), String>;
}

impl SendSlaveMessage for Option<&Sender<SlaveMessage>> {
    fn send(&self, msg: SlaveMessage) -> Result<(), String> {
        if let Some(sender) = self {
            sender
                .send(msg)
                .map_err(|err| format!("Could not open message channel: {err}"))?;
        }

        Ok(())
    }
}
