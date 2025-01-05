use async_trait::async_trait;

pub trait ActorStateBuilder<T>
{

    fn build(self) -> Option<T>;

}

#[async_trait]
pub trait AsyncActorStateBuilder<T>
{

    async fn build_async(self) -> Option<T>;

}
