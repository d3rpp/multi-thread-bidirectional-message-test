use std::sync::mpsc::{channel, Receiver, RecvError, SendError, Sender};

/// # Messager
/// The purpose of this messager is to allow bidirectional communitcation between threads on a system
///
/// it is quite literally 2 `std::sync::mspc::channel`'s that link 2 things together.
pub struct Messager<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> Messager<T> {
    /// Creates a new Messager Pair that are linked together
    ///
    /// ## Example
    /// ```rust
    /// let (a, b) = Messager::new()
    /// ```

    pub fn new() -> (Messager<T>, Messager<T>) {
        let (sender_l, receiver_l) = channel::<T>();
        let (sender_r, receiver_r) = channel::<T>();

        return (
            Messager {
                sender: sender_l,
                receiver: receiver_r,
            },
            Messager {
                sender: sender_r,
                receiver: receiver_l,
            },
        );
    }

    /// Sends a message to the other Messager instance
    /// returns a `Result<(), SendError<T>>` exactly like the `std::sync::mpsc::channel`
    pub fn send(&self, message: T) -> Result<(), SendError<T>> {
        self.sender.send(message)
    }

    /// Receives a message from the other Messager instance
    /// returns a `Result<T, RecvError>` exactly like the `std::sync::mpsc::channel`
    pub fn receive(&self) -> Result<T, RecvError> {
        self.receiver.recv()
    }
}
