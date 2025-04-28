//!
//! std::Thread based actors
//! 

mod thread_actor;

pub use thread_actor::*;

#[cfg(test)]
mod tests
{

    use crate::ActorState;

    use super::*;

    struct TwoPlusTwoActorState
    {

        two: u32

    }

    impl TwoPlusTwoActorState
    {

        pub fn new() -> Self
        {

            Self
            {

                two: 2

            }

        }
        
    }

    impl ActorState for TwoPlusTwoActorState
    {

        fn run(&mut self) -> bool
        {

            println!("two plus two is: {}", self.two + 2);

            true
            
        }

    }

    #[test]
    fn std_tread_actor_test()
    {

        ThreadActor::spawn(TwoPlusTwoActorState::new());

        //Don't run this in a test.

        //let _ = ThreadActor::spawn(TwoPlusTwoActorState::new()).join();

    }

}

