//pub mod actors;

mod actor_workers;

pub use actor_workers::*;

mod notifying_queues;

pub use notifying_queues::*;

mod tsm_actor;

pub use tsm_actor::*;

mod ssm_actor;

pub use ssm_actor::*;

mod plm_actor;

pub use plm_actor::*;

mod ts_io_actor;

pub use ts_io_actor::*;

mod actor_state_controller;

pub use actor_state_controller::*;

mod any_message;

pub use any_message::*;

mod multi_message;

pub use multi_message::*;

mod ttsb_io_actor;

pub use ttsb_io_actor::*;

mod st_io_actor;

pub use st_io_actor::*;

//mod join_handler;

mod dropped_indicator;

pub use dropped_indicator::*;

mod actor_joiner_controller;

pub use actor_joiner_controller::*;

mod actor_joiner;

pub use actor_joiner::*;

mod actor_queued_io;

pub use actor_queued_io::*;

#[cfg(test)]
mod tests {

    //use crate::actor_workers::*;

    //use crate::actor::{Jho_tsm,};

    struct IntTest
    {

        number: i32

    }

    impl IntTest
    {

        fn new() -> Self
        {

            Self
            {

                number: 0

            }

        }

        pub fn increment(&mut self) -> i32
        {

            self.number = self.number + 1;

            self.number

        }

        pub fn decrement(&mut self) -> i32
        {

            self.number = self.number - 1;

            self.number

        }

        pub fn plus(&mut self, val: i32) -> i32
        {

            self.number = self.number + val;

            self.number

        }

        pub fn minus(&mut self, val: i32) -> i32
        {

            self.number = self.number - val;

            self.number

        }

    }

    //Workers

    struct IntTest_increment
    {
    }

    impl IntTest_increment
    {

        pub fn new() -> Self
        {

            Self
            {
            }

        }

    }

    /*
    impl ActorWorker<IntTest, i32> for IntTest_increment
    {

        fn work(&mut self, data: &mut IntTest) -> i32
        {

            data.increment()

        }

    }
    */

    /* 
    enum IntTestMessage
    {

        Increment,
        Decrement,
        Plus(i32),
        Minus(i32)

    }

    fn IntTestMessageFn(obj: &mut IntTest, message: &IntTestMessage) -> Option<Box<dyn Any + Send>>
    {

        match message {
            
            IntTestMessage::Increment => {

                obj.increment();

                Some(Box::new(()))
                
            }
            IntTestMessage::Decrement=> {
                
                obj.decrement();

                Some(Box::new(()))

            }
            IntTestMessage::Plus(val) => {
                
                obj.plus(*val);

                Some(Box::new(obj.number))

            }
            IntTestMessage::Minus(val) => {
                
                obj.minus(*val);

                Some(Box::new(obj.number))

            }

        }

    }
    */

    //#[test]
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn it_works() {

        /*
        {

            use crate::actors::Jho_tsm::Actor; // as Jho_tsm_Actor;

            let jho_tsm_actor = Actor::new(IntTest::new()); //Jho_tsm_Actor::new(IntTest::new());

            let mut inc = IntTest_increment::new();

            let jh = jho_tsm_actor.do_work(inc);

            let res = jh.await.unwrap();

            inc = res.1;

            assert_eq!(1, res.0);
        
        }
        */

    }


}
