use crate::ActorInteractor;

//use paste::paste;

///
/// For implementing interactor access on the "front end" of the actor.
/// 
pub trait ActorFrontend<IN: ActorInteractor>
{

    fn get_interactor_ref(&self) -> &IN; 

}

/*
#[macro_export]
macro_rules! impl_actor_frontend
{

    ($actor_type:ty, $interactor:ty) =>
    {

        impl<$interactor> ActorFrontend<$interactor> for $actor_type
        {

            fn get_interactor_ref(&self) -> &$interactor
            {

                &self.interactor

            }    

        }

    }

}
*/

/*
#[macro_export]
macro_rules! impl_actor_frontend
{

    ($name_type:ty, $st:ty, $interactor:ty) =>
    {

        paste! {

            impl<$st: std::marker::Send + 'static + AsyncActorState<IN>, $interactor: ActorInteractor> ActorFrontend<$interactor> for $name_type<$st, $interactor>  //[< < $io , $st > >]
            {

                fn get_interactor_ref(&self) -> &$interactor //&IN
                {

                    &self.interactor

                }    

            }

        }

    }

}
*/