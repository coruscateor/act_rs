use std::{sync::Arc, any::Any};

use async_trait::async_trait;

use super::dropped_indicator::*;

use super::actor_joiner_controller::WhichPanicedHandler;

/*
pub trait InputActor<I>
{

    fn input(value: I);

}

pub trait OutputActor //<O>
{

    fn output() -> Option<Box<dyn Any>>; //<O>;

}
*/

pub trait ActorIO
{

    fn send_default(&self);

}

pub trait ActorStateController<IO> //T, //, I> : InputActor<I>
    where IO: ActorIO
{

    fn get_IO(&self) -> Arc<IO>;

    fn on_enter(&mut self);

    fn run(&mut self, di: &DroppedIndicator) -> bool;

    fn on_exit(&mut self);

    fn get_which_paniced_handler(&self) -> WhichPanicedHandler;

}

#[async_trait]
pub trait AsyncActorStateController<IO>
    where IO: ActorIO
{

    fn get_IO(&self) -> Arc<IO>;

    async fn on_enter_async(&mut self);

    async fn run_async(&mut self, di: &DroppedIndicator) -> bool;

    async fn on_exit_async(&mut self);

    fn get_which_paniced_handler(&self) -> WhichPanicedHandler;

}
