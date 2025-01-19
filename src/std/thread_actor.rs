//use async_trait::async_trait;
//use futures::Future;

//use futures::{executor::block_on, FutureExt};
use std::{marker::PhantomData, sync::Arc, panic::UnwindSafe};

use crate::ActorState;

use std::thread::{self, JoinHandle};

///
/// An std::Thread based actor.
///
#[allow(dead_code)]
pub struct ThreadActor
{
}

impl ThreadActor
{

    pub fn spawn<ST>(state: ST) -> JoinHandle<()>
        where ST: ActorState + Send + 'static
    {
        
        thread::spawn(move || {
    
            ThreadActor::run(state);

        })

    }

    fn run<ST>(mut state: ST)
        where ST: ActorState + Send + 'static
    {

        let mut proceed = true;
        
        if state.on_started()
        {

            while proceed
            {
                
                proceed = state.run();
    
            }

        }

        state.on_ending();

    }  
    
}
