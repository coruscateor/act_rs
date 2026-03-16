
#[cfg(feature = "async-trait")]
use async_trait::async_trait;

#[cfg(feature = "async-trait")]
use crate::ActorStateAsync;

use crate::ActorState;

///
/// An ActorState that deals with T.
/// 
pub trait ActorStateFlexible<T> : ActorState
    where Self: Sized,
          T: Into<bool>
{

    fn pre_run_flexible(&mut self) -> T;

    fn run_flexible(&mut self) -> T;

    fn pre_run(&mut self) -> bool
    {

        self.pre_run_flexible().into()

    }

    fn run(&mut self) -> bool
    {

        self.run_flexible().into()

    }

    fn post_run(self)
    {
    }

}

///
/// An async ActorState that deals with T.
/// 
#[cfg(feature = "async-trait")]
#[async_trait]
pub trait ActorStateFlexibleAsync<T> : ActorStateAsync
    where Self: Sized,
          T: Into<bool>
{

    async fn pre_run_flexible_async(&mut self) -> T;

    async fn run_flexible_async(&mut self) -> T;

    async fn pre_run_async(&mut self) -> bool
    {

        self.pre_run_flexible_async().await.into()

    }

    async fn run_async(&mut self) -> bool
    {

        self.run_flexible_async().await.into()

    }

    async fn post_run_async(self)
    {
    }

}

///
/// An ActorState that deals with T.
/// 
pub trait ActorStateFlexibleDefault<T> : ActorStateFlexible<T>
    where Self: Sized,
          T: Into<bool> + Default
{

    fn pre_run_flexible(&mut self) -> T
    {

        Default::default()

    }

}

///
/// An async ActorState that deals with T.
/// 
#[cfg(feature = "async-trait")]
#[async_trait]
pub trait ActorStateFlexibleDefaultAsync<T> : ActorStateFlexibleAsync<T>
    where Self: Sized,
          T: Into<bool> + Default
{

    async fn pre_run_flexible_async(&mut self) -> T
    {

        Default::default()

    }

}