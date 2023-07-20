use super::{notifying_array_queue::{InputNotfiyingArrayQueue, OutputNotfiyingArrayQueue, get_notifying_array_queue_io}, notifying_seg_queue::{InputNotfiyingSegQueue, OutputNotfiyingSegQueue, get_notifying_seg_queue_io}};

use crate::ActorInteractor;

use futures::executor::block_on;

//Input

///
/// An interactor containing only an InputNotfiyingArrayQueue.
/// 
pub struct InputNotfiyingArrayQueueInteractor<T: Default>
{

    queue: InputNotfiyingArrayQueue<T>

}

impl<T: Default> InputNotfiyingArrayQueueInteractor<T>
{

    pub fn new(queue: InputNotfiyingArrayQueue<T>) -> Self
    {

        Self
        {

            queue

        }
        
    }

    pub fn get_queue_ref(&self) -> &InputNotfiyingArrayQueue<T>
    {

        &self.queue

    }

}

impl<T: Default> Clone for InputNotfiyingArrayQueueInteractor<T>
{

    fn clone(&self) -> Self
    {

        Self
        {

            queue: self.queue.clone()

        }

    }

}

impl<T: Default> ActorInteractor for InputNotfiyingArrayQueueInteractor<T>
{

    fn input_default(&self)
    {

        _ = self.queue.push(T::default());

    }

}

///
/// Get a NotifyingArrayQueue with with the input object being an input interactor.
/// 
/// Usefull if you only want your actor to be interacted with by an input queue.
/// 
pub fn get_notifying_array_queue_iowii<T: Default>(buffer: usize) -> (InputNotfiyingArrayQueueInteractor<T>, OutputNotfiyingArrayQueue<T>)
{

    let res = get_notifying_array_queue_io(buffer);

    (InputNotfiyingArrayQueueInteractor::new(res.0), res.1)

}

//Ouptput

///
/// An interactor containing only an InputNotfiyingSegQueue.
/// 
pub struct InputNotfiyingSegQueueInteractor<T: Default>
{

    queue: InputNotfiyingSegQueue<T>

}

impl<T: Default> InputNotfiyingSegQueueInteractor<T>
{

    pub fn new(queue: InputNotfiyingSegQueue<T>) -> Self
    {

        Self
        {

            queue

        }
        
    }

    pub fn get_queue_ref(&self) -> &InputNotfiyingSegQueue<T>
    {

        &self.queue

    }

}

impl<T: Default> Clone for InputNotfiyingSegQueueInteractor<T>
{

    fn clone(&self) -> Self
    {

        Self
        {

            queue: self.queue.clone()

        }

    }

}

impl<T: Default> ActorInteractor for InputNotfiyingSegQueueInteractor<T>
{

    fn input_default(&self)
    {

        self.queue.push(T::default());

    }

}

///
/// Get a NotifyingSegQueue with with the input object being an input interactor.
/// 
/// Usefull if you only want your actor to be interacted with by an input queue.
/// 
pub fn get_notifying_seg_queue_iowii<T: Default>() -> (InputNotfiyingSegQueueInteractor<T>, OutputNotfiyingSegQueue<T>)
{

    let res = get_notifying_seg_queue_io();

    (InputNotfiyingSegQueueInteractor::new(res.0), res.1)

}



