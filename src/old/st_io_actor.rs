use futures::Future;
use futures::executor::block_on;
use crate::{notifying_queues::*, dropped_indicator::*, actor_state_controller};
use std::{marker::PhantomData, sync::Arc, thread::{self, JoinHandle}, panic::{catch_unwind, UnwindSafe}};

use super::{actor_state_controller::*, actor_joiner_controller::*, actor_joiner::*};

//join_handler::*,

pub struct ST_IO_Actor<SC, IO> where //T, O
    SC: std::marker::Send + 'static,
    IO: ActorIO
{

    jh: JoinHandle<()>,
    public_interface_dropped_indicator: Arc<()>,
    io: Arc<IO>,
    phantom_data: PhantomData<SC>,
    aajc: Arc<ActorJoinerController>

}

//std::thread Input/Output Actor

impl<SC, IO> ST_IO_Actor<SC, IO> where
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

        let jh = thread::spawn(move || {
    
            ST_IO_Actor::run(state_controller, public_interface_dropped_indicator_moved, aajc_moved);

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

        //aajc.set_inactive_relaxed();

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

impl<SC, IO> Drop for ST_IO_Actor<SC, IO> where
    SC: std::marker::Send, //+ 'static + ActorStateController<IO> + UnwindSafe,
    IO: ActorIO
{
    fn drop(&mut self) {
        

        //<Arc<IO> as actor_state_controller::ActorIO>::send_default(&self.io);

        self.io.send_default();

    }
}

/*
impl<SC, IO> JoinHandler for ST_IO_Actor<SC, IO> where
    SC: std::marker::Send + 'static
{

    fn join(&self)
    {

        self.jh.join(); //`self.jh` moved due to this method callrustcE0507

    }

}
*/

