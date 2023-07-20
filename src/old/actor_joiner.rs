//use tokio::sync::Notify;

//use std::sync::atomic::{AtomicBool, Ordering};

//use futures::executor::block_on;

use std::sync::Arc;

use super::actor_joiner_controller::ActorJoinerController;

pub struct ActorJoiner
{

    aajc: Arc<ActorJoinerController>

}

impl ActorJoiner
{

    pub fn new(aajc: &Arc<ActorJoinerController>) -> Self
    {

        Self
        {

            aajc: aajc.clone()

        }

    }
    
    pub fn get_is_active_relaxed(&self) -> bool
    {

        self.aajc.get_is_active_relaxed()

    }

    pub fn get_is_active_aquire(&self) -> bool
    {

        self.aajc.get_is_active_aquire()

    }

    pub fn get_is_active_seqcst(&self) -> bool
    {

        self.aajc.get_is_active_seqcst()

    }

    pub fn join(&self)
    {

        self.aajc.join()

    }

    pub async fn join_async(&self)
    {

        self.aajc.join_async().await

    }
    
    /*
    pub fn set_inactive_relaxed(&self)
    {

        self.ajc.set_inactive_relaxed()

    }

    pub fn set_inactive_release(&self)
    {

        self.ajc.set_inactive_release()

    }

    pub fn set_inactive_seqcst(&self)
    {

        self.ajc.set_inactive_seqcst()

    }
    */

}