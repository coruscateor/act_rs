use crate::ActorInteractor;

///
/// For implementing interactor access on the "front end" of the actor.
/// 
pub trait ActorFrontend<IN>
{

    fn interactor(&self) -> &IN; 

}
