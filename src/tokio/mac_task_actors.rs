/**

    Generates an async oriented actor to be instantiated within an async runtime context.

    The state type that is provided to the produced spawn method must implement:

    AsyncActorState

    or

    async fn on_start_async(&mut self, _di: &DroppedIndicator) -> bool;

    async fn run_async(&mut self, di: &DroppedIndicator) -> bool;

    async fn on_end_async(&mut self, _di: &DroppedIndicator);

    directly

    Also tokio::task::JoinHandle and paste::paste must be in module scope.

    The latter is a macro from the paste crate: https://crates.io/crates/paste.

    Works with version 1.0.15 and above.



    The name of the state type is generated from the provided $actor_type.

    As part of the macro output process "State" is appended to the provided $actor_type macro identity parameter, this type is required by the generated spawn method.
    


    The returned boolean values from the on_start_async and on_run_async method implementations indicate whether or not the actors execution should proceed.

    The on_end_async method is called regardless.

*/
#[macro_export]
macro_rules! impl_mac_task_actor
{

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
                    
                    if state.on_started_async().await
                    {

                        while proceed
                        {
                            
                            proceed = state.run_async().await;
                
                        }

                    }

                    state.on_ending_async().await;

                }

            }
            
        }

    }

}

/**
 * Similar to impl_mac_task_actor, but the produced spawn method takes an actor state builder object instead of the actor state itself.
 * 
 * Requires everything that impl_mac_task_actor does, but also that an actor state builder type with a method "build_async" that returns an actor state object be in the module scope of the macro call.
 * 
 * The actor state builder type name consists of the provided actor type name with the text "StateBuilder" appended.
*/
#[macro_export]
macro_rules! impl_mac_task_actor_built_state
{

    ($actor_type:ident) =>
    {

        paste!
        {

            pub struct $actor_type
            {
            }

            impl $actor_type
            {

                pub fn spawn(state_builder: [<$actor_type StateBuilder>]) -> JoinHandle<()>
                {
                    
                    tokio::spawn(async move {

                        $actor_type::run(state_builder).await;

                    })

                }

                async fn run(mut state_builder: [<$actor_type StateBuilder>])
                {

                    let mut opt_state = state_builder.build_async().await;

                    if let Some(mut state) = opt_state
                    {

                        let mut proceed = true; 
                        
                        if state.on_started_async().await
                        {

                            while proceed
                            {
                                
                                proceed = state.run_async().await;
                    
                            }

                        }

                        state.on_ending_async().await;

                    }

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
macro_rules! impl_on_started_async
{

    () =>
    {

        async fn on_started_async(&mut self) -> bool
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
macro_rules! impl_on_ending_async
{

    () =>
    {

        async fn on_ending_async(self)
        {
        }

    }

}

///
/// Produces default implementations of both the start_async and end_async methods.
///
#[macro_export]
macro_rules! impl_on_started_and_ending_async
{

    () =>
    {

        async fn on_started_async(&mut self) -> bool
        {
    
            true
    
        }

        async fn on_ending_async(self)
        {
        }

    }

}
