use std::sync::mpsc::{sync_channel, Receiver, RecvError, SendError, SyncSender};
use super::Duration;

#[derive(Clone)]
pub struct MailSender<T>(SyncSender<T>);

pub struct MailReceiver<T>(Receiver<T>);

impl<T> MailSender<T> {
    pub fn send(&self, item: T) -> Result<(), SendError<T>> {
        self.0.send(item)
    }
}

// TODO merge with cmsis os one
#[derive(Debug)]
pub enum RecvTimeoutError {
    Timeout,
    Other,
}

impl<T> MailReceiver<T> {
    pub fn recv(&self) -> Result<T, RecvError> {
        self.0.recv()
    }

    pub fn recv_timeout(&self, timeout: Duration) -> Result<T, RecvTimeoutError> {
        self.0.recv_timeout(timeout.into())
            .map_err(|e| match e {
                std::sync::mpsc::RecvTimeoutError::Timeout => RecvTimeoutError::Timeout,
                _ => RecvTimeoutError::Other,
            })
    }
}

// TODO use ! for error type when exhaustive patterns is available. See #35121
pub fn mail_queue<T>(size: u32) -> Result<(MailSender<T>, MailReceiver<T>), ()> {
    let (sender, receiver) = sync_channel(size as usize);
    return Ok((MailSender(sender), MailReceiver(receiver)));
}
