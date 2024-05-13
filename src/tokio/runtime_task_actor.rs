use async_trait::async_trait;
use futures::Future;
use tokio::{task::{self, JoinHandle, spawn_blocking, JoinError}, runtime::{Handle, Runtime}};
use futures::{executor::block_on, FutureExt};
use std::{marker::PhantomData, sync::Arc};

use crate::{ActorInteractor, AsyncActorState, DroppedIndicator, ActorFrontend};

///
/// A Task based actor that requres a runtime or a runtime handle to get started.
/// 
#[allow(dead_code)]
pub struct RuntimeTaskActor<ST, IN> where
    ST: AsyncActorState<IN> + Send + 'static,
    IN: ActorInteractor
{

    interactor: IN,
    phantom_data: PhantomData<ST>,
    dropped_indicator: Arc<()>

}

//tokio Task, spawned from a runtime handle Input/Output Actor

impl<ST, IN> RuntimeTaskActor<ST, IN> where
    ST: AsyncActorState<IN> + Send + 'static,
    IN: ActorInteractor
{

    pub fn new(handle: &Handle, state: ST) -> Self
    {

        let interactor =  state.interactor().clone();

        let dropped_indicator = Arc::new(());

        let dropped_indicator_moved = dropped_indicator.clone();
        
        handle.spawn(async move {
    
            RuntimeTaskActor::run(state, dropped_indicator_moved).await;

        });

        Self
        {

            interactor,
            phantom_data: PhantomData::default(),
            dropped_indicator,

        }

    }

    pub fn from_runtime(runtime: &Runtime, state: ST) -> Self
    {

        RuntimeTaskActor::new(runtime.handle(), state)

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

    pub fn get_interactor(&self) -> &IN
    {

        &self.interactor

    }  

}

impl<ST, IN> ActorFrontend<IN> for RuntimeTaskActor<ST, IN> where
    ST: AsyncActorState<IN> + Send + 'static,
    IN: ActorInteractor
{

    fn interactor(&self) -> &IN
    {

        &self.interactor

    }    

}

impl<ST, IN> Drop for RuntimeTaskActor<ST, IN> where
    ST: AsyncActorState<IN> + std::marker::Send + 'static,
    IN: ActorInteractor
{

    fn drop(&mut self)
    {
        
        self.interactor.input_default();

    }

}
    