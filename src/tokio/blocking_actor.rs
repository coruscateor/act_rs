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
    phantom_data: PhantomData<ST>

}

impl<ST, IN> BlockingActor<ST, IN> where
    ST: ActorState<IN> + std::marker::Send + 'static
{

    pub fn new(state: ST) -> Self
        where IN: Clone
    {

        let interactor =  state.interactor().clone();
        
        tokio::task::spawn_blocking(move || {
    
            BlockingActor::run(state);

        });

        Self
        {

            interactor,
            phantom_data: PhantomData::default()

        }

    }

    fn run(mut state: ST)
    {

        let mut proceed = true; 
        
        if state.on_enter()
        {

            while proceed
            {
                
                proceed = state.run();
    
            }
    
            state.on_exit();

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