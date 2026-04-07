use std::any::Any;

use async_trait::async_trait;

#[async_trait]
pub trait AsyncPanicHandler : Send + Sync
{

    async fn handle_panic(&self, boxed_panic: Box<dyn Any + Send>);
    
}
