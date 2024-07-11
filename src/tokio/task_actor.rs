use async_trait::async_trait;
use futures::Future;

use tokio::task::{self, JoinHandle, spawn_blocking, JoinError};
use futures::{executor::block_on, FutureExt};
use std::{marker::PhantomData, sync::Arc, panic::UnwindSafe};

use crate::{AsyncActorState, ActorFrontend};

///
/// A task based actor.
/// 
pub struct TaskActor
{
}

impl TaskActor where
{

    pub fn spawn<ST>(state: ST) -> JoinHandle<()>
        where ST: AsyncActorState + Send + 'static
    {
        
        tokio::spawn(async move {
    
            TaskActor::run(state).await;

        })

    }

    async fn run<ST>(mut state: ST)
        where ST: AsyncActorState + Send + 'static
    {

        let mut proceed = true; 
        
        if state.beginning_async().await
        {

            while proceed
            {
                
                proceed = state.run_async().await;
    
            }

        }

        state.ending_async().await;

    }

}
