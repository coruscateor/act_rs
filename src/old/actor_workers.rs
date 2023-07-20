use std::{marker::PhantomData};
use async_trait::async_trait;

pub trait ActorWorker<T>
{

    fn work(&mut self, data: &mut T);

}

pub trait ReturningActorWorker<T, R>
{

    fn work(&mut self, data: &mut T) -> R;

}

//Do Nothing

pub struct DoNothing
{

}

impl DoNothing
{

    pub fn new() -> Self
    {

        Self
        {
        }

    }

}

impl<T> ActorWorker<T> for DoNothing
{

    fn work(&mut self, _data: &mut T)
    {
    }

}

pub struct DoNothingReturns
{
}

impl DoNothingReturns
{

    pub fn new() -> Self
    {

        Self
        {
        }

    }

}

impl<T, R> ReturningActorWorker<T, R> for DoNothingReturns where
    R: Default
{

    fn work(&mut self, data: &mut T) -> R
    {

        Default::default()

    }

}

//async

#[async_trait]
pub trait AsyncActorWorker<T, R>
{

    async fn work(&mut self, data: &mut T);

}

#[async_trait]
pub trait ReturningAsyncActorWorker<T, R>  //: std::marker::Send
{
    
    async fn work(&mut self, data: &mut T) -> R;

}

//async - Do Nothing

pub struct AsyncDoNothing
{
}

impl AsyncDoNothing
{

    pub fn new() -> Self
    {

        Self
        {
        }

    }

}

#[async_trait]
impl<T> AsyncActorWorker<T, ()> for AsyncDoNothing where
    T: std::marker::Send
{

    async fn work(&mut self, _data: &mut T)
    {
    }

}

pub struct AsyncDoNothingReturns
{
}

impl AsyncDoNothingReturns
{

    pub fn new() -> Self
    {

        Self
        {
        }

    }

}

#[async_trait]
impl<T, R> ReturningAsyncActorWorker<T, R> for AsyncDoNothingReturns where
    T: std::marker::Send,
    //O: std::marker::Send //+ std::marker::Sync
    R: Default
{

    async fn work(&mut self, _data: &mut T)  -> R
    {

        Default::default()

    }

}

//Yeild Task Now

pub struct AsyncYieldNow
{
}

#[async_trait]
impl<T> AsyncActorWorker<T, ()> for AsyncYieldNow where
    T: std::marker::Send
{

    async fn work(&mut self, _data: &mut T) -> ()
    {
        
        tokio::task::yield_now().await;

    }

}

pub struct AsyncYieldNowReturns
{
}

#[async_trait]
impl<T, R> ReturningAsyncActorWorker<T, R> for AsyncYieldNowReturns where
    T: std::marker::Send,
    //O: std::marker::Send
    R: Default
{

    async fn work(&mut self, _data: &mut T) -> R
    {

        tokio::task::yield_now().await;

        Default::default()

    }

}

