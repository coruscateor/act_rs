use async_trait::async_trait;

use super::dropped_indicator::*;

///
/// All ActorInteractors must implement this trait. Assumes you're using at least one input queue.
/// 
/// Actors, or rather actor front-ends call input_default in an attempt to wakeup a possibly asleep actor so that it can determine that the front-end has been dropped and exit.
/// 
pub trait ActorInteractor: Clone
{

    fn input_default(&self);

}

///
/// To be implemented on actor front-ends.
/// 
pub trait HasInteractor<IN>
    where IN: ActorInteractor
{

    fn interactor(&self) -> &IN;

}

///
/// The actor state trait for standard thread and blocking thread based actors.
/// 
/// The returned boolean values from the on_enter and run method implementations indicate whether or not actor execution should proceed.
/// 
pub trait ActorState<IN> : HasInteractor<IN>
    where IN: ActorInteractor
{

    fn on_enter(&mut self, _di: &DroppedIndicator) -> bool
    {

        true

    }

    fn run(&mut self, di: &DroppedIndicator) -> bool;

    fn on_exit(&mut self, _di: &DroppedIndicator)
    {
    }

}

///
/// The start trait for async oriented actors.
/// 
/// The returned boolean values from the on_enter_async and run_async method implementations indicate whether or not actor execution should proceed.
/// 
/// Broken: async traits don't seem to be able to work between crates (You can't use the trait defined here with an implmentation in another crate).
/// 
/// Use impl_mac_runtime_task_actor or impl_mac_task_actor instead.
///
#[async_trait]
pub trait AsyncActorState<IN> : HasInteractor<IN>
    where IN: ActorInteractor
{

    async fn on_enter_async(&mut self, _di: &DroppedIndicator) -> bool
    {

        true

    }

    async fn run_async(&mut self, di: &DroppedIndicator) -> bool;

    async fn on_exit_async(&mut self, _di: &DroppedIndicator)
    {
    }

}
