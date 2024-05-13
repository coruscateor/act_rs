use async_trait::async_trait;
use futures::Future;

use futures::{executor::block_on, FutureExt};
use std::{marker::PhantomData, sync::Arc, panic::UnwindSafe};

use crate::{ActorFrontend, ActorInteractor, ActorState, DroppedIndicator};

use std::thread;

///
/// An std::Thred based actor.
///
#[allow(dead_code)]
pub struct ThreadActor<ST, IN> where
    ST: Send + 'static + ActorState<IN>,
    IN: ActorInteractor
{

    interactor: IN,
    phantom_data: PhantomData<ST>,
    dropped_indicator: Arc<()>

}

//Thread:spawn Input/Output Actor

impl<ST, IN> ThreadActor<ST, IN> where
    ST: Send + 'static + ActorState<IN>,
    IN: ActorInteractor
{

    pub fn new(state: ST) -> Self
    {

        let interactor =  state.interactor().clone();

        let dropped_indicator = Arc::new(());

        let dropped_indicator_moved = dropped_indicator.clone();
        
        thread::spawn(move || {
    
            ThreadActor::run(state, dropped_indicator_moved);

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

impl<ST, IN> ActorFrontend<IN> for ThreadActor<ST, IN> where
    ST: ActorState<IN> + Send + 'static,
    IN: ActorInteractor
{

    fn interactor(&self) -> &IN
    {

        &self.interactor

    }    

}

impl<ST, IN> Drop for ThreadActor<ST, IN> where
    ST: ActorState<IN> + Send,
    IN: ActorInteractor
{

    fn drop(&mut self)
    {
        
        self.interactor.input_default();

    }

}
