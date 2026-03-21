//use async_trait::async_trait;
//use futures::Future;

//use futures::{executor::block_on, FutureExt};

use std::any::Any;
use std::{marker::PhantomData, sync::Arc, panic::UnwindSafe};

use crate::{ActorState, ActorStateBuilder};

use std::thread::{self, Builder, JoinHandle};

use std::panic::catch_unwind;

use std::io::Result;

///
/// An std::Thread based actor.
///
pub struct ThreadActor
{
}

impl ThreadActor
{

    pub fn spawn<ST>(state: ST) -> JoinHandle<()>
        where ST: ActorState + Send + 'static
    {
        
        thread::spawn(move ||
        {
    
            ThreadActor::run(state);

        })

    }

    pub fn spawn_and_build_state<ST, STB>(state_builder: STB) -> JoinHandle<()>
        where ST: ActorState + Send + 'static,
              STB: ActorStateBuilder<ST> + Send + 'static
    {
        
        thread::spawn(move ||
        {

            if let Some(state) = state_builder.build()
            {

                ThreadActor::run(state);

            }      

        })

    }

    pub fn spawn_catch_unwind<ST, F>(state: ST, err_fn: F) -> JoinHandle<()>
        where ST: ActorState + Send + UnwindSafe + 'static,
              F: FnOnce(Box<dyn Any + Send>) + Send + 'static
    {
        
        thread::spawn(move ||
        {

            let result = catch_unwind(||
            {
                    
                ThreadActor::run(state);

            });

            if let Err(err) = result
            {

                err_fn(err);

            }

        })

    }

    pub fn spawn_build_state_and_catch_unwind<ST, STB, F>(state_builder: STB, err_fn: F) -> JoinHandle<()>
        where ST: ActorState + Send + 'static,
              STB: ActorStateBuilder<ST> + Send + UnwindSafe + 'static,
              F: FnOnce(Box<dyn Any + Send>) + Send + 'static
    {
        
        thread::spawn(move ||
        {

            let result = catch_unwind(||
            {

                if let Some(state) = state_builder.build()
                {

                    ThreadActor::run(state);

                }

            });

            if let Err(err) = result
            {

                err_fn(err);

            }

        })

    }

    pub fn spawn_built_thread<ST>(builder: Builder, state: ST) -> Result<JoinHandle<()>>
        where ST: ActorState + Send + 'static
    {
        
        builder.spawn(move ||
        {
    
            ThreadActor::run(state);

        })

    }

    pub fn spawn_built_thread_and_build_state<ST, STB>(builder: Builder, state_builder: STB) -> Result<JoinHandle<()>>
        where ST: ActorState + Send + 'static,
              STB: ActorStateBuilder<ST> + Send + 'static
    {
        
        builder.spawn(move ||
        {

            if let Some(state) = state_builder.build()
            {

                ThreadActor::run(state);

            }      

        })

    }

    pub fn spawn_built_thread_and_catch_unwind<ST, F>(builder: Builder, state: ST, err_fn: F) -> Result<JoinHandle<()>>
        where ST: ActorState + Send + UnwindSafe + 'static,
              F: FnOnce(Box<dyn Any + Send>) + Send + 'static
    {
        
        builder.spawn(move ||
        {

            let result = catch_unwind(||
            {
                    
                ThreadActor::run(state);

            });

            if let Err(err) = result
            {

                err_fn(err);

            }

        })

    }

    pub fn spawn_built_thread_build_state_and_catch_unwind<ST, STB, F>(builder: Builder, state_builder: STB, err_fn: F) -> Result<JoinHandle<()>>
        where ST: ActorState + Send + 'static,
              STB: ActorStateBuilder<ST> + Send + UnwindSafe + 'static,
              F: FnOnce(Box<dyn Any + Send>) + Send + 'static
    {
        
        builder.spawn(move ||
        {

            let result = catch_unwind(||
            {

                if let Some(state) = state_builder.build()
                {

                    ThreadActor::run(state);

                }

            });

            if let Err(err) = result
            {

                err_fn(err);

            }

        })

    }
    
    pub fn run<ST>(mut state: ST)
        where ST: ActorState + Send + 'static
    {

        let mut proceed = true;
        
        if state.pre_run()
        {

            while proceed
            {
                
                proceed = state.run();
    
            }

        }
        
        state.post_run();

    }  
    
}
