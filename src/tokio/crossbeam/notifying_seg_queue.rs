use crossbeam::queue::SegQueue;

use tokio::sync::{Notify, futures::Notified};

use delegate::delegate;

use std::sync::Arc;

///
/// A SegQueue that can be waited on.
/// 
pub struct NotfiyingSegQueue<T>
{

    queue: SegQueue<T>,
    notify: Notify

}

impl<T> NotfiyingSegQueue<T>
{

    pub fn new() -> Self
    {

        Self
        {

            queue: SegQueue::new(),
            notify: Notify::new()

        }

    }

    pub fn new_arc() -> Arc<Self>
    {

        Arc::new(Self::new())

    }

    pub fn push_notify_one(&self, value: T)
    {

        self.queue.push(value);

        self.notify.notify_one();

    }

    pub fn push_notify_waiters(&self, value: T)
    {

        self.queue.push(value);

        self.notify.notify_waiters();

    }

    //Tries to pop a value until successful

    pub async fn pop_or_wait(&self) -> T
    {

        let future = self.notify.notified();

        tokio::pin!(future);

        loop
        {

            //https://docs.rs/tokio/latest/tokio/sync/futures/struct.Notified.html

            future.as_mut().enable();

            let pop_res = self.pop();

            match pop_res
            {

                Some(val) =>
                {

                    return val;

                }
                None =>
                {

                    future.as_mut().await;

                    future.set(self.notify.notified());

                }
                
            }
            
        }

    }

    pub async fn try_pop_or_wait(&self, tries: usize) -> (Option<T>, usize)
    {

        let future = self.notify.notified();

        tokio::pin!(future);

        let mut count: usize = 0;

        loop
        {

            if count == tries
            {

                return (None, count);

            }

            future.as_mut().enable();

            let pop_res = self.pop();

            match pop_res
            {

                Some(val) =>
                {

                    return (Some(val), count);

                }
                None =>
                {

                    future.as_mut().await;

                    future.set(self.notify.notified());

                }
                
            }

            count += 1;
            
        }

    }

    //

    pub async fn pop_or_wait_notify_waiters(&self) -> T
    {

        loop
        {

            //https://docs.rs/tokio/latest/tokio/sync/futures/struct.Notified.html

            let pop_res = self.pop();

            match pop_res
            {

                Some(val) =>
                {

                    return val;

                }
                None =>
                {

                    self.notify.notified().await;

                }
                
            }
            
        }

    }

    pub async fn try_pop_or_wait_notify_waiters(&self, tries: usize) -> (Option<T>, usize)
    {

        let mut count: usize = 0;

        loop
        {

            if count == tries
            {

                return (None, count);

            }

            let pop_res = self.pop();

            match pop_res
            {

                Some(val) =>
                {

                    return (Some(val), count);

                }
                None =>
                {

                    self.notify.notified().await;

                }
                
            }

            count += 1;
            
        }

    }

    delegate!
    {

        to self.queue
        {

            pub fn push(&self, value: T);

            pub fn pop(&self) -> Option<T>;

            pub fn is_empty(&self) -> bool;

            pub fn len(&self) -> usize;

        }

    }

    delegate!
    {

        to self.notify
        {

            pub fn notified(&self) -> Notified<'_>;

            pub fn notify_one(&self);

            pub fn notify_waiters(&self);

        }

    }

}

//IO

//Input

///
/// An input only NotfiyingSegQueue.
/// 
pub struct InputNotfiyingSegQueue<T>
{

    queue: Arc<NotfiyingSegQueue<T>>

}

impl<T> InputNotfiyingSegQueue<T>
{

    pub fn new(queue: Arc<NotfiyingSegQueue<T>>) -> Self
    {

        Self
        {

            queue

        }

    }

    pub fn has_another_queue_ref(&self) -> bool
    {

        Arc::strong_count(&self.queue) > 1

    }

    delegate!
    {

        to self.queue
        {

            pub fn push_notify_one(&self, value: T);

            pub fn push_notify_waiters(&self, value: T);

            //

            pub fn push(&self, value: T);            

            pub fn is_empty(&self) -> bool;

            pub fn len(&self) -> usize;


        }

    }

    
}

impl<T> Clone for InputNotfiyingSegQueue<T>
{

    fn clone(&self) -> Self
    {

        Self
        {
            
            queue: self.queue.clone()
        
        }

    }

}

//Output

///
/// An output only NotfiyingSegQueue.
/// 
pub struct OutputNotfiyingSegQueue<T>
{

    queue: Arc<NotfiyingSegQueue<T>>

}

impl<T> OutputNotfiyingSegQueue<T>
{

    pub fn new(queue: Arc<NotfiyingSegQueue<T>>) -> Self
    {

        Self
        {

            queue

        }

    }

    pub fn has_another_queue_ref(&self) -> bool
    {

        Arc::strong_count(&self.queue) > 1

    }

    delegate!
    {

        to self.queue
        {

            pub async fn pop_or_wait(&self) -> T;

            pub async fn try_pop_or_wait(&self, tries: usize) -> (Option<T>, usize);

            pub async fn pop_or_wait_notify_waiters(&self) -> T;

            pub async fn try_pop_or_wait_notify_waiters(&self, tries: usize) -> (Option<T>, usize);

            //

            pub fn pop(&self) -> Option<T>;

            pub fn is_empty(&self) -> bool;

            pub fn len(&self) -> usize;

        }

    }

    
}

impl<T> Clone for OutputNotfiyingSegQueue<T>
{

    fn clone(&self) -> Self
    {

        Self
        {
            
            queue: self.queue.clone()
        
        }

    }

}

//
//Gets both ends of a divided NotfiyingSegyQueue
//
pub fn get_notifying_seg_queue_io<T>() -> (InputNotfiyingSegQueue<T>, OutputNotfiyingSegQueue<T>) 
{

    let queue = NotfiyingSegQueue::new_arc();

    (InputNotfiyingSegQueue::new(queue.clone()), OutputNotfiyingSegQueue::new(queue))

}

