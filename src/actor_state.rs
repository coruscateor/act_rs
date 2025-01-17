use async_trait::async_trait;

///
/// The trait used for standard thread and blocking thread based actors.
/// 
/// The returned boolean values from the on_begin and run method implementations indicate whether or not actor execution should proceed.
/// 
pub trait ActorState
    where Self: Sized
{

    fn on_start(&mut self) -> bool
    {

        true

    }

    fn run(&mut self) -> bool;

    fn on_end(self)
    {
    }

}

///
/// The trait used for async oriented actors.
/// 
/// The returned boolean values from the on_start_async and run_async method implementations indicate whether or not actor execution should proceed.
///
#[async_trait]
pub trait ActorStateAsync
    where Self: Sized
{

    async fn on_start_async(&mut self) -> bool
    {

        true

    }

    async fn run_async(&mut self) -> bool;

    async fn on_end_async(self)
    {
    }

}



