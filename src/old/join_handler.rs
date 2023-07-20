use async_trait::async_trait;

pub trait JoinHandler
{

    fn join(&self);

}

/*
#[async_trait]
pub trait AsyncJoinHandler
{

    async fn join_async(&self) -> Result<(), tokio::task::JoinError>;

}
*/

#[async_trait]
pub trait TokioJoinHandler //: AsyncJoinHandler
{
    
    async fn join_async(&self) -> Result<(), tokio::task::JoinError>;

    fn is_finished(&self) -> bool;
    
}


