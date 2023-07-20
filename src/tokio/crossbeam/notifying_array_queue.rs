use crossbeam::queue::ArrayQueue;

use tokio::sync::{Notify, futures::Notified};

use delegate::delegate;

use std::sync::Arc;

//An ArrayQueue that can be waited on.
pub struct NotfiyingArrayQueue<T>
{

    queue: ArrayQueue<T>,
    notify: Notify

}

impl<T> NotfiyingArrayQueue<T>
{

    pub fn new(cap: usize) -> Self
    {

        Self
        {

            queue: ArrayQueue::new(cap),
            notify: Notify::new()

        }

    }

    pub fn new_arc(cap: usize) -> Arc<Self>
    {

        Arc::new(Self::new(cap))

    }

    pub fn push_notify_one(&self, value: T) -> Result<(), T>
    {

        let res = self.queue.push(value);

        if let Ok(_) = &res
        {

            self.notify.notify_one();

        }

        res

    }

    pub fn force_push_notify_one(&self, value: T) -> Option<T>
    {

        let res = self.queue.force_push(value);

        self.notify.notify_one();

        res

    }

    pub fn push_notify_waiters(&self, value: T) -> Result<(), T>
    {

        let res = self.queue.push(value);

        if let Ok(_) = res
        {

            self.notify.notify_waiters();

        }

        res

    }

    pub fn force_push_notify_waiters(&self, value: T) -> Option<T>
    {

        let res = self.queue.force_push(value);

        self.notify.notify_waiters();

        res

    }

    //Tries to pop a value until successful

    pub async fn pop_or_wait(&self) -> T
    {

        let future = self.notify.notified();

        tokio::pin!(future);

        loop
        {

            //https://docs.rs/tokio/latest/tokio/sync/futures/struct.Notified.html

            //let future_mut = future.as_mut();

            //future_mut.enable();

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

                    //self.notify.notified().await;

                    //let future_mut = future.as_mut();

                    //future_mut.await;

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

                    //self.notify.notified().await;

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

            pub fn push(&self, value: T) -> Result<(), T>;

            pub fn force_push(&self, value: T) -> Option<T>;

            pub fn pop(&self) -> Option<T>;

            pub fn capacity(&self) -> usize;

            pub fn is_empty(&self) -> bool;

            pub fn is_full(&self) -> bool;

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
/// An input only NotfiyingArrayQueue.
/// 
pub struct InputNotfiyingArrayQueue<T>
{

    queue: Arc<NotfiyingArrayQueue<T>>

}

impl<T> InputNotfiyingArrayQueue<T>
{

    pub fn new(queue: Arc<NotfiyingArrayQueue<T>>) -> Self
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

            pub fn push_notify_one(&self, value: T) -> Result<(), T>;

            pub fn force_push_notify_one(&self, value: T) -> Option<T>;

            pub fn push_notify_waiters(&self, value: T) -> Result<(), T>;

            pub fn force_push_notify_waiters(&self, value: T) -> Option<T>;

            //

            pub fn push(&self, value: T) -> Result<(), T>;

            pub fn force_push(&self, value: T) -> Option<T>;

            pub fn capacity(&self) -> usize;

            pub fn is_empty(&self) -> bool;

            pub fn is_full(&self) -> bool;

            pub fn len(&self) -> usize;

        }

    }

    
}

impl<T> Clone for InputNotfiyingArrayQueue<T>
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
/// An output only NotfiyingArrayQueue.
/// 
pub struct OutputNotfiyingArrayQueue<T>
{

    queue: Arc<NotfiyingArrayQueue<T>>

}

impl<T> OutputNotfiyingArrayQueue<T>
{

    pub fn new(queue: Arc<NotfiyingArrayQueue<T>>) -> Self
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

            pub fn capacity(&self) -> usize;

            pub fn is_empty(&self) -> bool;

            pub fn is_full(&self) -> bool;

            pub fn len(&self) -> usize;

        }

    }

    
}

impl<T> Clone for OutputNotfiyingArrayQueue<T>
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
//Gets both ends of a divided NotfiyingArrayQueue
//
pub fn get_notifying_array_queue_io<T>(cap: usize) -> (InputNotfiyingArrayQueue<T>, OutputNotfiyingArrayQueue<T>) 
{

    let queue = NotfiyingArrayQueue::new_arc(cap);

    (InputNotfiyingArrayQueue::new(queue.clone()), OutputNotfiyingArrayQueue::new(queue))

}

