/**

    Generates an async oriented actor to be instantiated within an async runtime context.

    The state type that is provided to the produced spawn method must implement:

    ActorStateAsync

    or

    async fn pre_run_async(&mut sel) -> bool;

    async fn run_async(&mut self) -> bool;

    async fn post_run_async(self);

    directly

    Also tokio::task::JoinHandle and paste::paste must be in module scope.

    The latter is a macro from the [paste crate](https://crates.io/crates/paste).

    Works with version 1.0.15 and above.



    The name of the state type is generated from the provided $actor_type.

    As part of the macro output process "State" is appended to the provided $actor_type macro identity parameter, this type is required by the generated spawn method.
    


    The returned bool values from the pre_run_async and run_async method implementations indicate whether or not the actors execution should proceed.

    The post_run_async method is called regardless.

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
                    
                    if state.pre_run_async().await
                    {

                        while proceed
                        {
                            
                            proceed = state.run_async().await;
                
                        }

                    }

                    state.post_run_async().await;

                }

            }
            
        }

    }

}

/**
 * Similar to impl_mac_task_actor, but the produced spawn method takes an actor state builder object instead of the actor state itself.
 * 
 * Requires everything that impl_mac_task_actor does, but also that an actor state builder type with a method "build_async" that returns an optional actor state object share the scope of the macro call.
 * 
 * The actor state builder type name consists of the provided actor type name with "StateBuilder" appended.
 * 
 * Note that if build_async returns a None value then none of the run methods are called (including post_run_async).
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
                        
                        if state.pre_run_async().await
                        {

                            while proceed
                            {
                                
                                proceed = state.run_async().await;
                    
                            }

                        }

                        state.post_run_async().await;

                    }

                }

            }
            
        }

    }

}

//"Default" implementations of pre_run_async and post_run_async methods to be used by the actor state.

///
/// The "default" implementation of the pre_run_async method for impl_mac_task_actor and impl_mac_task_actor_built_state implementators.
/// 
/// This macro produces a method that returns a true bool value when called.
/// 
#[macro_export]
macro_rules! impl_pre_run_async
{

    () =>
    {

        async fn pre_run_async(&mut self) -> bool
        {
    
            true
    
        }

    }

}

///
/// The "default" implementation of the post_run_async method for impl_mac_task_actor and impl_mac_task_actor_built_state implementators.
/// 
/// This macro produces an empty method.
/// 
#[macro_export]
macro_rules! impl_post_run_async
{

    () =>
    {

        async fn post_run_async(self)
        {
        }

    }

}

///
/// Produces the "default" implementations of both the pre_run_async and post_run_async methods.
///
#[macro_export]
macro_rules! impl_pre_and_post_run_async
{

    () =>
    {

        async fn pre_run_async(&mut self) -> bool
        {
    
            true
    
        }

        async fn post_run_async(self)
        {
        }

    }

}
