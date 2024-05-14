#![doc = include_str!("../README.md")]

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod dropped_indicator;

pub use dropped_indicator::*;

mod actor_state;

pub use actor_state::*;

//feature="doc"

//#[cfg(feature="std")] //, doc))]
//#[doc = "std"]
//#[doc(cfg(feature="std"))]
//#[cfg(any(feature="std", doc))]
#[cfg(feature="std")]
pub mod std;

//#[cfg(any(feature="tokio", doc))]
#[cfg(feature="tokio")]
pub mod tokio;

mod actor_frontend;

pub use actor_frontend::*;

#[doc(hidden)]
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

    struct IntTestIncrement
    {
    }

    impl IntTestIncrement
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