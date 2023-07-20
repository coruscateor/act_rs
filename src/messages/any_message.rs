use std::{boxed::Box, sync::Arc, any::Any};

///
/// Send anything at runtime.
/// 
pub enum AnyMessage
{

    Box(Box<dyn Any>),
    Arc(Arc<dyn Any>)
}

impl AnyMessage
{

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
        where T: Clone + 'static
    {

        match self
        {
            
           AnyMessage::Box(bx) =>
            {

                if bx.is::<T>()
                {

                    return Some(TypedMessage::Box(Box::new(bx.downcast_ref::<T>().unwrap().clone())));

                }

            },
            AnyMessage::Arc(arc) =>
            {

                if arc.is::<T>()
                {

                    return Some(TypedMessage::Arc(Arc::new(arc.downcast_ref::<T>().unwrap().clone())));

                }
                
            }

        }

        Option::None

    }

}

///
/// Like AnyMessage but generic. 
/// 
pub enum TypedMessage<T>
{

    Box(Box<T>),
    Arc(Arc<T>),

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
            
        }

    }

}

