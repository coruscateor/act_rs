use async_trait::async_trait;
use futures::Future;
//use tokio::sync::mpsc::{unbounded_channel, UnboundedSender, UnboundedReceiver};
//use super::actor_workers::AsyncActorWorkerNoReturns;
use tokio::task::{self, JoinHandle, spawn_blocking, JoinError};
use futures::{executor::block_on, FutureExt};
use crate::{notifying_queues::*, dropped_indicator::*}; //, actor_workers::AsyncDonNothingNoReturns};
use std::{marker::PhantomData, sync::Arc, panic::UnwindSafe};
//use super::InOutData_NotifyingArrayQueue;
//use super::InOutData;

use super::{actor_state_controller::*, actor_joiner_controller::*, actor_joiner::*}; //join_handler::*};

use std::sync::atomic::{AtomicBool, Ordering};

pub struct TS_IO_Actor<SC, IO> where //T, O
    //T: std::marker::Send + 'static, 
    //O: std::marker::Send + 'static
    SC: std::marker::Send + 'static,
    IO: ActorIO
{

    jh: JoinHandle<()>,
    //public_interface_dropped_indicator: Arc<AtomicBool>,
    public_interface_dropped_indicator: Arc<()>,
    io: Arc<IO>,
    phantom_data: PhantomData<SC>,
    //future mutex?
    aajc: Arc<ActorJoinerController>

}

//tokio::spawn Input/Output Actor

impl<SC, IO> TS_IO_Actor<SC, IO> where
    SC: std::marker::Send + 'static + AsyncActorStateController<IO> + UnwindSafe, //ActorStateController<IO>
    IO: ActorIO
{

    pub fn new(state_controller: SC) -> Self //, input_output: IO)
    {

        //let boxed_sc = Box::new(state_controller);

        let public_interface_dropped_indicator = Arc::new(()); //(AtomicBool::new(true));

        let public_interface_dropped_indicator_moved = public_interface_dropped_indicator.clone();

        //let io = boxed_sc.get_IO();

        let io = state_controller.get_IO();

        let aajc = Arc::new(ActorJoinerController::new(state_controller.get_which_paniced_handler()));

        let aajc_moved = aajc.clone();

        let jh = tokio::spawn(async move {
    
            TS_IO_Actor::run(state_controller, public_interface_dropped_indicator_moved,aajc_moved).await; //boxed_sc, public_interface_dropped_indicator_moved).await;

        });

        Self
        {

            jh,
            public_interface_dropped_indicator,
            io,
            phantom_data: PhantomData::default(),
            aajc

        }
        

    }

    async fn run(mut state_controller: SC, public_interface_dropped_indicator: Arc<()>, aajc: Arc<ActorJoinerController>) //mut state_controller: Box<SC>, public_interface_dropped_indicator: Arc<AtomicBool>)
    {
        /*

        let mut proceed_regardless = true; 

        while proceed_regardless || Arc::strong_count(&public_interface_dropped_indicator) > 1
        {
            
            proceed_regardless = state_controller.run_async().await;

            //Does yeild need to be called here?

            //tokio::task::yield_now().await;
            
        }
        */

        //public_interface_dropped_indicator.store(false, Ordering::Relaxed);

        //async-trait doesn't play well with FutureExt::catch_unwind; so not panic catching
        
        //let result = async move {

        let mut proceed = true; 

        let di = DroppedIndicator::new(&public_interface_dropped_indicator);

        //let result = 
        
        state_controller.on_enter_async().await;

        //result.

        while proceed //|| Arc::strong_count(&public_interface_dropped_indicator) > 1
        {
            
            proceed = state_controller.run_async(&di).await;

        }

        state_controller.on_exit_async().await;

        //}.catch_unwind().await;

        aajc.set_inactive_relaxed();

    }

    pub fn get_IO(&self) -> &IO
    {

        self.io.as_ref()

    }

    /*
    pub fn clone_IO(&self) -> Arc<IO>
    {

        self.io.clone()

    }
    */

    /*
    pub fn get_join_handle(&self) -> &JoinHandle<()>
    {

        &self.jh

    }
    */

    /*
    pub fn is_active(&self) -> bool
    {

        self.public_interface_dropped_indicator.load(Ordering::Relaxed)

    }
    */

    /*
    pub fn public_interface_has_dropped(&self) -> bool
    {

        Arc::strong_count(&self.public_interface_dropped_indicator) == 1

    }
    */

    pub fn get_actor_joiner(&self) -> ActorJoiner
    {

        ActorJoiner::new(&self.aajc)

    }

    pub fn get_is_acive(&self) -> bool
    {

        self.aajc.get_is_active_relaxed()

    }

}

impl<SC, IO> Drop for TS_IO_Actor<SC, IO> where
    SC: std::marker::Send,
    IO: ActorIO
{
    fn drop(&mut self) {
        

        self.io.send_default();

    }
}

/*
#[async_trait]
impl<SC, IO> TokioJoinHandler for TS_IO_Actor<SC, IO> where
    SC: std::marker::Send + 'static
{

    //#[async_trait]
    async fn join_async(&self) -> Result<(), JoinError> //+ std::marker::Send
    {

        self.jh.await

    }

    fn is_finished(&self) -> bool {
        todo!()
    }
}
*/

    