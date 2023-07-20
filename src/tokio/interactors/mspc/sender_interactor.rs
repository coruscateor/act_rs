use tokio::sync::mpsc::{Sender, Receiver};

use crate::ActorInteractor;

use futures::executor::block_on;

///
/// An interactor containing an mspc sender.
/// 
pub struct SenderInteractor<T: Default>
{

    sender: Sender<T>

}

impl<T: Default> SenderInteractor<T>
{

    pub fn new(sender: Sender<T>) -> Self
    {

        Self
        {

            sender

        }
        
    }

    pub fn get_sender_ref(&self) -> &Sender<T>
    {

        &self.sender

    }

}

impl<T: Default> Clone for SenderInteractor<T>
{

    fn clone(&self) -> Self
    {

        Self
        {

            sender: self.sender.clone()

        }

    }

}

impl<T: Default> ActorInteractor for SenderInteractor<T>
{

    fn input_default(&self)
    {

        _ = block_on(self.sender.send(T::default()));

    }

}

///
/// Calls tokio::sync::mpsc::channel and returns an SenderInteractor in additon to the Tokio receiver.
/// 
pub fn channel<T: Default>(buffer: usize) -> (SenderInteractor<T>, Receiver<T>)
{

    let res = tokio::sync::mpsc::channel(buffer);

    (SenderInteractor::new(res.0), res.1)

}



