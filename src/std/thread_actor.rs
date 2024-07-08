use async_trait::async_trait;
use futures::Future;

use futures::{executor::block_on, FutureExt};
use std::{marker::PhantomData, sync::Arc, panic::UnwindSafe};

use crate::{ActorFrontend, ActorState, DroppedIndicator};

use std::thread;

///
/// An std::Thread based actor.
///
#[allow(dead_code)]
pub struct ThreadActor<ST, IN> where
    ST: Send + 'static + ActorState<IN>
{

    interactor: IN,
    phantom_data: PhantomData<ST>

}

impl<ST, IN> ThreadActor<ST, IN> where
    ST: Send + 'static + ActorState<IN>
{

    pub fn new(state: ST) -> Self
        where IN: Clone
    {

        let interactor =  state.interactor().clone();
        
        thread::spawn(move || {
    
            ThreadActor::run(state);

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

impl<ST, IN> ActorFrontend<IN> for ThreadActor<ST, IN> where
    ST: ActorState<IN> + Send + 'static
{

    fn interactor(&self) -> &IN
    {

        &self.interactor

    }    

}
