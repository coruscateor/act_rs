use async_trait::async_trait;
use futures::Future;

use futures::{executor::block_on, FutureExt};
use tokio::runtime::{Handle, Runtime};
use std::{marker::PhantomData, sync::Arc, panic::UnwindSafe};

use crate::{ActorInteractor, ActorState, DroppedIndicator};

///
/// A blocking thread actor that requres a runtime or a runtime handle to get started.
/// 
pub struct RuntimeBlockingActor<SC, IN> where
    SC: std::marker::Send + 'static,
    IN: ActorInteractor
{

    interactor: IN,
    phantom_data: PhantomData<SC>,
    dropped_indicator: Arc<()>

}

//Thread:spawn Input/Output Actor

impl<SC, IN> RuntimeBlockingActor<SC, IN> where
    SC: std::marker::Send + 'static + ActorState<IN>,
    IN: ActorInteractor
{

    pub fn new(handle: &Handle, state: SC) -> Self
    {

        let interactor =  state.interactor();

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

    pub fn from_runtime(runtime: &Runtime, state: SC) -> Self
    {

        RuntimeBlockingActor::new(runtime.handle(), state)

    }

    fn run(mut state: SC, dropped_indicator: Arc<()>)
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

    pub fn interactor(&self) -> &IN
    {

        &self.interactor

    }    
    
}

impl<SC, IO> Drop for RuntimeBlockingActor<SC, IO> where
    SC: std::marker::Send,
    IO: ActorInteractor
{

    fn drop(&mut self)
    {
        
        self.interactor.input_default();

    }

}


