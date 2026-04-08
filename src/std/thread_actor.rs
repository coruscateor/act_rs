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

    ///
    /// Spawn a new actor with the provided state.
    /// 
    pub fn spawn<ST>(state: ST) -> JoinHandle<()>
        where ST: ActorState + Send + 'static
    {
        
        thread::spawn(move ||
        {
    
            ThreadActor::run(state);

        })

    }

    ///
    /// Spawn a new actor and construct its state with the provided state builder in its thread.
    /// 
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

    pub fn spawn_catch_unwind<ST, F>(state: ST, err_fn: &Arc<F>) -> JoinHandle<()>
        where ST: ActorState + Send + UnwindSafe + 'static,
              F: Fn(Box<dyn Any + Send>) + Send + Sync + 'static
    {
        
        let err_fn_clone = err_fn.clone();

        thread::spawn(move ||
        {

            let result = catch_unwind(||
            {
                    
                ThreadActor::run(state);

            });

            if let Err(err) = result
            {

                err_fn_clone(err);

            }

        })

    }

    pub fn spawn_build_state_and_catch_unwind<ST, STB, F>(state_builder: STB, err_fn: &Arc<F>) -> JoinHandle<()>
        where ST: ActorState + Send + 'static,
              STB: ActorStateBuilder<ST> + Send + UnwindSafe + 'static,
              F: Fn(Box<dyn Any + Send>) + Send + Sync + 'static
    {

        let err_fn_clone = err_fn.clone();
        
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

                err_fn_clone(err);

            }

        })

    }

    pub fn build_spawn<ST>(builder: Builder, state: ST) -> Result<JoinHandle<()>>
        where ST: ActorState + Send + 'static
    {
        
        builder.spawn(move ||
        {
    
            ThreadActor::run(state);

        })

    }

    pub fn build_spawn_and_build_state<ST, STB>(builder: Builder, state_builder: STB) -> Result<JoinHandle<()>>
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

    pub fn build_spawn_and_catch_unwind<ST, F>(builder: Builder, state: ST, err_fn: &Arc<F>) -> Result<JoinHandle<()>>
        where ST: ActorState + Send + UnwindSafe + 'static,
              F: Fn(Box<dyn Any + Send>) + Send + Sync + 'static
    {
        
        let err_fn_clone = err_fn.clone();

        builder.spawn(move ||
        {

            let result = catch_unwind(||
            {
                    
                ThreadActor::run(state);

            });

            if let Err(err) = result
            {

                err_fn_clone(err);

            }

        })

    }

    pub fn build_spawn_build_state_and_catch_unwind<ST, STB, F>(builder: Builder, state_builder: STB, err_fn: &Arc<F>) -> Result<JoinHandle<()>>
        where ST: ActorState + Send + 'static,
              STB: ActorStateBuilder<ST> + Send + UnwindSafe + 'static,
              F: Fn(Box<dyn Any + Send>) + Send + Sync + 'static
    {

        let err_fn_clone = err_fn.clone();
        
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

                err_fn_clone(err);

            }

        })

    }
    
    pub fn run<ST>(mut state: ST)
        where ST: ActorState + Send + 'static
    {
        
        if state.pre_run()
        {

            let mut proceed = true; 

            while proceed
            {
                
                proceed = state.run();
    
            }

        }
        
        state.post_run();

    }  
    
}
