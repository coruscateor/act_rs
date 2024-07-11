/**

    Generates an async oriented actor to be instantiated within an async runtime context.

    $state_type must implement:

    AsyncActorState

    or

    async fn beginning_async(&mut self, _di: &DroppedIndicator) -> bool;

    async fn run_async(&mut self, di: &DroppedIndicator) -> bool;

    async fn ending_async(&mut self, _di: &DroppedIndicator);

    directly

    Also:

    use tokio::task::JoinHandle;

    must be in module scope.

    

    The returned boolean values from the beginning_async and run_async method implementations indicate whether or not the actors execution should proceed.

*/
#[macro_export]
macro_rules! impl_mac_task_actor
{

    ($state_type:ty, $type_name:ident) =>
    {

        pub struct $type_name
        {
        }

        impl $type_name
        {

            pub fn spawn(state: $state_type) -> JoinHandle<()>
            {
                
                tokio::spawn(async move {

                    $type_name::run(state).await;

                })

            }

            async fn run(mut state: $state_type)
            {

                let mut proceed = true; 
                
                if state.beginning_async().await
                {

                    while proceed
                    {
                        
                        proceed = state.run_async().await;
            
                    }

                }

                state.ending_async().await;

            }

        }
        
    }

}

//Default implementations of beginning and ending methods to be used by the actor state

///
/// A default implementation of the beginning_async mehthod for impl_mac_task_actor implementators.
/// 
/// In this case it is a method returns a true bool value.
/// 
#[macro_export]
macro_rules! impl_default_beginning_async
{

    () =>
    {

        async fn beginning_async(&mut self) -> bool
        {
    
            true
    
        }

    }

}

///
/// Produces a default implementation of the ending_async method.
///
/// In this case it is an empty method.
/// 
#[macro_export]
macro_rules! impl_default_ending_async
{

    () =>
    {

        async fn ending_async(&mut self)
        {
        }

    }

}

///
/// Produces default implementations of both the beginning_async and ending_async methods.
///
#[macro_export]
macro_rules! impl_default_beginning_and_ending_async
{

    () =>
    {

        impl_default_beginning_async!();

        impl_default_ending_async!();

    }

}
