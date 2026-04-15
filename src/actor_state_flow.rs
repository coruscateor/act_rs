
#[cfg(feature = "async-trait")]
use async_trait::async_trait;

#[cfg(feature = "async-trait")]
use crate::ActorStateAsync;

use crate::{ActorFlow, ActorState};

///
/// An ActorState that deals with ActorFlows.
/// 
pub trait ActorStateFlow : ActorState + Sized
{

    fn pre_run_flow(&mut self) -> ActorFlow
    {

        ActorFlow::Proceed

    }

    fn run_flow(&mut self) -> ActorFlow;

    fn pre_run(&mut self) -> bool
    {

        self.pre_run_flow().is_proceed()

    }

    fn run(&mut self) -> bool
    {

        self.run_flow().is_proceed()

    }

    fn post_run(self)
    {
    }

}

///
/// An async ActorState that deals with ActorFlows.
/// 
#[cfg(feature = "async-trait")]
#[async_trait]
pub trait ActorStateFlowAsync : ActorStateAsync + Sized
{

    async fn pre_run_flow_async(&mut self) -> ActorFlow
    {

        ActorFlow::Proceed

    }

    async fn run_flow_async(&mut self) -> ActorFlow;

    async fn pre_run_async(&mut self) -> bool
    {

        self.pre_run_flow_async().await.is_proceed()

    }

    async fn run_async(&mut self) -> bool
    {

        self.run_flow_async().await.is_proceed()

    }

    async fn post_run_async(self)
    {
    }

}