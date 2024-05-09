
#[macro_export]
macro_rules! impl_pub_sender
{

    ($for_type:ty) =>
    {

        pub fn sender(&self) -> &$for_type
        {

            &self.sender
            
        }

    }

}

#[macro_export]
macro_rules! impl_new_sender
{

    ($for_type:ty) =>
    {
        
        pub fn new(sender: $for_type) -> Self
        {

            Self
            {

                sender

            }
            
        }

    }

}

#[macro_export]
macro_rules! impl_interactor_clone
{

    ($for_type:ty) =>
    {
        
        impl<T: Default> Clone for $for_type
        {

            fn clone(&self) -> Self
            {

                Self
                {

                    sender: self.sender.clone()

                }

            }

        }
        
    }

}

#[macro_export]
macro_rules! impl_actor_interactor
{

    ($for_type:ty) =>
    {

        impl<T: Default> ActorInteractor for $for_type
        {

            fn input_default(&self)
            {

                _ = self.sender.send(T::default());

            }

        }

    }

}

///
/// For interactors that have senders with async send methods.
///
/// Requires: "use futures::executor::block_on;""
/// 
#[macro_export]
macro_rules! impl_actor_interactor_async
{

    ($for_type:ty) =>
    {

        impl<T: Default> ActorInteractor for $for_type
        {

            fn input_default(&self)
            {

                _ = block_on(self.sender.send(T::default()));

            }

        }

    }

}