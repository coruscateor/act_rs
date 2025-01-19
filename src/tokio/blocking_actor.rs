use async_trait::async_trait;
//use futures::Future;

//use futures::{executor::block_on, FutureExt};
use tokio::task::JoinHandle;
use std::{marker::PhantomData, sync::Arc, panic::UnwindSafe};

use crate::ActorState;

pub struct BlockingActor
{
}

impl BlockingActor
{

    pub fn spawn<ST>(state: ST) -> JoinHandle<()>
        where ST: ActorState + Send + 'static
    {
        
        tokio::task::spawn_blocking(move || {
    
            BlockingActor::run(state);

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
