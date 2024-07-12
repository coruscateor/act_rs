/**

    Generates an async oriented actor to be instantiated within an async runtime context.

    The state type that is provided to the produced spawn method must implement:

    AsyncActorState

    or

    async fn start_async(&mut self, _di: &DroppedIndicator) -> bool;

    async fn run_async(&mut self, di: &DroppedIndicator) -> bool;

    async fn end_async(&mut self, _di: &DroppedIndicator);

    directly

    Also tokio::task::JoinHandle and paste::paste must be in module scope.

    The latter is a macro from the paste crate: https://crates.io/crates/paste.

    Works with version 1.0.15 and above.



    The name of the state type is generated from the provided $actor_type.

    As part of the macro output process "State" is appended to the provided $actor_type macro identity parameter, this type is required by the generated spawn method.
    


    The returned boolean values from the start_async and run_async method implementations indicate whether or not the actors execution should proceed.

    The end_async method is called regardless.

*/
#[macro_export]
macro_rules! impl_mac_task_actor
{

    //$state_type:ty, 
 
    //$type_name

    ($actor_type:ident) =>
    {

        paste!
        {

            pub struct $actor_type
            {
            }

            impl $actor_type
            {

                pub fn spawn(state: [<$actor_type State>]) -> JoinHandle<()>
                {
                    
                    tokio::spawn(async move {

                        $actor_type::run(state).await;

                    })

                }

                async fn run(mut state: [<$actor_type State>])
                {

                    let mut proceed = true; 
                    
                    if state.start_async().await
                    {

                        while proceed
                        {
                            
                            proceed = state.run_async().await;
                
                        }

                    }

                    state.end_async().await;

                }

            }
            
        }

    }

}

//Default implementations of start and end methods to be used by the actor state

///
/// A default implementation of the start_async mehthod for impl_mac_task_actor implementators.
/// 
/// In this case it is a method returns a true bool value.
/// 
#[macro_export]
macro_rules! impl_default_start_async
{

    () =>
    {

        async fn start_async(&mut self) -> bool
        {
    
            true
    
        }

    }

}

///
/// Produces a default implementation of the end_async method.
///
/// In this case it is an empty method.
/// 
#[macro_export]
macro_rules! impl_default_end_async
{

    () =>
    {

        async fn end_async(&mut self)
        {
        }

    }

}

///
/// Produces default implementations of both the start_async and end_async methods.
///
#[macro_export]
macro_rules! impl_default_start_and_end_async
{

    () =>
    {

        impl_default_start_async!();

        impl_default_end_async!();

    }

}
