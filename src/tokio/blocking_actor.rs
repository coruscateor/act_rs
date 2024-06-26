use async_trait::async_trait;
use futures::Future;

use futures::{executor::block_on, FutureExt};
use std::{marker::PhantomData, sync::Arc, panic::UnwindSafe};

use crate::{ActorFrontend, ActorState, DroppedIndicator};

#[allow(dead_code)]
pub struct BlockingActor<ST, IN> where
    ST: ActorState<IN> + std::marker::Send + 'static
{

    interactor: IN,
    phantom_data: PhantomData<ST>,
    dropped_indicator: Arc<()>

}

impl<ST, IN> BlockingActor<ST, IN> where
    ST: ActorState<IN> + std::marker::Send + 'static
{

    pub fn new(state: ST) -> Self
        where IN: Clone
    {

        let interactor =  state.interactor().clone();

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

impl<SC, IN> ActorFrontend<IN> for BlockingActor<SC, IN> where
    SC: ActorState<IN> + Send + 'static
{

    fn interactor(&self) -> &IN
    {

        &self.interactor

    }    

}