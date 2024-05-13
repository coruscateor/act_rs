//https://doc.rust-lang.org/book/appendix-02-operators.html#non-operator-symbols

/**

    Generates an async orinented actor created using a Task spawned using a Runtime Handle
    

    Must have:

    use std::sync::Arc;

    use tokio::runtime::{Runtime, Handle};

    In module scope



    $state_type must implement trait:

    HasInteractor

    or

    fn interactor(&self) -> &IN

    directly

    Also:

    AsyncActorState

    or

    async fn on_enter_async(&mut self, _di: &DroppedIndicator) -> bool;

    async fn run_async(&mut self, di: &DroppedIndicator) -> bool;

    async fn on_exit_async(&mut self, _di: &DroppedIndicator);

    directly



    The returned boolean values from the on_enter_async and run_async method implementations indicate whether or not the actors execution should proceed.

*/
#[macro_export]
macro_rules! impl_mac_runtime_task_actor
{

    ($state_type:ty, $interactor_type:ty, $type_name:ident) =>
    {

        pub struct $type_name
        {

            interactor: $interactor_type,
            dropped_indicator: Arc<()>

        }

        //A tokio::Task spawned from a runtime handle Input/Output Actor

        impl $type_name
        {

            pub fn new(handle: &Handle, state: $state_type) -> Self
            {

                let interactor = state.interactor().clone();

                let dropped_indicator = Arc::new(());

                let dropped_indicator_moved = dropped_indicator.clone();
                
                handle.spawn(async move {

                    $type_name::run(state, dropped_indicator_moved).await;

                });

                Self
                {

                    interactor,
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

/**

    Generates an async orinented actor created within an async runtime context.

    Must have:

    use std::sync::Arc;

    In module scope



    $state_type must implement:

    HasInteractor

    or

    fn interactor(&self) -> &IN

    directly

    Also:

    AsyncActorState

    or

    async fn on_enter_async(&mut self, _di: &DroppedIndicator) -> bool;

    async fn run_async(&mut self, di: &DroppedIndicator) -> bool;

    async fn on_exit_async(&mut self, _di: &DroppedIndicator);

    directly



    The returned boolean values from the on_enter_async and run_async method implementations indicate whether or not the actors execution should proceed.

*/
#[macro_export]
macro_rules! impl_mac_task_actor
{

    ($state_type:ty, $interactor_type:ty, $type_name:ident) =>
    {

        pub struct $type_name
        {

            interactor: $interactor_type,
            dropped_indicator: Arc<()>

        }

        impl $type_name
        {

            pub fn new(state: $state_type) -> Self
            {

                let interactor = state.interactor().clone();

                let dropped_indicator = Arc::new(());

                let dropped_indicator_moved = dropped_indicator.clone();
                
                tokio::spawn(async move {

                    $type_name::run(state, dropped_indicator_moved).await;

                });

                Self
                {

                    interactor,
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
        
    }

}

//Default implementations of entry and exit methods to be used by the actor state

///
/// A default implementation of the on_enter_async mehthod for impl_mac_runtime_task_actor and impl_mac_task_actor implementators.
/// 
/// In this case it is a method returns a true constant.
/// 
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

///
/// Produces a default implementation of the on_exit_async method for impl_mac_runtime_task_actor and impl_mac_task_actor implementators.
///
/// In this case it is an empty method.
/// 
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

///
/// Produces default implementations of both the on_exit_async and on_exit_async methods for impl_mac_runtime_task_actor and impl_mac_task_actor implementators.
///
#[macro_export]
macro_rules! impl_default_on_enter_and_exit_async
{

    () =>
    {

        impl_default_on_enter_async!();

        impl_default_on_exit_async!();

    }

}

///
/// Produces an implementation of the on_enter_async method where DroppedIndicator::not_dropped gets called with its result returned immediately.
/// 
#[macro_export]
macro_rules! impl_not_dropped_on_enter_async
{

    () =>
    {

        async fn on_enter_async(&mut self, di: &DroppedIndicator) -> bool
        {
    
            di.not_dropped()
    
        }

    }

}

///
/// Produces default implementations of both the on_exit_async and on_exit_async methods using the impl_default_on_enter_async and impl_default_on_exit_async macros.
///
#[macro_export]
macro_rules! impl_not_dropped_on_enter_and_default_exit_async
{

    () =>
    {

        impl_not_dropped_on_enter_async!();

        impl_default_on_exit_async!();

    }

}


