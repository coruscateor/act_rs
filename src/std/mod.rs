//!
//! std::Thread based actors
//! 

mod thread_actor;

pub use thread_actor::*;

#[cfg(test)]
mod tests
{

    use crate::ActorState;

    use std::sync::mpsc::{Sender, channel}; //Receiver,

    use super::*;

    struct TwoPlusTwoActorState
    {

        number: u32,
        client_sender: Sender<String>

    }

    impl TwoPlusTwoActorState
    {

        pub fn new(client_sender: Sender<String>) -> Self
        {

            Self
            {

                number: 2,
                client_sender

            }

        }
        
    }

    impl ActorState for TwoPlusTwoActorState
    {

        fn run(&mut self) -> bool
        {

            if self.number < 4
            {

                self.number += 2;

                let message = format!("two plus two is: {}", self.number);

                if let Err(_) = self.client_sender.send(message)
                {

                    return false;

                }

                return true;

            }

            false

            //println!("two plus two is: {}", self.two + 2);
            
        }

    }

    #[test]
    fn std_tread_actor_test()
    {

        let (sender, receiver) = channel();

        ThreadActor::spawn(TwoPlusTwoActorState::new(sender));

        let res = receiver.recv().expect("Error: Message not delivered");

        println!("{}", res);

        //Don't run this in a test.

        //let _ = ThreadActor::spawn(TwoPlusTwoActorState::new()).join();

    }

}

