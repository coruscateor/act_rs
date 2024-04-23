use async_trait::async_trait;
use futures::Future;

use tokio::task::{self, JoinHandle, spawn_blocking, JoinError};
use futures::{executor::block_on, FutureExt};
use std::{marker::PhantomData, sync::Arc, panic::UnwindSafe};


use crate::{ActorInteractor, AsyncActorState, DroppedIndicator, ActorFrontend};

///
/// A task based actor.
/// 
/// BROKEN: create your type using impl_mac_task_actor
/// 
pub struct TaskActor<ST, IN> where
    ST: std::marker::Send + 'static,
    IN: ActorInteractor
{

    interactor: IN,
    phantom_data: PhantomData<ST>,
    dropped_indicator: Arc<()>

}



impl<ST, IN> TaskActor<ST, IN> where
    ST: std::marker::Send + 'static + AsyncActorState<IN>,
    IN: ActorInteractor
{

    pub fn new(state: ST) -> Self
    {

        let interactor =  state.interactor();

        let dropped_indicator = Arc::new(());

        let dropped_indicator_moved = dropped_indicator.clone();
        
        tokio::spawn(async move {
    
            TaskActor::run(state, dropped_indicator_moved).await;

        });

        Self
        {

            interactor,
            phantom_data: PhantomData::default(),
            dropped_indicator,

        }

    }

    async fn run(mut state: ST, dropped_indicator: Arc<()>)
    {

        let mut proceed = true; 

        let di = DroppedIndicator::new(dropped_indicator);
        
        if state.on_enter_async(&di).await
        {

            while proceed
            {
                
                proceed = state.run_async(&di).await;
    
            }
    
            state.on_exit_async(&di).await;

        }

    }

}

impl<IN, SC> ActorFrontend<IN> for TaskActor<SC, IN>
    where IN: ActorInteractor, SC: Send
{

    fn interactor(&self) -> &IN
    {

        &self.interactor

    }    

}

impl<SC, IN> Drop for TaskActor<SC, IN> where
    SC: std::marker::Send,
    IN: ActorInteractor
{

    fn drop(&mut self)
    {
        
        self.interactor.input_default();

    }

}
    