use async_trait::async_trait;

use super::dropped_indicator::*;

///
/// All ActorInteractors must implement this trait. Assumes you're using at least one input queue.
/// 
/// Actors, or rather actor front-ends call input_default in an attempt to wakeup a possibly asleep actor so that it can determine that the front-end has been dropped and exit.
/// 
#[deprecated(since = "0.3.0", note = "Channels and other means of inter-thread communication should have ways of signalling to waiting actors that the front-end has dropped without needing every inter-actor to implement this trait.")]
pub trait ActorInteractor: Clone
{

    fn input_default(&self);

}

///
/// To be implemented on actor-states.
/// 
pub trait HasInteractor<IN>
{

    fn interactor(&self) -> &IN;

}

///
/// The actor state trait for standard thread and blocking thread based actors.
/// 
/// The returned boolean values from the on_enter and run method implementations indicate whether or not actor execution should proceed.
/// 
pub trait ActorState<IN> : HasInteractor<IN>
{

    fn on_enter(&mut self) -> bool
    {

        true

    }

    fn run(&mut self) -> bool;

    fn on_exit(&mut self)
    {
    }

}

///
/// The start trait for async oriented actors.
/// 
/// The returned boolean values from the on_enter_async and run_async method implementations indicate whether or not actor execution should proceed.
///
#[async_trait]
pub trait AsyncActorState<IN> : HasInteractor<IN>
{

    async fn on_enter_async(&mut self) -> bool
    {

        true

    }

    async fn run_async(&mut self) -> bool;

    async fn on_exit_async(&mut self)
    {
    }

}
