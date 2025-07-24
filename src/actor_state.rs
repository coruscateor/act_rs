//#[cfg(feature = "tokio")]

use async_trait::async_trait;

///
/// The trait used for standard thread and blocking thread based actors.
/// 
/// The returned boolean values from the pre_run and run method implementations should indicate whether or not actor execution should proceed.
/// 
pub trait ActorState
    where Self: Sized
{

    fn pre_run(&mut self) -> bool
    {

        true

    }

    fn run(&mut self) -> bool;

    fn post_run(self)
    {
    }

}

///
/// The trait used for async oriented actors.
/// 
/// The returned boolean values from the pre_run_async and run_async method implementations should indicate whether or not actor execution should proceed.
///
#[async_trait]
pub trait ActorStateAsync
    where Self: Sized
{

    async fn pre_run_async(&mut self) -> bool
    {

        true

    }

    async fn run_async(&mut self) -> bool;

    async fn post_run_async(self)
    {
    }

}

//#[cfg(feature = "tokio")]



