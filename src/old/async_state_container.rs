use std::future::Future;

use std::marker::PhantomData;
use std::ops::{Fn, FnMut, FnOnce};
use std::sync::Arc;

use crate::{DroppedIndicator, HasInteractor, ActorInteractor};

pub struct AsycStateContainer<T, IN, FB, FNB, FR, FNR, FE, FNE>
    where FB: Future<Output = bool>,
         FNB: Fn(&mut T, &DroppedIndicator) -> FB, //FnOnce
          FR: Future<Output = bool>,
         FNR: Fn(&mut T, &DroppedIndicator) -> FR,
          FE: Future<Output = ()>,
         FNE: Fn(&mut T, &DroppedIndicator) -> FE,
         IN: ActorInteractor,
           T: HasInteractor<IN>
{

    state: T,
    fn_enter: Option<FNB>,
    fn_run: FNR,
    fn_exit: Option<FNE>,
    phantom_data: PhantomData<IN>

}

impl<T, IN, FB, FNB, FR, FNR, FE, FNE> AsycStateContainer<T, IN, FB, FNB, FR, FNR, FE, FNE>
where FB: Future<Output = bool>,
        FNB: Fn(&mut T, &DroppedIndicator) -> FB, //FnOnce
        FR: Future<Output = bool>,
        FNR: Fn(&mut T, &DroppedIndicator) -> FR,
        FE: Future<Output = ()>,
        FNE: Fn(&mut T, &DroppedIndicator) -> FE,
        IN: ActorInteractor,
        T: HasInteractor<IN>
{

    pub fn new(state: T, fn_enter: Option<FNB>, fn_run: FNR, fn_exit: Option<FNE>) -> Self
    {

        Self
        {

            state,
            fn_enter,
            fn_run,
            fn_exit,
            phantom_data: PhantomData::default()

        }

    }

    /*
    pub async fn run(&mut self, dropped_indicator: Arc<()>)
    {

        let di = DroppedIndicator::new(dropped_indicator);

        if let Some(fn_enter) = &self.fn_enter
        {

            let res = fn_enter(&mut self.state, &di).await;

            if !res
            {

                return;

            }

        }
        
        let mut proceed = true;

        while proceed
        {
            
            proceed = (self.fn_run)(&mut self.state, &di).await;

        }
        
        if let Some(fn_exit) = &self.fn_exit
        {

            fn_exit(&mut self.state, &di).await;

        }

    }
    */

    pub fn get_state(mut self) -> &'static mut T
    {

        &mut self.state

    }

    pub fn get_fn_enter(mut self) -> &'static mut Option<FNB>
    {

        &mut self.fn_enter

    }

    fn_run: FNR,
    fn_exit: Option<FNE>,

}

impl<T, IN, FB, FNB, FR, FNR, FE, FNE> HasInteractor<IN> for AsycStateContainer<T, IN, FB, FNB, FR, FNR, FE, FNE>
    where FB: Future<Output = bool>,
            FNB: Fn(&mut T, &DroppedIndicator) -> FB, //FnOnce
            FR: Future<Output = bool>,
            FNR: Fn(&mut T, &DroppedIndicator) -> FR,
            FE: Future<Output = ()>,
            FNE: Fn(&mut T, &DroppedIndicator) -> FE,
            IN: ActorInteractor,
            T: HasInteractor<IN>
{

    fn get_interactor(&self) -> IN
    {

        self.state.get_interactor()
        
    }

}
