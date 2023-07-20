use tokio::sync::oneshot::error::TryRecvError;
use tokio::sync::oneshot::{Sender as TkSender, Receiver as TkReceiver, channel as tk_channel};

use std::sync::atomic::{AtomicBool, Ordering};

use std::sync::Arc;

pub struct Sender<T>
{

    sender: TkSender<T>,
    has_sent: Arc<AtomicBool>

}

impl<T> Sender<T>
{

    pub fn new(sender: TkSender<T>, has_sent: Arc<AtomicBool>) -> Self
    {

        Self
        {

            sender,
            has_sent

        }

    }

    pub fn has_sent(&self) -> bool
    {

        self.has_sent.load(Ordering::SeqCst)

    }

    pub fn send(self, value: T) -> Result<(), T>
    {

        let res = self.sender.send(value);

        self.has_sent.store(true, Ordering::SeqCst);

        res

    }

}

pub struct Receiver<T>
{

    reciver: TkReceiver<T>,
    has_sent: Arc<AtomicBool>

}

impl<T> Receiver<T>
{

    pub fn new(reciver: TkReceiver<T>, has_sent: Arc<AtomicBool>) -> Self
    {

        Self
        {

            reciver,
            has_sent

        }

    }

    pub fn has_recived(&self) -> bool
    {

        self.has_sent.load(Ordering::SeqCst)

    }

    pub fn try_recv(&mut self) -> Option<Result<T, TryRecvError>>
    {

        if self.has_recived()
        {

            return Some(self.reciver.try_recv()) //.blocking_recv())

        }

        None

    }

}

pub fn channel<T>() -> (Sender<T>, Receiver<T>)
{

    let res = tokio::sync::oneshot::channel();

    let has_sent = Arc::new(AtomicBool::new(false));

    (Sender::new(res.0, has_sent.clone()), Receiver::new(res.1, has_sent))

}

