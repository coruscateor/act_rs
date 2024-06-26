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

Act.rs is an actor library built to be used with the standard library and Tokio.

</div>

<br />

## What Is An Actor?

An actor is an object that runs in its own thread or task. You usually might communicate with it via a message queue.

Actors have their own state, so you can just send a message indicating what you want done to a particular actor without necessarily having to move everything to do it into its scope.

<br />

## Components Of An Actor

Act.rs actors have these three essential components:

1. The actor-state
2. The interactor - can be part of #1
3. The actor itself, which has two components of its own:
    1. The front-end - what users of actors interact with, contains the interactor or a clone of it.
    2. The back-end - The thread/task were the actor does its work. The actor-state is moved into this scope.

<br />

## Putting The Components Together

Create a state struct that contains the state of your actor, this includes an interactor.

This state struct should implement either ActorState or AsyncActorState depending on whether or not the actor is async (Macro generated actors don't have this requirement and the state can implement the required methods directly)

The interactor is exposed via HasInteractor which is required by both ActorState and AsyncActorState.

Finally pass the state into the actor constructor and there you have your actor (see the examples).

<br />

## Pipelining

Another potential benefit of actors is they can make it reasonably straight-forward to setup pipelines.

You might setup a pipeline to divide work into stages to be performed on different threads.

<br />

## Potential Issues When Setting Up

When setting up your actors with input message queues, you should:

- Make sure your actor doesn't wait excessively or get stuck (wait indefinitely) when doing work.
- If you are using actors as part of a pipeline; watch out for message loops.
- Make sure that the actor doesn't exit unexpectedly.

If you follow these guidelines you should have a pleasant time using Act.rs.

<br />

## Examples

[Req It](https://github.com/coruscateor/req_it/blob/latest/src/actors/graphql_actor.rs)

[Escape It](https://github.com/coruscateor/escape_it/blob/latest/src/conversion_actor.rs)

[Act.rs Async Traits Test](https://github.com/coruscateor/act_rs_async_traits_test/tree/latest)

<br />

## Features

| Feature     | Description |
| ----------- | ----------- |
| tokio | Enable Tokio based actors and interactors. |
| std   | Enable std based actors and interactors.|

<br />

## Todo

- Add more documentation
- Add more examples
- Add some tests
- Cleanup the code
- Solidify the API for 1.0.
- Add methods to all actor structs and macros which allow you to construct the actor-state in the actors thread, passing in any necessary parameters in order to do this e.g. the actors interactor.
- Improve code reuse
- Remove the requirement that the actor-state should possess the interactor (and make a reference to it available). When using channels for interaction it makes sense that the send side should only be in the front-end of the actor.

<br />

## Possibilities:

- Rename the required actor-state methods methods on_enter, on_exit as well as their async counterparts to something a bit more appropriate (particularly in regards to the point about in-actor-thread state-constructors in the Todo list).
- Add other async framework implementations such as [smol](https://crates.io/crates/smol).

<br />

## Coding Style

This project uses a coding style that emphasises the use of white space over keeping the line and column counts as low as possible.

So this:

```rust
fn foo()
{

    bar();

}

```

Not this:

```rust
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


