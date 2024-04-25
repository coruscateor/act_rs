//mod task_actor;

//pub use task_actor::*;

mod dropped_indicator;

pub use dropped_indicator::*;

mod actor_state;

pub use actor_state::*;

//mod thread_actor;

//pub use thread_actor::*;

//mod blocking_actor;

//pub use blocking_actor::*;

//pub mod tokio_interactors;

//pub mod tokio_oneshot;

//pub mod messages;

#[cfg(feature="std")]
pub mod std;

//_actors

#[cfg(feature="tokio")]
pub mod tokio;

mod actor_frontend;

pub use actor_frontend::*;

//mod async_state_container;

//pub use async_state_container::*;

pub mod macros;

#[cfg(test)]
mod tests {

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

    //#[test]
    #[cfg(feature="tokio")]
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn it_works() {

    }


}