use futures::Future;
use tokio::task::{self, JoinHandle, spawn_blocking};
use futures::executor::block_on;
use crate::{notifying_queues::*, dropped_indicator::*};
use std::{marker::PhantomData, sync::Arc, panic::{catch_unwind, UnwindSafe}};

use super::{actor_state_controller::*, actor_joiner_controller::*, actor_joiner::*};

pub struct TTSB_IO_Actor<SC, IO> where //T, O
    SC: std::marker::Send + 'static,
    IO: ActorIO
{

    jh: JoinHandle<()>,
    public_interface_dropped_indicator: Arc<()>,
    io: Arc<IO>,
    phantom_data: PhantomData<SC>,
    aajc: Arc<ActorJoinerController>

}

//tokio::task::spawn_blocking Input/Output Actor

impl<SC, IO> TTSB_IO_Actor<SC, IO> where
    SC: std::marker::Send + 'static + ActorStateController<IO> + UnwindSafe,
    IO: ActorIO
{

    pub fn new(state_controller: SC) -> Self
    {

        let public_interface_dropped_indicator = Arc::new(());

        let public_interface_dropped_indicator_moved = public_interface_dropped_indicator.clone();

        let io = state_controller.get_IO();

        let aajc = Arc::new(ActorJoinerController::new(state_controller.get_which_paniced_handler()));

        let aajc_moved = aajc.clone();

        let jh = tokio::task::spawn_blocking(move || {
    
            TTSB_IO_Actor::run(state_controller, public_interface_dropped_indicator_moved, aajc_moved);

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

    fn run(mut state_controller: SC, public_interface_dropped_indicator: Arc<()>, aajc: Arc<ActorJoinerController>)
    {

        /*
        let mut proceed_regardless = true; 

        while proceed_regardless || Arc::strong_count(&public_interface_dropped_indicator) > 1
        {
            
            proceed_regardless = state_controller.run();

        }
        */

        let result = catch_unwind(move ||
        {

            let mut proceed = true; 

            let di = DroppedIndicator::new(&public_interface_dropped_indicator);
    
            state_controller.on_enter();
    
            while proceed //|| Arc::strong_count(&public_interface_dropped_indicator) > 1
            {
                
                proceed = state_controller.run(&di);
    
            }
    
            state_controller.on_exit();

        });

        match result
        {

            Ok(_val) =>
            {

                aajc.set_inactive_relaxed();

            }
            Err(err) =>
            {

                aajc.set_error_relaxed(err);

            }
            
        }

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

    pub fn get_actor_joiner(&self) -> ActorJoiner
    {

        ActorJoiner::new(&self.aajc)

    }

    pub fn get_is_acive(&self) -> bool
    {

        self.aajc.get_is_active_relaxed()

    }


}

impl<SC, IO> Drop for TTSB_IO_Actor<SC, IO> where
    SC: std::marker::Send,
    IO: ActorIO
{
    fn drop(&mut self) {
        

        self.io.send_default();

    }
}


