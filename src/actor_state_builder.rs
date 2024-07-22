
pub trait ActorStateBuilder<T>
{

    fn build(self) -> (bool, T);

}

pub trait AsyncActorStateBuilder<T>
{

    fn build_async(self) -> (bool, T);

}
