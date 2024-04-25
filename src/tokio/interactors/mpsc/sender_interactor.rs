use tokio::sync::mpsc::{Sender, Receiver};

use crate::{impl_actor_interactor, impl_interactor_clone, impl_new_sender, impl_pub_sender, ActorInteractor};

//use futures::executor::block_on;

//use crate::macros;

///
/// An interactor containing an mpsc sender.
/// 
pub struct SenderInteractor<T: Default>
{

    sender: Sender<T>

}

impl<T: Default> SenderInteractor<T>
{

    /*
    pub fn new(sender: Sender<T>) -> Self
    {

        Self
        {

            sender

        }
        
    }
    */

    impl_new_sender!(Sender<T>);

    /*
    pub fn sender(&self) -> &Sender<T>
    {

        &self.sender

    }
    */

    impl_pub_sender!(Sender<T>);

}

/*
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
*/

impl_interactor_clone!(SenderInteractor<T>);

/*
impl<T: Default> ActorInteractor for SenderInteractor<T>
{

    fn input_default(&self)
    {

        _ = block_on(self.sender.send(T::default()));

    }

}
*/

impl_actor_interactor!(SenderInteractor<T>);

///
/// Calls tokio::sync::mpsc::channel and returns a SenderInteractor in additon to the Tokio receiver.
/// 
pub fn channel<T: Default>(buffer: usize) -> (SenderInteractor<T>, Receiver<T>)
{

    let res = tokio::sync::mpsc::channel(buffer);

    (SenderInteractor::new(res.0), res.1)

}



