use std::sync::mpsc::{SyncSender, Receiver};

use crate::{impl_interactor_clone, impl_new_sender, impl_pub_sender};

///
/// An interactor containing an mpsc sender.
///
#[deprecated(since = "0.3.0", note = "Deprecated due to ActorInteractor being deprecated.")]
pub struct SyncSenderInteractor<T: Default>
{

    sender: SyncSender<T>

}

impl<T: Default> SyncSenderInteractor<T>
{

    impl_new_sender!(SyncSender<T>);

    impl_pub_sender!(SyncSender<T>);

}

impl_interactor_clone!(SyncSenderInteractor<T>);

///
/// Calls std::sync::mpsc::sync_channel and returns a SenderInteractor in additon to the std receiver.
/// 
pub fn sync_channel<T: Default>(buffer: usize) -> (SyncSenderInteractor<T>, Receiver<T>)
{

    let res = std::sync::mpsc::sync_channel(buffer);

    (SyncSenderInteractor::new(res.0), res.1)

}



