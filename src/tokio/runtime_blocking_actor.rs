use async_trait::async_trait;
use futures::Future;

use futures::{executor::block_on, FutureExt};
use tokio::runtime::{Handle, Runtime};
use std::{marker::PhantomData, sync::Arc, panic::UnwindSafe};

use crate::{ActorFrontend, ActorInteractor, ActorState, AsyncActorState, DroppedIndicator};

///
/// A blocking thread actor that requres a runtime or a runtime handle to get started.
/// 
#[allow(dead_code)]
pub struct RuntimeBlockingActor<ST, IN> where
    ST: ActorState<IN> + Send + 'static,
    IN: ActorInteractor
{

    interactor: IN,
    phantom_data: PhantomData<ST>,
    dropped_indicator: Arc<()>

}

//Thread:spawn Input/Output Actor

impl<ST, IN> RuntimeBlockingActor<ST, IN> where
    ST: ActorState<IN> + Send + 'static,
    IN: ActorInteractor
{

    pub fn new(handle: &Handle, state: ST) -> Self
    {

        let interactor =  state.interactor().clone();

        let dropped_indicator = Arc::new(());

        let dropped_indicator_moved = dropped_indicator.clone();
        
        handle.spawn_blocking(move || {
    
            RuntimeBlockingActor::run(state, dropped_indicator_moved);

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

        RuntimeBlockingActor::new(runtime.handle(), state)

    }

    fn run(mut state: ST, dropped_indicator: Arc<()>)
    {

        let mut proceed = true; 

        let di = DroppedIndicator::new(dropped_indicator);
        
        if state.on_enter(&di)
        {

            while proceed
            {
                
                proceed = state.run(&di);
    
            }
    
            state.on_exit(&di);

        }

    }
    
}

impl<ST, IN> ActorFrontend<IN> for RuntimeBlockingActor<ST, IN> where
    ST: ActorState<IN> + Send + 'static,
    IN: ActorInteractor
{

    fn interactor(&self) -> &IN
    {

        &self.interactor

    }    

}

impl<ST, IN> Drop for RuntimeBlockingActor<ST, IN> where
    ST: ActorState<IN> + Send + 'static,
    IN: ActorInteractor
{

    fn drop(&mut self)
    {
        
        self.interactor.input_default();

    }

}


