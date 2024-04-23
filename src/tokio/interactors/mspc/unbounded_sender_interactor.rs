
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver};

use crate::ActorInteractor;

///
/// An interactor containing an unbounded mspc sender.
/// 
pub struct UnboundedSenderInteractor<T: Default>
{

    sender: UnboundedSender<T>

}

impl<T: Default> UnboundedSenderInteractor<T>
{

    pub fn new(sender: UnboundedSender<T>) -> Self
    {

        Self
        {

            sender

        }
        
    }

    pub fn sender(&self) -> &UnboundedSender<T>
    {

        &self.sender

    }

}

impl<T: Default> Clone for UnboundedSenderInteractor<T>
{

    fn clone(&self) -> Self
    {

        Self
        {

            sender: self.sender.clone()

        }

    }

}

impl<T: Default> ActorInteractor for UnboundedSenderInteractor<T>
{

    fn input_default(&self)
    {

        _ = self.sender.send(T::default());

    }

}

///
/// Calls tokio::sync::mpsc::unbounded_channel and returns an UnboundedSenderInteractor in additon to the Tokio unbounded receiver.
/// 
pub fn unbounded_channel<T: Default>() -> (UnboundedSenderInteractor<T>, UnboundedReceiver<T>)
{

    let res = tokio::sync::mpsc::unbounded_channel();

    (UnboundedSenderInteractor::new(res.0), res.1)

}



