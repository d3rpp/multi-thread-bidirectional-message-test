use std::sync::mpsc::{channel, Receiver, RecvError, SendError, Sender};

pub struct Messenger<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> Messenger<T> {
    pub fn new() -> (Messenger<T>, Messenger<T>) {
        let (sender_l, receiver_l) = channel::<T>();
        let (sender_r, receiver_r) = channel::<T>();

        return (
            Messenger {
                sender: sender_l,
                receiver: receiver_r,
            },
            Messenger {
                sender: sender_r,
                receiver: receiver_l,
            },
        );
    }

    pub fn send(&self, message: T) -> Result<(), SendError<T>> {
        self.sender.send(message)
    }

    pub fn receive(&self) -> Result<T, RecvError> {
        self.receiver.recv()
    }
}
