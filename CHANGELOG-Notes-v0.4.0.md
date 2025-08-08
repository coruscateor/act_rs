902c6f4e0738a9dd294585e295bca49be4fe2472 -

- Updated the version string "0.4.0-alpha".



5f5bd1d408c82a95fc8125025044a4bad75efda9 -

- Updated the crates description.

- Made the the async_trait dependency independent of the tokio feature in regards to the ActorStateAsync trait.

- Disabled the tokio module and feature.

- Moved the impl_pre_run_async, impl_post_run_async and the impl_pre_and_post_run_async macro definitions out of the tokio sub-module and into the base module.



81396a881a9a0d4a473e89b82cf902a719cc5b8b -

Updated the crates description in the Cargo.toml and the readme.



95c58dfc79d1e48c866e864d36c650fbda0d8a61 -

- Made the async-trait dependency non-optional.



5ec9c9b94b8f1c891d4dede5189f5ef974edd031 -

- Fixed the homepage URL in the Cargo.toml file.

- Removed the disabled tokio dependency, feature and sub-module.

- Updated the readme.

- ActorStateBuilderAsync no longer dependant the removed tokio feature.



