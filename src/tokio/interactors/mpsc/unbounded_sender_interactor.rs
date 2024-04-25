
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver};

use crate::{impl_actor_interactor, impl_interactor_clone, impl_new_sender, impl_pub_sender, ActorInteractor};

///
/// An interactor containing an unbounded mspc sender.
///
pub struct UnboundedSenderInteractor<T: Default>
{

    sender: UnboundedSender<T>

}

impl<T: Default> UnboundedSenderInteractor<T>
{

    /*
    pub fn new(sender: UnboundedSender<T>) -> Self
    {

        Self
        {

            sender

        }
        
    }
    */

    impl_new_sender!(UnboundedSender<T>);

    /*
    pub fn sender(&self) -> &UnboundedSender<T>
    {

        &self.sender

    }
    */

    impl_pub_sender!(UnboundedSender<T>);


}

/*
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
*/

impl_interactor_clone!(UnboundedSenderInteractor<T>);

/*
impl<T: Default> ActorInteractor for UnboundedSenderInteractor<T>
{

    fn input_default(&self)
    {

        _ = self.sender.send(T::default());

    }

}
*/

impl_actor_interactor!(UnboundedSenderInteractor<T>);

///
/// Calls tokio::sync::mpsc::unbounded_channel and returns a UnboundedSenderInteractor in additon to the Tokio unbounded receiver.
/// 
pub fn unbounded_channel<T: Default>() -> (UnboundedSenderInteractor<T>, UnboundedReceiver<T>)
{

    let res = tokio::sync::mpsc::unbounded_channel();

    (UnboundedSenderInteractor::new(res.0), res.1)

}

/*

Using #[derive(Clone)]:

the trait bound `T: Clone` is not satisfied
the trait `Clone` is not implemented for `T`, which is required by `UnboundedSenderInteractor<T>: Clone`rustcClick for full compiler diagnostic
unbounded_sender_interactor.rs(11, 10): required for `UnboundedSenderInteractor<T>` to implement `Clone`
actor_state.rs(12, 28): required by a bound in `actor_state::ActorInteractor`
unbounded_sender_interactor.rs(71, 16): consider further restricting this bound: ` + std::clone::Clone`

 */

