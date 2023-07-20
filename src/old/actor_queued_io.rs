use super::notifying_queues::*;

use super::ActorIO;


//ArrayQueue - Input

pub struct NotifyingArrayQueueInput<M>
    where M: Default
{

    input: NotifyingArrayQueuePusher<M>

}

impl<M> NotifyingArrayQueueInput<M>
    where M: Default
{

    pub fn new(input: NotifyingArrayQueuePusher<M>) -> Self
    {

        Self
        {

            input

        }

    }

    pub fn get_input(&self) -> &NotifyingArrayQueuePusher<M>
    {

        &self.input

    }

}

impl<M> ActorIO for NotifyingArrayQueueInput<M>
    where M: Default
{

    fn send_default(&self)
    {

        self.input.push(M::default());
    
    }

}

//ArrayQueue - IO

pub struct NotifyingArrayQueueIO<MI, MO>
    where MI: Default
{

    input: NotifyingArrayQueuePusher<MI>,
    output: NotifyingArrayQueuePoper<MO>

}

impl<MI, MO> NotifyingArrayQueueIO<MI, MO>
    where MI: Default
{

    pub fn new(input: NotifyingArrayQueuePusher<MI>, output: NotifyingArrayQueuePoper<MO>) -> Self
    {

        Self
        {

            input,
            output

        }

    }

    pub fn get_input(&self) -> &NotifyingArrayQueuePusher<MI>
    {

        &self.input

    }

    pub fn get_output(&self) -> &NotifyingArrayQueuePoper<MO>
    {

        &self.output

    }

}

impl<MI, MO> ActorIO for NotifyingArrayQueueIO<MI, MO>
    where MI: Default
{

    fn send_default(&self)
    {

        self.input.push(MI::default());
    
    }

}


//SegQueue - Input

pub struct NotifyingSegQueueInput<M>
    where M: Default
{

    input: NotifyingSegQueuePusher<M>

}

impl<M> NotifyingSegQueueInput<M>
    where M: Default
{

    pub fn new(input: NotifyingSegQueuePusher<M>) -> Self
    {

        Self
        {

            input

        }

    }

    pub fn get_input(&self) -> &NotifyingSegQueuePusher<M>
    {

        &self.input

    }

}

impl<M> ActorIO for NotifyingSegQueueInput<M>
    where M: Default
{

    fn send_default(&self)
    {

        self.input.push(M::default());
    
    }

}

//SegQueue - IO

pub struct NotifyingSegQueueIO<MI, MO>
    where MI: Default
{

    input: NotifyingSegQueuePusher<MI>,
    output: NotifyingSegQueuePoper<MO>

}

impl<MI, MO> NotifyingSegQueueIO<MI, MO>
    where MI: Default
{

    pub fn new(input: NotifyingSegQueuePusher<MI>, output: NotifyingSegQueuePoper<MO>) -> Self
    {

        Self
        {

            input,
            output

        }

    }

    pub fn get_input(&self) -> &NotifyingSegQueuePusher<MI>
    {

        &self.input

    }

    pub fn get_output(&self) -> &NotifyingSegQueuePoper<MO>
    {

        &self.output

    }

}

impl<MI, MO> ActorIO for NotifyingSegQueueIO<MI, MO>
    where MI: Default
{

    fn send_default(&self)
    {

        self.input.push(MI::default());
    
    }

}
