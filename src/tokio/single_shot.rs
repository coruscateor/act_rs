use tokio::sync::mpsc::{Sender, Receiver};

/// A bounded channel which has a size of one.
/// 
/// A reusable oneshot, basically.
pub fn channel<T>() -> (Sender<T>, Receiver<T>)
{

    tokio::sync::mpsc::channel(1)

}

