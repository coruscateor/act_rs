#[cfg(feature = "tokio")]
use async_trait::async_trait;

///
/// The trait used for standard thread and blocking thread based actors.
/// 
/// The returned boolean values from the on_started and run method implementations should indicate whether or not actor execution should proceed.
/// 
pub trait ActorState
    where Self: Sized
{

    fn on_started(&mut self) -> bool
    {

        true

    }

    fn run(&mut self) -> bool;

    fn on_ending(self)
    {
    }

}

///
/// The trait used for async oriented actors.
/// 
/// The returned boolean values from the on_started_async and run_async method implementations should indicate whether or not actor execution should proceed.
///
#[cfg(feature = "tokio")]
#[async_trait]
pub trait ActorStateAsync
    where Self: Sized
{

    async fn on_started_async(&mut self) -> bool
    {

        true

    }

    async fn run_async(&mut self) -> bool;

    async fn on_ending_async(self)
    {
    }

}



