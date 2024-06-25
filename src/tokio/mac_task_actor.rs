/**

    Generates an async oriented actor to be instantiated within an async runtime context.

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
                //where IN: Clone //<IN> //: IN
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
        
    }

}

//Different macros for different Task-actor methods of initialisation?

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


