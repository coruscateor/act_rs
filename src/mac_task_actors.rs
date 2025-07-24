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

