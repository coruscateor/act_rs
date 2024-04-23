/**

    An async orinented actor created using a Task spawned using a Runtime Handle
    

    Must have:

    use std::{marker::PhantomData, sync::Arc};

    use tokio::runtime::{Runtime, Handle};

    use act_rusty::ActorInteractor;

    In module scope


    Interactor_type must implement:

    pub trait ActorFrontend<IN: ActorInteractor>
    {

        fn interactor_ref(&self) -> &IN; 

    }

    or

    fn interactor_ref(&self) -> &IN


    state_type must implement:

    async fn on_enter_async(&mut self, _di: &DroppedIndicator) -> bool;

    async fn run_async(&mut self, di: &DroppedIndicator) -> bool;

    async fn on_exit_async(&mut self, _di: &DroppedIndicator);


    The returned boolean values from the on_enter_async and run_async method implementations indicate whether or not actor execution should proceed.

*/

#[macro_export]
macro_rules! impl_mac_runtime_task_actor
{

    ($interactor_type:ty, $state_type:ty, $type_name:ident) =>
    {

        pub struct $type_name
        {

            interactor: $interactor_type,//IN,
            phantom_data: PhantomData<$state_type>, //<ST>,
            dropped_indicator: Arc<()>

        }

        //tokio Task, spawned from a runtime handle Input/Output Actor

        impl $type_name
        {

            pub fn new(handle: &Handle, state: $state_type) -> Self
            {

                let interactor = state.interactor();

                let dropped_indicator = Arc::new(());

                let dropped_indicator_moved = dropped_indicator.clone();
                
                handle.spawn(async move {

                    $type_name::run(state, dropped_indicator_moved).await;

                });

                Self
                {

                    interactor,
                    phantom_data: PhantomData::default(),
                    dropped_indicator,

                }

            }

            pub fn from_runtime(runtime: &Runtime, state: $state_type) -> Self
            {

                $type_name::new(runtime.handle(), state)

            }

            async fn run(mut state: $state_type, dropped_indicator: Arc<()>)
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

        }

        impl ActorFrontend<$interactor_type> for $type_name 
        {

            fn interactor(&self) -> &$interactor_type
            {

                &self.interactor

            }    

        }

        impl Drop for $type_name
        {

            fn drop(&mut self)
            {
                
                self.interactor.input_default();

            }

        }
        
    }

}

/*

    An async orinented actor created within an async runtime context.

    must have:

    use std::{marker::PhantomData, sync::Arc};

    use paste::paste;

    //use act_rusty::ActorInteractor;

    In module scope

    interactor_type must implement:

    pub trait ActorFrontend<IN: ActorInteractor>
    {

        fn interactor(&self) -> &IN; 

    }

    or

    fn interactor(&self) -> &IN

    state_type must implement:

    async fn on_enter_async(&mut self, _di: &DroppedIndicator) -> bool;

    async fn run_async(&mut self, di: &DroppedIndicator) -> bool;

    async fn on_exit_async(&mut self, _di: &DroppedIndicator);


    The returned boolean values from the on_enter_async and run_async method implementations indicate whether or not actor execution should proceed.

*/

//

#[macro_export]
macro_rules! impl_mac_task_actor
{

    ($interactor_type:ty, $state_type:ty, $type_name:ident) =>
    {

        pub struct $type_name
        {

            interactor: $interactor_type,
            phantom_data: PhantomData<$state_type>,
            dropped_indicator: Arc<()>

        }

        impl $type_name
        {

            pub fn new(state: $state_type) -> Self
            {

                let interactor = state.interactor();

                let dropped_indicator = Arc::new(());

                let dropped_indicator_moved = dropped_indicator.clone();
                
                tokio::spawn(async move {

                    $type_name::run(state, dropped_indicator_moved).await;

                });

                Self
                {

                    interactor,
                    phantom_data: PhantomData::default(),
                    dropped_indicator,

                }

            }

            async fn run(mut state: $state_type, dropped_indicator: Arc<()>)
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

        }

        impl ActorFrontend<$interactor_type> for $type_name
        {

            fn interactor(&self) -> &$interactor_type
            {

                &self.interactor

            }    

        }

        impl Drop for $type_name
        {

            fn drop(&mut self)
            {
                
                self.interactor.input_default();

            }

        }

        //}
        
    }

}

//I don't think I need these actually...

//Default implementations of entry and exit methods to be used by the actor state

//
// A default implementation of the on_enter_async mehthod for impl_mac_runtime_task_actor and impl_mac_task_actor implementators.
//
/*
#[macro_export]
macro_rules! impl_default_on_enter_async
{

    () =>
    {

        async fn on_enter_async(&mut self, _di: &DroppedIndicator) -> bool
        {
    
            true
    
        }

    }

}
*/

//
// A default implementation of the on_exit_async method for impl_mac_runtime_task_actor and impl_mac_task_actor implementators.
//
/*
#[macro_export]
macro_rules! impl_default_on_exit_async
{

    () =>
    {

        async fn on_exit_async(&mut self, _di: &DroppedIndicator)
        {
        }
    

    }

}
*/

//
// Default implementations of both the on_exit_async and on_exit_async methods for impl_mac_runtime_task_actor and impl_mac_task_actor implementators.
//
/*
#[macro_export]
macro_rules! impl_default_on_enter_and_exit_async
{

    () =>
    {

        impl_default_on_enter_async!();

        impl_default_on_exit_async!();

    }

}
*/



