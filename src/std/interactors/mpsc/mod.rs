//!
//! Standard library Multi-producer, single consumer interators.
//!

mod sender_interactor;

pub use sender_interactor::*;

mod sync_sender_interactor;

pub use sync_sender_interactor::*;
