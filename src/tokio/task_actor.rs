use async_trait::async_trait;
//use futures::Future;

use tokio::task::{self, JoinHandle, spawn_blocking, JoinError};
//use futures::{executor::block_on, FutureExt};
use std::{marker::PhantomData, sync::Arc, panic::UnwindSafe};

use crate::ActorStateAsync;

///
/// A task based actor.
/// 
pub struct TaskActor
{
}

impl TaskActor where
{

    pub fn spawn<ST>(state: ST) -> JoinHandle<()>
        where ST: ActorStateAsync + Send + 'static
    {
        
        tokio::spawn(async move {
    
            TaskActor::run(state).await;

        })

    }

    async fn run<ST>(mut state: ST)
        where ST: ActorStateAsync + Send + 'static
    {

        let mut proceed = true; 
        
        if state.on_started_async().await
        {

            while proceed
            {
                
                proceed = state.run_async().await;
    
            }

        }
        
        state.on_ending_async().await;

    }

}
