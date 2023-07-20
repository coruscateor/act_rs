# Act_rs

Act_rs is an actor library.

## What is an actor?

An actor is an object that runs in its own thread or task. You usually might communicate with it via a message queue.

They have their own state, so you can just send a message indicating what you want done to a particular actor without necessarily having to move everything.

## Pipelining

Another potential benefit of actors is they can make it reasonably straight-forward to setup pipelines.

You might setup a pipeline to divide work into stages to be performed on different threads and to keep message queue sizes under control.


## Potential Issues When Setting Up

When setting your actors with input message queues, you should:

- Make sure your actor doesn't wait excessively or get stuck (wait indefinitely).
- If you are using actors as part of a pipeline; watch out for message loops.
- Make sure that the actor doesn't exit unexpectedly, especially with messages still in the input queue.

If you follow these guidelines you should have a pleasant time using Act_rs.

## Todo:

- Add more documentation
- Add code examples
- Add some tests
- Cleanup the code
- Solidify the API for 1.0
- Link to example projects
- Make actor implementations compile-time features,


## Possibilities:

- Add other async framework implementations such as std_async.

## Coding Style

This project uses a coding style the emphasises the use of white space over keeping the line and column counts as low as possible.

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


