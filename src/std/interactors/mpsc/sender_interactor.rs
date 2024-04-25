use std::sync::mpsc::{Sender, Receiver};

use crate::{impl_actor_interactor, impl_interactor_clone, impl_new_sender, impl_pub_sender, ActorInteractor};

///
/// An interactor containing an mpsc sender.
/// 
pub struct SenderInteractor<T: Default>
{

    sender: Sender<T>

}

impl<T: Default> SenderInteractor<T>
{

    impl_new_sender!(Sender<T>);

    impl_pub_sender!(Sender<T>);

}

impl_interactor_clone!(SenderInteractor<T>);

impl_actor_interactor!(SenderInteractor<T>);

///
/// Calls std::sync::mpsc::channel and returns a SenderInteractor in additon to the std receiver.
/// 
pub fn channel<T: Default>() -> (SenderInteractor<T>, Receiver<T>)
{

    let res = std::sync::mpsc::channel();

    (SenderInteractor::new(res.0), res.1)

}



