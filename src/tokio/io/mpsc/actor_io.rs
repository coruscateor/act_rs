use std::{sync::{Arc, Mutex, MutexGuard, PoisonError}};

use tokio::sync::mpsc::{channel, unbounded_channel, Receiver, Sender, UnboundedReceiver, UnboundedSender};

//ActorIOClient, ActorIOServer

///
/// For use on the “client side” of the actor.
/// 
pub struct ActorIOClient<IM, OM>          
{

    actor_input_sender: Sender<IM>,
    actor_output_receiver: Arc<Mutex<Receiver<OM>>>,

}

impl<IM, OM> ActorIOClient<IM, OM>
{

    pub fn new(actor_input_sender: Sender<IM>, actor_output_receiver: Receiver<OM>) -> Self
    {

        Self
        {

            actor_input_sender,
            actor_output_receiver: Arc::new(Mutex::new(actor_output_receiver))

        }

    }

    pub fn input_sender(&self) -> &Sender<IM>
    {

        &self.actor_input_sender

    }

    pub fn output_receiver_lock(&self) -> Result<MutexGuard<Receiver<OM>>, PoisonError<MutexGuard<Receiver<OM>>>>
    {

        self.actor_output_receiver.lock()

    }

}

impl<IM, OM> Clone for ActorIOClient<IM, OM>
{

    fn clone(&self) -> Self
    {

        Self
        {
            
            actor_input_sender: self.actor_input_sender.clone(),
            actor_output_receiver: self.actor_output_receiver.clone()
            
        }

    }

}

///
/// For use on the “server side” of the actor.
/// 
pub struct ActorIOServer<IM, OM>
{

    actor_input_receiver: Receiver<IM>,
    actor_output_sender: Sender<OM>,

}

impl<IM, OM> ActorIOServer<IM, OM>
{

    pub fn new(actor_input_receiver: Receiver<IM>, actor_output_sender: Sender<OM>) -> Self
    {

        Self
        {

            actor_input_receiver,
            actor_output_sender

        }

    }

    pub fn input_receiver(&mut self) -> &mut Receiver<IM>
    {

        &mut self.actor_input_receiver

    }

    pub fn output_sender(&self) -> &Sender<OM>
    {

        &self.actor_output_sender

    }

}

///
/// Initialises bounded input and output channels.
/// 
/// Keep the ActorIOClient and put the ActorIOServer server object in the actor state.
/// 
pub fn actor_io<IM, OM>(input_buffer_size: usize, output_buffer_size: usize) -> (ActorIOClient<IM, OM>, ActorIOServer<IM, OM>)
{

    let (actor_input_sender,actor_input_receiver) = channel(input_buffer_size);

    let (actor_output_sender,actor_output_receiver) = channel(output_buffer_size);

    (ActorIOClient::new(actor_input_sender, actor_output_receiver), ActorIOServer::new(actor_input_receiver, actor_output_sender))

}

//UnboundedActorIOClient, UnboundedActorIOServer

///
/// For use on the “client side” of the actor.
/// 
pub struct UnboundedActorIOClient<IM, OM>
{

    actor_input_sender: UnboundedSender<IM>,
    actor_output_receiver: Arc<Mutex<UnboundedReceiver<OM>>>,

}

impl<IM, OM> UnboundedActorIOClient<IM, OM>
{

    pub fn new(actor_input_sender: UnboundedSender<IM>, actor_output_receiver: UnboundedReceiver<OM>) -> Self
    {

        Self
        {

            actor_input_sender,
            actor_output_receiver: Arc::new(Mutex::new(actor_output_receiver))

        }

    }

    pub fn input_sender(&self) -> &UnboundedSender<IM>
    {

        &self.actor_input_sender

    }

    pub fn output_receiver_lock(&self) -> Result<MutexGuard<UnboundedReceiver<OM>>, PoisonError<MutexGuard<UnboundedReceiver<OM>>>>
    {

        self.actor_output_receiver.lock()

    }

}

impl<IM, OM> Clone for UnboundedActorIOClient<IM, OM>
{

    fn clone(&self) -> Self
    {

        Self
        {
            
            actor_input_sender: self.actor_input_sender.clone(),
            actor_output_receiver: self.actor_output_receiver.clone()

        }

    }

}

///
/// For use on the “server side” of the actor.
/// 
pub struct UnboundedActorIOServer<IM, OM>
{

    actor_input_receiver: UnboundedReceiver<IM>,
    actor_output_sender: UnboundedSender<OM>

}

impl<IM, OM> UnboundedActorIOServer<IM, OM>
{

    pub fn new(actor_input_receiver: UnboundedReceiver<IM>, actor_output_sender: UnboundedSender<OM>) -> Self
    {

        Self
        {

            actor_input_receiver,
            actor_output_sender

        }

    }

    pub fn input_receiver(&mut self) -> &mut UnboundedReceiver<IM>
    {

        &mut self.actor_input_receiver

    }

    pub fn output_sender(&self) -> &UnboundedSender<OM>
    {

        &self.actor_output_sender

    }

}

///
/// Initialises unbounded input and output channels.
/// 
/// Keep the UnboundedActorIOClient and put the UnboundedActorIOServer server object in the actor state.
/// 
pub fn unbounded_actor_io<IM, OM>() -> (UnboundedActorIOClient<IM, OM>, UnboundedActorIOServer<IM, OM>)
{

    let (actor_input_sender,actor_input_receiver) = unbounded_channel();

    let (actor_output_sender,actor_output_receiver) = unbounded_channel();

    (UnboundedActorIOClient::new(actor_input_sender, actor_output_receiver), UnboundedActorIOServer::new(actor_input_receiver, actor_output_sender))

}

//Mixed actor IO inter-actors...

