use crate::ActorInteractor;

///
/// For implementing interactor access on the "front end" of the actor.
/// 
pub trait ActorFrontend<IN: ActorInteractor>
{

    fn interactor(&self) -> &IN; 

}
