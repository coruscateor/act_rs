use async_trait::async_trait;
use futures::Future;
use tokio::{task::{self, JoinHandle, spawn_blocking, JoinError}, runtime::{Handle, Runtime}};
use futures::{executor::block_on, FutureExt};
use std::{marker::{PhantomData, Send, Sync}, sync::Arc};

use crate::{ActorInteractor, AsyncActorState, DroppedIndicator, impl_actor_frontend, ActorFrontend, HasInteractor}; //AsycStateContainer,

use paste::paste;

pub struct RuntimeTaskFnActor<ST, IN, FB, FNB, FR, FNR, FE, FNE> where
    //ST: std::marker::Send + 'static,
    IN: ActorInteractor,
    FB: Future<Output = bool>,
    FNB: Fn(&mut ST, &DroppedIndicator) -> FB,
    FR: Future<Output = bool> + Send,
    FNR: Fn(&mut ST, &DroppedIndicator) -> FR,
    FE: Future<Output = ()>,
    FNE: Fn(&mut ST, &DroppedIndicator) -> FE,
    IN: ActorInteractor,
    ST: HasInteractor<IN> + Send
{

    io: IN,
    phantom_data: PhantomData<ST>, //state container
    dropped_indicator: Arc<()>,
    phantom_data_FB: PhantomData<FB>,
    phantom_data_FNB: PhantomData<FNB>,
    phantom_data_FR: PhantomData<FR>,
    phantom_data_FNR: PhantomData<FNR>,
    phantom_data_FE: PhantomData<FE>,
    phantom_data_FNE: PhantomData<FNE>,

}

//tokio Task, spawned from a runtime handle Input/Output Actor

impl<ST, IN, FB, FNB, FR, FNR, FE, FNE> RuntimeTaskFnActor<ST, IN, FB, FNB, FR, FNR, FE, FNE> where
    IN: ActorInteractor + Send + 'static,
    FB: Future<Output = bool> + Send + 'static,
    FNB: (Fn(&mut ST, &DroppedIndicator) -> FB) + Send + Sync + 'static, //Sync + 
    FR: Future<Output = bool> + Send + 'static,
    FNR: Fn(&mut ST, &DroppedIndicator) -> FR  + Send + 'static, //Sync + 
    FE: Future<Output = ()> + Send + 'static,
    FNE: Fn(&mut ST, &DroppedIndicator) -> FE + Send + Sync + 'static, //Sync + 
    IN: ActorInteractor + 'static,
    ST: HasInteractor<IN> + Send + 'static
{

    pub fn new(handle: &Handle, state: ST, fn_enter: FNB, fn_run: FNR, fn_exit: FNE) -> Self //fn_enter: Option<FNB>, fn_run: FNR, fn_exit: Option<FNE>) -> Self //mut state: AsycStateContainer<T, IN, FB, FNB, FR, FNR, FE, FNE>) -> Self
    {

        let io =  state.get_interactor();

        let dropped_indicator = Arc::new(());

        let dropped_indicator_moved = dropped_indicator.clone();
        
        handle.spawn(async move {
    

            RuntimeTaskFnActor::run(state, dropped_indicator_moved, fn_enter, fn_run, fn_exit).await;
            
            //RuntimeTaskActor::run(state, dropped_indicator_moved).await;

            //state.run(dropped_indicator_moved).await;

            /*
            let di = DroppedIndicator::new(dropped_indicator_moved);

            if let Some(fn_enter) = &fn_enter
            {
    
                //let res_f = fn_enter(&mut state, &di); //.await;
    
                //let res = res_f.await;

                let res = fn_enter(&mut state, &di).await;

                if !res
                {
    
                    return;
    
                }
    
            }
            
            let mut proceed = true;
    
            while proceed
            {
                
                //proceed = (fn_run)(&mut state, &di).await;

                //proceed = fn_run(&mut state, &di).await;
    
                let res = (fn_run)(&mut state, &di);

                proceed = res.await; 

            }
            
            if let Some(fn_exit) = &fn_exit
            {
    
                fn_exit(&mut state, &di).await;
    
                //let res = fn_exit(&mut state, &di);

                //res.await;

            }
            */

            //let res = (fn_run)(&mut state, &di); //.await;

            //let aw = res.await; 

        });

        Self
        {

            io,
            phantom_data: PhantomData::default(),
            dropped_indicator,
            phantom_data_FB: PhantomData::default(),
            phantom_data_FNB: PhantomData::default(),
            phantom_data_FR: PhantomData::default(),
            phantom_data_FNR: PhantomData::default(),
            phantom_data_FE: PhantomData::default(),
            phantom_data_FNE: PhantomData::default(),

        }

    }

    pub fn from_runtime(runtime: &Runtime, state: ST, fn_enter: FNB, fn_run: FNR, fn_exit: FNE) -> Self //fn_enter: Option<FNB>, fn_run: FNR, fn_exit: Option<FNE>) -> Self //state: AsycStateContainer<T, IN, FB, FNB, FR, FNR, FE, FNE>) -> Self
    {

        RuntimeTaskFnActor::new(runtime.handle(), state, fn_enter, fn_run, fn_exit)

    }

    async fn run(mut state: ST, dropped_indicator: Arc<()>, fn_enter: FNB, fn_run: FNR, fn_exit: FNE)
    {

        let di = DroppedIndicator::new(dropped_indicator);

        let res = fn_enter(&mut state, &di).await;

        if !res
        {

            return;

        }
        
        let mut proceed = true;

        while proceed
        {

            let res = (fn_run)(&mut state, &di);

            proceed = res.await; 

        }
        
        fn_exit(&mut state, &di).await;

    }

    /*
    async fn run(mut state: ST, dropped_indicator: Arc<()>, fn_enter: Option<FNB>, fn_run: FNR, fn_exit: Option<FNE>)
    {

        let di = DroppedIndicator::new(dropped_indicator);

        if let Some(fn_enter) = &fn_enter
        {

            let res = fn_enter(&mut state, &di).await;

            if !res
            {

                return;

            }

        }
        
        let mut proceed = true;

        while proceed
        {

            let res = (fn_run)(&mut state, &di);

            proceed = res.await; 

        }
        
        if let Some(fn_exit) = &fn_exit
        {

            fn_exit(&mut state, &di).await;

        }

    }
    */

    /*
    async fn run(mut state: ST, dropped_indicator: Arc<()>)
    {

        let mut proceed = true; 

        let di = DroppedIndicator::new(dropped_indicator);
        
        if state.on_enter_async(&di).await
        {

            while proceed
            {
                
                proceed = state.run_async(&di).await;
    
            }
    
            state.on_exit_async(&di).await;

        }

    }
    */

    pub fn get_interactor_ref(&self) -> &IN
    {

        &self.io

    } 

}

//impl_actor_frontend!(RuntimeTaskFnActor, SC, IN); 

impl<ST, IN, FB, FNB, FR, FNR, FE, FNE> Drop for RuntimeTaskFnActor<ST, IN, FB, FNB, FR, FNR, FE, FNE> where
    IN: ActorInteractor,
    FB: Future<Output = bool>,
    FNB: Fn(&mut ST, &DroppedIndicator) -> FB, //FnOnce
    FR: Future<Output = bool> + Send,
    FNR: Fn(&mut ST, &DroppedIndicator) -> FR,
    FE: Future<Output = ()>,
    FNE: Fn(&mut ST, &DroppedIndicator) -> FE,
    IN: ActorInteractor,
    ST: HasInteractor<IN> + std::marker::Send
{

    fn drop(&mut self)
    {
        
        self.io.input_default();

    }

}


/*
pub struct RuntimeTaskFnActor<ST, IN> where
    ST: std::marker::Send + 'static,
    IN: ActorInteractor
{

    io: IN,
    phantom_data: PhantomData<ST>,
    dropped_indicator: Arc<()>

}

//tokio Task, spawned from a runtime handle Input/Output Actor

impl<ST, IN> RuntimeTaskFnActor<ST, IN> where
    ST: std::marker::Send + 'static + AsyncActorState<IN>,
    IN: ActorInteractor
{

    pub fn new(handle: &Handle, state: ST) -> Self
    {

        let io =  state.get_interactor();

        let dropped_indicator = Arc::new(());

        let dropped_indicator_moved = dropped_indicator.clone();
        
        handle.spawn(async move {
    
            //RuntimeTaskActor::run(state, dropped_indicator_moved).await;

        });

        Self
        {

            io,
            phantom_data: PhantomData::default(),
            dropped_indicator,

        }

    }

    pub fn from_runtime(runtime: &Runtime, state: ST) -> Self
    {

        RuntimeTaskFnActor::new(runtime.handle(), state)

    }

    /*
    async fn run(mut state: ST, dropped_indicator: Arc<()>)
    {

        let mut proceed = true; 

        let di = DroppedIndicator::new(dropped_indicator);
        
        if state.on_enter_async(&di).await
        {

            while proceed
            {
                
                proceed = state.run_async(&di).await;
    
            }
    
            state.on_exit_async(&di).await;

        }

    }
    */

}

impl_actor_frontend!(RuntimeTaskFnActor, SC, IN); 

impl<SC, IN> Drop for RuntimeTaskFnActor<SC, IN> where
    SC: std::marker::Send,
    IN: ActorInteractor
{

    fn drop(&mut self)
    {
        
        self.io.input_default();

    }

}
*/

