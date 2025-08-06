<div align="center">

# Act.rs

[![Crates.io](https://img.shields.io/crates/v/act_rs)](https://crates.io/crates/act_rs)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue)](#license)
[![Downloads](https://img.shields.io/crates/d/act_rs)](https://crates.io/crates/act_rs)
[![Docs](https://docs.rs/act_rs/badge.svg)](https://docs.rs/act_rs/latest/act_rs/)
[![Twitch Status](https://img.shields.io/twitch/status/coruscateor)](https://www.twitch.tv/coruscateor)

[X](https://twitter.com/Coruscateor) | 
[Twitch](https://www.twitch.tv/coruscateor) | 
[Youtube](https://www.youtube.com/@coruscateor) | 
[Mastodon](https://mastodon.social/@Coruscateor) | 
[GitHub](https://github.com/coruscateor) | 
[GitHub Sponsors](https://github.com/sponsors/coruscateor)

Act.rs is a minimal actor framework.

</div>

<br />

## What Is An Actor?

An actor is an object that runs in its own thread or task. You would usually communicate with it via channels.

Actors have their own state, so ideally you just send a them messages indicating what you want done and you shouldn't necessarily need to move everything to do this work into the scope of each individual actor.

## Tokio Functionality

The Tokio functionality has been moved.

<br />

## A Basic Example

```rust

    //Adapted from the std TwoPlusTwoActorState ThreadActor test. 

    //use crate::ActorState;

    use act_rs::ActorState;

    use std::sync::mpsc::{Sender, channel};

    //use super::*;

    use act_rs::std::ThreadActor;

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
            
        }

    }

    fn main()
    {

        let (sender, receiver) = channel();

        ThreadActor::spawn(TwoPlusTwoActorState::new(sender));

        let res = receiver.recv().expect("Error: Message not delivered");

        println!("{}", res);

    }

```

<br />

In the above example an actor sends a String message to the main thread which prints it out.

Note that the actor continues or stops running depending on whether you return true or false from the run method. This is the run phase of the actor. There are also pre-run and post-run phases represented by pre_run and post_run methods which can also be manually implemented.

This is also the case with async actors as well, but each method you implement has an additional "_async" on the end of its name.

Lastly Act.rs actors do not handle communications or errors by default. It is left up to you to decide how to handle these issues.

<br />


## An Overview

You create a state struct that contains the state of your actor.

This state struct should implement either ActorState or ActorStateAsync depending on whether or not you want the actor to be async compatible (Macro generated actors don't have this requirement and the state stuct can implement the required methods in its impl block).

Finally pass the state into the actor spawn method and there you have your actor, which basically runs until its run method returns false.

<br />

## Pipelining

Another potential benefit of actors is that they can be used to help you setup pipelines.

You would setup a pipeline to divide work into stages which potentially would be performed on different threads depending on what kind of actors you chose among other things.

<br />

## Potential Issues When Setting Up

When setting up your actors with input channels or message queues, you should:

- Make sure that your actors don't wait excessively or get stuck (wait indefinitely) when doing work.
- If you are using actors as part of a pipeline; watch out for message loops.
- Make sure that your actors don't exit unexpectedly.
- Check for bottlenecks and possibly implement a way to add actors and remove actors from a particular  stage or channel dynamically (pipelines).

If you follow these guidelines you should have a productive time using Act.rs.

<br />

## Examples

- [Req It](https://github.com/coruscateor/req_it/blob/latest/src/actors/graphql_actor.rs)

- [Escape It](https://github.com/coruscateor/escape_it/blob/latest/src/conversion_actor.rs)

- [Act.rs Async Traits Test](https://github.com/coruscateor/act_rs_async_traits_test/tree/latest)

- [Mapage Types Viewer](https://github.com/coruscateor/mapage_types_viewer)

<br />

## Features

| Feature     | Description |
| ----------- | ----------- |
| std   | Enable std based actors.|

<br />

## Todo

- Add more documentation
- Add examples
- Add tests
- Cleanup the code
- Add methods to all actor structs and macros which allow you to construct the actor-state in the actors thread or task, passing in any necessary parameters in order to do this e.g. channel sender and receiver objects.
- Make the inclusion of the async_trait dependency and dependant types optional.

<br />

## Coding Style

This project uses a coding style that emphasises the use of white space over keeping the line and column counts as low as possible.

So this:

```rust

fn bar() {}

fn foo()
{

    bar();

}

```

Not this:

```rust

fn bar() {}

fn foo()
{
    bar();
}

```

<br/>

## License

Licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0 (see also: https://www.tldrlegal.com/license/apache-license-2-0-apache-2-0))
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT (see also: https://www.tldrlegal.com/license/mit-license))

at your discretion

<br/>

## Contributing

Please clone the repository and create an issue explaining what feature or features you'd like to add or bug or bugs you'd like to fix and perhaps how you intend to implement these additions or fixes. Try to include details though it doesn't need to be exhaustive and we'll take it from there (dependant on availability).

<br/>

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.


