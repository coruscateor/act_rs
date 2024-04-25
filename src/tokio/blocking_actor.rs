use async_trait::async_trait;
use futures::Future;

use futures::{executor::block_on, FutureExt};
use std::{marker::PhantomData, sync::Arc, panic::UnwindSafe};

use crate::{ActorInteractor, ActorState, DroppedIndicator};

#[allow(dead_code)]
pub struct BlockingActor<SC, IN> where
    SC: std::marker::Send + 'static,
    IN: ActorInteractor
{

    interactor: IN,
    phantom_data: PhantomData<SC>,
    dropped_indicator: Arc<()>

}

///
///Thread:spawn Input/Output Actor
///
impl<SC, IN> BlockingActor<SC, IN> where
    SC: std::marker::Send + 'static + ActorState<IN>,
    IN: ActorInteractor
{

    pub fn new(state: SC) -> Self
    {

        let interactor =  state.interactor();

        let dropped_indicator = Arc::new(());

        let dropped_indicator_moved = dropped_indicator.clone();
        
        tokio::task::spawn_blocking(move || {
    
            BlockingActor::run(state, dropped_indicator_moved);

        });

        Self
        {

            interactor,
            phantom_data: PhantomData::default(),
            dropped_indicator,

        }

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

impl<SC, IO> Drop for BlockingActor<SC, IO> where
    SC: std::marker::Send,
    IO: ActorInteractor
{

    fn drop(&mut self)
    {
        
        self.interactor.input_default();

    }

}
