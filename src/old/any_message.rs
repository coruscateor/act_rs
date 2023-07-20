use std::{boxed::Box, sync::Arc, any::Any};

pub enum AnyMessage
{

    Box(Box<dyn Any>),
    Arc(Arc<dyn Any>),
    Empty

}

impl AnyMessage
{

    pub fn is_empty(&self) -> bool
    {

        if let AnyMessage::Empty = self
        {

            return true;

        }

        false

    }

    pub fn downcast_ref<T>(&self) -> Option<&T>
        where T: 'static 
    {

        match self
        {
            
            AnyMessage::Box(bx) =>
            {

                if bx.is::<T>()
                {

                    return bx.downcast_ref::<T>();

                }

            },
            AnyMessage::Arc(arc) =>
            {

                if arc.is::<T>()
                {

                    return arc.downcast_ref::<T>();

                }
                
            },
            AnyMessage::Empty => {

            }
            
        }

        Option::None

    }

    /*
    pub fn downcast_mut<T>(&self) -> Option<&mut T>
        where T: 'static 
    {

        match self
        {
            
            Message::Box(bx &mut Box<T>) =>
            {

                if bx.is::<T>()
                {

                    return bx.downcast_mut::<T>();

                }

            },
            Message::Arc(arc) =>
            {

                if arc.is::<T>()
                {

                    return arc.downcast_mut::<T>();

                }
                
            },
            Message::Empty => {

            }
            
        }

        Option::None

    }
    */

    pub fn try_into_clone<T>(&self) -> Option<TypedMessage<T>>
        where T: Clone + 'static //'static + Any + Send + Sync
        //where T: 'static + Any + Send + Sync
    {

        match self
        {
            
           AnyMessage::Box(bx) =>
            {

                if bx.is::<T>()
                {

                    return Some(TypedMessage::Box(Box::new(bx.downcast_ref::<T>().unwrap().clone()))); //.downcast().unwrap()));

                }

            },
            AnyMessage::Arc(arc) =>
            {

                if arc.is::<T>()
                {
                    
                    //return Some(TypedMessage::Arc(arc.downcast_ref::<T>().unwrap().clone()));

                    //return Some(TypedMessage::Arc(arc.downcast().unwrap()));

                    return Some(TypedMessage::Arc(Arc::new(arc.downcast_ref::<T>().unwrap().clone())));

                }
                
            },
            AnyMessage::Empty => {
                
                //Option::None

            }
            
        }

        Option::None

    }

}

impl Default for AnyMessage
{

    fn default() -> Self
    {
        
        AnyMessage::Empty

    }

}

pub enum TypedMessage<T>
{

    Box(Box<T>),
    Arc(Arc<T>),
    Empty

}

impl<T> TypedMessage<T>
{

    pub fn is_empty(&self) -> bool
    {

        if let TypedMessage::Empty = self
        {

            return true;

        }

        false

    }

}

impl<T> Into<AnyMessage> for TypedMessage<T>
    where T: 'static
{

    fn into(self) -> AnyMessage {
        
        match self
        {
            
            TypedMessage::Box(bx) =>
            {

                AnyMessage::Box(bx)

            },
            TypedMessage::Arc(arc) =>
            {
            
                AnyMessage::Arc(arc)
                
            },
            TypedMessage::Empty => {
                
                AnyMessage::Empty

            }
            
        }

    }

}

impl<T> Default for TypedMessage<T>
{

    fn default() -> Self
    {
        
        TypedMessage::Empty

    }

}