//use async_trait::async_trait;
//use futures::Future;

//use futures::{executor::block_on, FutureExt};
use std::{marker::PhantomData, sync::Arc, panic::UnwindSafe};

use crate::{ActorState, ActorStateBuilder};

use std::thread::{self, JoinHandle};

///
/// An std::Thread based actor.
///
pub struct ThreadActor
{
}

impl ThreadActor
{

    pub fn spawn<ST>(state: ST) -> JoinHandle<()>
        where ST: ActorState + Send + 'static
    {
        
        thread::spawn(move ||
        {
    
            ThreadActor::run(state);

        })

    }

    pub fn spawn_and_build<ST, STB>(state_builder: STB) -> JoinHandle<()>
        where ST: ActorState + Send + 'static,
              STB: ActorStateBuilder<ST> + Send + 'static
    {
        
        thread::spawn(move ||
        {

            if let Some(state) = state_builder.build()
            {

                ThreadActor::run(state);

            }      

        })

    }

    fn run<ST>(mut state: ST)
        where ST: ActorState + Send + 'static
    {

        let mut proceed = true;
        
        if state.pre_run()
        {

            while proceed
            {
                
                proceed = state.run();
    
            }

        }
        
        state.post_run();

    }  
    
}
