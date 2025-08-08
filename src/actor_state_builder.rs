//#[cfg(feature = "tokio")]

use async_trait::async_trait;

///
/// The contract that all ActorStateBuilder obects are expected to adhere to. The build method is called during the “build” phase of supported actor type instances. If the result of this call is None then it is expected that the actor will exit without executing any further phases.
/// 
pub trait ActorStateBuilder<T>
{

    fn build(self) -> Option<T>;

}

//#[cfg(feature = "tokio")]

///
/// The contract that all ActorStateBuilderAsync obects are expected to adhere to. The build_async method is called during the “build” phase of supported actor type instances. If the result of this call is None then it is expected that the actor will exit without executing any further phases.
/// 
#[async_trait]
pub trait ActorStateBuilderAsync<T>
{

    async fn build_async(self) -> Option<T>;

}
