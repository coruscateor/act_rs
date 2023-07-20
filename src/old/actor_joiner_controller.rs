use tokio::sync::Notify;

use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, any::Any};

use futures::executor::block_on;

//use super::actor_joiner::ActorJoiner;

use std::sync::Mutex; //, Arc};

use crossbeam::queue::ArrayQueue;

pub struct ActorJoinerController
{

    is_active: AtomicBool,
    exited_notifer: Notify,
    //weak_self: Oprion<Arc<Self>> //Not really optional
    //panic_error: Mutex<Option<Arc<Box<dyn Any + Send>>>>
    has_paniced: AtomicBool,
    paniced_handler: PanicedHandler

}

impl ActorJoinerController
{

    pub fn new(which_paniced_handler: WhichPanicedHandler) -> Self
    {

        let paniced_handler; //: PanicedHandler

        match which_paniced_handler
        {

            WhichPanicedHandler::Default =>
            {

                paniced_handler = PanicedHandler::ArrayQueue(ArrayQueue::new(1));

            },
            /*WhichPanicedHandler::BoxFn(val) =>
            {

                paniced_handler = PanicedHandler::BoxFn(val);

            },*/
            WhichPanicedHandler::ArcFn(val) =>
            {

                paniced_handler = PanicedHandler::ArcFn(val);

            }

        }

        Self
        {

            is_active: AtomicBool::new(true),
            exited_notifer: Notify::new(),
            //weak_self: None
            //panic_error: Mutex::default()
            has_paniced: AtomicBool::new(false),
            paniced_handler

        }

    }

    pub fn get_is_active_relaxed(&self) -> bool
    {

        self.is_active.load(Ordering::Relaxed)

    }

    pub fn get_is_active_aquire(&self) -> bool
    {

        self.is_active.load(Ordering::Acquire)

    }

    pub fn get_is_active_seqcst(&self) -> bool
    {

        self.is_active.load(Ordering::SeqCst)

    }

    pub fn join(&self)
    {

        block_on(self.exited_notifer.notified())

    }

    pub async fn join_async(&self)
    {

        self.exited_notifer.notified().await

    }

    //pub

    pub fn set_inactive_relaxed(&self)
    {

        self.is_active.store(false, Ordering::Relaxed);

        self.exited_notifer.notify_waiters();

    }

    //pub

    pub fn set_inactive_release(&self)
    {

        self.is_active.store(false, Ordering::Release);

        self.exited_notifer.notify_waiters();

    }

    //pub

    pub fn set_inactive_seqcst(&self)
    {

        self.is_active.store(false, Ordering::SeqCst);

        self.exited_notifer.notify_waiters();

    }

    //errors

    pub fn set_error_relaxed(&self, err: Box<dyn Any + Send>)
    {

        self.set_error(err);

        self.set_inactive_relaxed();

        //self.is_active.store(false, Ordering::Relaxed);

        //self.exited_notifer.notify_waiters();

    }

    //pub

    pub fn set_error_release(&self, err: Box<dyn Any + Send>)
    {

        self.set_error(err);

        self.set_inactive_release();

        //self.is_active.store(false, Ordering::Release);

        //self.exited_notifer.notify_waiters();

    }

    //pub

    pub fn set_error_seqcst(&self, err: Box<dyn Any + Send>)
    {

        self.set_error(err);

        self.set_inactive_seqcst();

        //self.is_active.store(false, Ordering::SeqCst);

        //self.exited_notifer.notify_waiters();

    }

    fn set_error(&self, err: Box<dyn Any + Send>)
    {

        match &self.paniced_handler
        {

            PanicedHandler::ArrayQueue(val) => 
            {

                val.push(err);

            },
            /*PanicedHandler::BoxFn(val) =>
            {

                val(err);

            },*/
            PanicedHandler::ArcFn(val) =>
            {

                val.lock().unwrap()(err);

            }

        }

        /*
        let mut pe_mut = self.panic_error.get_mut().unwrap();

        *pe_mut = Some(Arc::new(err));
        */

    }

    pub fn get_error(&self) -> Option<Box<dyn Any + Send>>
    {

        if let PanicedHandler::ArrayQueue(val) = &self.paniced_handler
        {

            val.pop()

        }
        else
        {
        
            None

        }

    }

    /*
    pub fn get_actor_joiner(&self) -> ActorJoiner
    {

        //ActorJoiner::new

    }
    */

}

/*
impl Drop for ActorJoinerController
{

    fn drop(&mut self) {
        
        self.set_inactive_relaxed();

    }

}
*/

//pub

enum PanicedHandler
{

    ArrayQueue(ArrayQueue<Box<dyn Any + Send>>),
    //BoxFn(Box<dyn Fn(Box<dyn Any + Send>) -> ()>),
    ArcFn(Arc<Mutex<dyn Fn(Box<dyn Any + Send>) -> () + Send>>)

}

pub enum WhichPanicedHandler
{

    Default,
    //BoxFn(Box<dyn Fn(Box<dyn Any + Send>) -> ()>),
    ArcFn(Arc<Mutex<dyn Fn(Box<dyn Any + Send>) -> () + Send>>)

}

impl Default for WhichPanicedHandler
{

    fn default() -> Self
    {

        WhichPanicedHandler::Default

    }

}