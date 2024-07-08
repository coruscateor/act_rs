use async_trait::async_trait;
use futures::Future;

use tokio::task::{self, JoinHandle, spawn_blocking, JoinError};
use futures::{executor::block_on, FutureExt};
use std::{marker::PhantomData, sync::Arc, panic::UnwindSafe};

use crate::{AsyncActorState, DroppedIndicator, ActorFrontend};

///
/// A task based actor.
/// 
#[allow(dead_code)]
pub struct TaskActor<ST, IN> where
    ST: AsyncActorState<IN> + Send + 'static
{

    interactor: IN,
    phantom_data: PhantomData<ST>

}

impl<ST, IN> TaskActor<ST, IN> where
    ST: AsyncActorState<IN> + Send + 'static
{

    pub fn new(state: ST) -> Self
        where IN: Clone
    {

        let interactor =  state.interactor().clone();
        
        tokio::spawn(async move {
    
            TaskActor::run(state).await;

        });

        Self
        {

            interactor,
            phantom_data: PhantomData::default()

        }

    }

    async fn run(mut state: ST)
    {

        let mut proceed = true; 
        
        if state.on_enter_async().await
        {

            while proceed
            {
                
                proceed = state.run_async().await;
    
            }
    
            state.on_exit_async().await;

        }

    }

}

impl<ST, IN> ActorFrontend<IN> for TaskActor<ST, IN> where
    ST: AsyncActorState<IN> + Send + 'static
{

    fn interactor(&self) -> &IN
    {

        &self.interactor

    }    

}