use crossbeam::queue::{ArrayQueue, SegQueue};
use tokio::sync::Notify;
use std::sync::Arc;
use std::ops::Drop;

pub struct NotifyingArrayQueue<T>
{

    queue: ArrayQueue<T>,
    notifier: Notify

}

impl<T> NotifyingArrayQueue<T>
{

    pub fn new(cap: usize) -> Self
    {

        Self
        {

            queue: ArrayQueue::<T>::new(cap),
            notifier: Notify::new()

        }

    }

    pub fn new_arc(cap: usize) -> Arc<Self>
    {

        Arc::new(Self::new(cap))

    }

    pub fn push(&self, value: T) -> Result<(), T>
    {

        self.queue.push(value)

    }

    pub fn push_notify_all(&self, value: T) -> Result<(), T>
    {

        let res = self.queue.push(value);

        self.notifier.notify_waiters();

        res

    }

    pub fn push_notify_one(&self, value: T) -> Result<(), T>
    {

        let res = self.queue.push(value);

        self.notifier.notify_one();

        res

    }

    pub fn pop(&self) -> Option<T>
    {

        self.queue.pop()
        
    }

    async fn pop_or_wait(&self) -> Option<T>
    {

        //loop {
         
            let opt_res = self.queue.pop();

            if let Some(res) = opt_res
            {
    
                return Some(res);
            
            }
            else
            {
    
                self.notifier.notified().await;

                self.queue.pop()
    
            }
            
        //}

    }

    pub async fn pop_or_wait_count(&self, mut count: usize) -> Option<T>
    {

        while count > 0 {
         
            let opt_res = self.queue.pop();

            if opt_res.is_some()
            {

                return opt_res;

            }
            else
            {
    
                self.notifier.notified().await;
    
            }

            count -= 1;
                
        }
        
        None

    }

    /*
    pub async fn pop_or_wait_once(&self) -> Option<T>
    {

        self.pop_or_wait_count(1).await

    }
    */

    pub fn capacity(&self) -> usize
    {

        self.queue.capacity()

    }

    pub fn is_empty(&self) -> bool
    {

        self.queue.is_empty()

    }

    pub fn is_full(&self) -> bool
    {

        self.queue.is_full()

    }

    pub fn len(&self) -> usize
    {

        self.queue.len()

    }

}

pub struct NotifyingSegQueue<T>
{

    queue: SegQueue<T>,
    notifier: Notify

}

impl<T> NotifyingSegQueue<T>
{

    pub fn new() -> Self
    {

        Self
        {

            queue: SegQueue::<T>::new(),
            notifier: Notify::new()

        }

    }

    pub fn new_arc() -> Arc<Self>
    {

        Arc::new(Self::new())

    }

    pub fn push(&self, value: T)
    {

        self.queue.push(value)

    }

    pub fn push_notify_all(&self, value: T)
    {

        self.queue.push(value);

        self.notifier.notify_waiters();

    }

    pub fn push_notify_one(&self, value: T)
    {

        self.queue.push(value);

        self.notifier.notify_one();

    }

    pub fn pop(&self) -> Option<T>
    {

        self.queue.pop()
        
    }

    async fn pop_or_wait(&self) -> Option<T>
    {

        //loop {
         
            let opt_res = self.queue.pop();

            if let Some(res) = opt_res
            {
    
                return Some(res);
            
            }
            else
            {
    
                self.notifier.notified().await;

                self.queue.pop()
    
            }
            
        //}

    }

    pub async fn pop_or_wait_count(&self, mut count: usize) -> Option<T>
    {

        while count > 0 {
         
            let opt_res = self.queue.pop();

            if opt_res.is_some()
            {

                return opt_res;

            }
            else
            {
    
                self.notifier.notified().await;
    
            }

            count -= 1;
                
        }
        
        None

    }

    /*
    pub async fn pop_or_wait_once(&self) -> Option<T>
    {

        self.pop_or_wait_count(1).await

    }
    */

    pub fn is_empty(&self) -> bool
    {

        self.queue.is_empty()

    }

    fn len(&self) -> usize
    {

        self.queue.len()

    }

}

//Pusher/Popers

pub struct QueueNotify<T>
{

    pub queue: T,
    pub notifier: Notify

}

impl<T> QueueNotify<T>
{

    pub fn new(queue: T) -> Self
    {

        Self { queue, notifier: Notify::new() }

    }

}

impl<T> Drop for QueueNotify<T>
{

    fn drop(&mut self) {


        self.notifier.notify_waiters();

    }

}

//ArrayQueue

pub struct NotifyingArrayQueuePusher<T>
{

    qna: Arc<QueueNotify<ArrayQueue<T>>>

}

impl<T> NotifyingArrayQueuePusher<T>
{

    pub fn new(qna: &Arc<QueueNotify<ArrayQueue<T>>>) -> Self
    {

        Self
        {

            qna: qna.clone()

        }

    }

    pub fn push(&self, value: T) -> Result<(), T>
    {

        self.qna.queue.push(value)

    }

    pub fn push_notify_all(&self, value: T) -> Result<(), T>
    {

        let res = self.qna.queue.push(value);

        self.qna.notifier.notify_waiters();

        res

    }

    pub fn push_notify_one(&self, value: T) -> Result<(), T>
    {

        let res = self.qna.queue.push(value);

        self.qna.notifier.notify_one();

        res

    }

    pub fn capacity(&self) -> usize
    {

        self.qna.queue.capacity()

    }

    pub fn is_empty(&self) -> bool
    {

        self.qna.queue.is_empty()

    }

    pub fn is_full(&self) -> bool
    {

        self.qna.queue.is_full()

    }

    pub fn len(&self) -> usize
    {

        self.qna.queue.len()

    }

}

impl<T> Drop for NotifyingArrayQueuePusher<T>
{

    fn drop(&mut self) {


        self.qna.notifier.notify_waiters();

    }

}

pub struct NotifyingArrayQueuePoper<T>
{

    qna: Arc<QueueNotify<ArrayQueue<T>>>

}

impl<T> NotifyingArrayQueuePoper<T>
{

    pub fn new(qna: &Arc<QueueNotify<ArrayQueue<T>>>) -> Self
    {

        Self
        {

            qna: qna.clone()

        }

    }

    pub fn pop(&self) -> Option<T>
    {

        self.qna.queue.pop()
        
    }

    pub async fn pop_or_wait(&self) -> Option<T>
    {

        let mut opt_res;

        //loop {
         
            opt_res = self.qna.queue.pop();

            if opt_res.is_some()
            {

                //break;

            }
            else
            {

                if Arc::strong_count(&self.qna) == 2
                {

                    self.qna.notifier.notified().await;

                    return self.qna.queue.pop();

                }
                else
                {
                    
                    //break;

                }
    
            }
            
        //}

        opt_res

    }

    pub async fn pop_or_wait_count(&self, mut count: usize) -> Option<T>
    {

        while count > 0 {
         
            let opt_res = self.qna.queue.pop();

            if opt_res.is_some()
            {

                return opt_res;

            }
            else
            {
    
                if Arc::strong_count(&self.qna) == 2
                {

                    self.qna.notifier.notified().await;

                }
                else
                {

                    break;
                    
                }
    
            }

            count -= 1;
                
        }
        
        None

    }

    /*
    pub async fn pop_or_wait_once(&self) -> Option<T>
    {

        let opt_res = self.qna.queue.pop();

        if opt_res.is_some()
        {

            return opt_res;

        }
        
        if Arc::strong_count(&self.qna) == 2
        {

            self.qna.notifier.notified().await;

            return self.qna.queue.pop();

        }
        
        None

    }
    */

    pub fn capacity(&self) -> usize
    {

        self.qna.queue.capacity()

    }

    pub fn is_empty(&self) -> bool
    {

        self.qna.queue.is_empty()

    }

    pub fn is_full(&self) -> bool
    {

        self.qna.queue.is_full()

    }

    pub fn len(&self) -> usize
    {

        self.qna.queue.len()

    }

}

impl<T> Drop for NotifyingArrayQueuePoper<T>
{

    fn drop(&mut self) {


        self.qna.notifier.notify_waiters();

    }

}


pub fn notifying_array_queue_pusher_poper<T>(cap: usize) -> (NotifyingArrayQueuePusher<T>, NotifyingArrayQueuePoper<T>)
{

    let anaq = Arc::new(QueueNotify::new(ArrayQueue::<T>::new(cap)));

    let pusher = NotifyingArrayQueuePusher::new(&anaq);

    let poper = NotifyingArrayQueuePoper::new(&anaq);

    (pusher, poper)

}

//SegQueue

pub struct NotifyingSegQueuePusher<T>
{

    qna: Arc<QueueNotify<SegQueue<T>>>

}

impl<T> NotifyingSegQueuePusher<T>
{

    pub fn new(qna: &Arc<QueueNotify<SegQueue<T>>>) -> Self
    {

        Self
        {

            qna: qna.clone()

        }

    }

    pub fn push(&self, value: T) //-> Result<(), T>
    {

        self.qna.queue.push(value)

    }

    pub fn push_notify_all(&self, value: T) //-> Result<(), T>
    {

        //let res = 
        
        self.qna.queue.push(value);

        self.qna.notifier.notify_waiters();

        //res

    }

    pub fn push_notify_one(&self, value: T) //-> Result<(), T>
    {

        //let res = 
        
        self.qna.queue.push(value);

        self.qna.notifier.notify_one();

        //res

    }

    pub fn is_empty(&self) -> bool
    {

        self.qna.queue.is_empty()

    }

    pub fn len(&self) -> usize
    {

        self.qna.queue.len()

    }

}

impl<T> Drop for NotifyingSegQueuePusher<T>
{

    fn drop(&mut self) {


        self.qna.notifier.notify_waiters();

    }

}

pub struct NotifyingSegQueuePoper<T>
{

    qna: Arc<QueueNotify<SegQueue<T>>>

}

impl<T> NotifyingSegQueuePoper<T>
{

    pub fn new(qna: &Arc<QueueNotify<SegQueue<T>>>) -> Self
    {

        Self
        {

            qna: qna.clone()

        }

    }

    pub fn pop(&self) -> Option<T>
    {

        self.qna.queue.pop()
        
    }

    pub async fn pop_or_wait(&self) -> Option<T>
    {

        let mut opt_res;

        //loop {
         
            opt_res = self.qna.queue.pop();

            if opt_res.is_some()
            {

                //break;

            }
            else
            {

                if Arc::strong_count(&self.qna) == 2
                {

                    self.qna.notifier.notified().await;

                    return self.qna.queue.pop();

                }
                else
                {
                    
                    //break;

                }
    
            }
            
        //}

        opt_res

    }

    pub async fn pop_or_wait_count(&self, mut count: usize) -> Option<T>
    {

        while count > 0 {
         
            let opt_res = self.qna.queue.pop();

            if opt_res.is_some()
            {

                return opt_res;

            }
            else
            {
    
                if Arc::strong_count(&self.qna) == 2
                {

                    self.qna.notifier.notified().await;

                }
                else
                {

                    break;
                    
                }
    
            }

            count -= 1;
                
        }
        
        None

    }

    /*
    pub async fn pop_or_wait_once(&self) -> Option<T>
    {

        let opt_res = self.qna.queue.pop();

        if opt_res.is_some()
        {

            return opt_res;

        }
        
        if Arc::strong_count(&self.qna) == 2
        {

            self.qna.notifier.notified().await;

            return self.qna.queue.pop();

        }
        
        None

    }
    */

    pub fn is_empty(&self) -> bool
    {

        self.qna.queue.is_empty()

    }

    pub fn len(&self) -> usize
    {

        self.qna.queue.len()

    }

}

impl<T> Drop for NotifyingSegQueuePoper<T>
{

    fn drop(&mut self) {


        self.qna.notifier.notify_waiters();

    }

}

pub fn notifying_seg_queue_pusher_poper<T>() -> (NotifyingSegQueuePusher<T>, NotifyingSegQueuePoper<T>)
{

    let anaq = Arc::new(QueueNotify::new(SegQueue::<T>::new()));

    let pusher = NotifyingSegQueuePusher::new(&anaq);

    let poper = NotifyingSegQueuePoper::new(&anaq);

    (pusher, poper)

}
