#[cfg(feature = "tokio")]
use async_trait::async_trait;

pub trait ActorStateBuilder<T>
{

    fn build(self) -> Option<T>;

}

#[cfg(feature = "tokio")]
#[async_trait]
pub trait ActorStateBuilderAsync<T>
{

    async fn build_async(self) -> Option<T>;

}
