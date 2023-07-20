use async_trait::async_trait;
use futures::Future;

use futures::{executor::block_on, FutureExt};
use std::{marker::PhantomData, sync::Arc, panic::UnwindSafe};

use crate::{ActorInteractor, ActorState, DroppedIndicator};

use std::thread; //::Thread;

///
/// An std::Thred based actor.
/// 
pub struct ThreadActor<ST, IN> where
    ST: std::marker::Send + 'static,
    IN: ActorInteractor
{

    io: IN,
    phantom_data: PhantomData<ST>,
    dropped_indicator: Arc<()>

}

//Thread:spawn Input/Output Actor

impl<ST, IN> ThreadActor<ST, IN> where
    ST: std::marker::Send + 'static + ActorState<IN>,
    IN: ActorInteractor
{

    pub fn new(state: ST) -> Self
    {

        let io =  state.get_interactor();

        let dropped_indicator = Arc::new(());

        let dropped_indicator_moved = dropped_indicator.clone();
        
        thread::spawn(move || {
    
            ThreadActor::run(state, dropped_indicator_moved);

        });

        Self
        {

            io,
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

    pub fn get_interactor_ref(&self) -> &IN
    {

        &self.io

    }    
    
}

impl<SC, IO> Drop for ThreadActor<SC, IO> where
    SC: std::marker::Send,
    IO: ActorInteractor
{

    fn drop(&mut self)
    {
        
        self.io.input_default();

    }

}
