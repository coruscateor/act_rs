# Changelog

## Version 0.2.0 (_/05/2024)

- Added a FUNDING.yml file.
- Stopped the .vscode and .idea directories from being tracked in the repository.
- Removed a random Cargo copy.toml file.
- Updated the edition in the cargo.toml form “2018” to “2021”.

-- Made the tokio and crossbeam dependences and associated modules optional.

- Made the std and tokio modules optional and dependant of their respective features.

- Spell checked some documentation above ActorInteractor.

- Removed old code.
- Remamed the “get_interactor_ref” method to “interactor” in ActorFrontend and updated this elsewhere.
- Remamed the “get_interactor” method to “interactor” in ActorInteractor and updated this elsewhere.
- Removed the messages module.

- Renamed “get_sender_ref” to “sender” in SenderInteractor.

- Renamed “get_unbounded_sender_ref” to “sender” in UnboundedSenderInteractor.

-- Disabled impl_default_on_enter_async, impl_default_on_exit_async and impl_default_on_enter_and_exit_async in tokio/mac_task_actors.rs.

- Removed the single_shot module.

987fa0b48a9a270750850dc6fdcd65df09dc3532

-- Disabled crossbeam (will be removed)

- Removed crossbeam
- Added a macros module with impl_pub_sender, impl_new_sender, impl_interactor_clone and impl_actor_interactor macros.
- Added an interactors module with an mpsc sub-module containing sender_interactor (SenderInteractor, channel) and sync_sender_interactor (SyncSenderInteractor, sync_channel) modules to the std module.
- Decorated std::ThreadActor and tokio::BlockingActor with #[allow(dead_code)].
- Added a message at the top of the tokio/interactors/broadcast/mod file.
- Under tokio/interactors the name of the module “mspc” has been changed to “mpsc”.
- Under tokio::interactors::mpsc SenderInteractor and UnboundedSenderInteractor have been practically re-implemented using macros from the macros module.

-- impl_default_on_enter_async, impl_default_on_exit_async and impl_default_on_enter_and_exit_async have been re-enabled with usage advice each added to their documentation.

-- impl_default_on_enter_async, impl_default_on_exit_async and impl_default_on_enter_and_exit_async have had usage advice added to each of their respective documentation sections.

c53838055f7a281b7cbebc8f89d4a22c73cbbf22

- Changed Act_rs to Act.rs in the readme.

35a1b28a3a89b621419a395515768e6931173c3e

- Added a macro called impl_actor_interactor_async for interactors that use async senders.
- Cleaned up std::ThreadActor, tokio::BlockingActor, tokio::interactors::mpsc::UnboundedSenderInteractor, tokio::RuntimeBlockingActor, tokio::RuntimeTaskActor and tokio::TaskActor.
- Replaced the impl_actor_interactor macro call with impl_actor_interactor_async in tokio::interactors::mpsc::SenderInteractor to prevent a situation where the send method on the sender object doesn’t actually get called in the input_default method definition.
- Cleaned up and corrected the documentation in the tokio::mac_task_actors module (impl_mac_runtime_task_actor, impl_mac_task_actor).

a633f91ad65081da5eec9b5b9a7a1b35f862b693

- The interactor method of the HasInteractor trait now returns a reference instead of a value.
- The has_not_dropped method of the DroppedIndicator struct has been renamed to not_dropped.
- BlockingActor, impl_mac_runtime_task_actor, impl_mac_task_actor, RuntimeBlockingActor, RuntimeTaskActor and TaskActor stucts and macros have been updated to be compatible with the change in the HasInteractor trait.

0e76d15b7fccbe78088753232e05f581f7cca0ca

- Updated std::ThreadActor to be compatible with the change in the HasInteractor trait.

91437a3529e37e6fe4d40f99a5c2f02e0dd48938

-- Updated the readme.

- Added comments to the DroppedIndicator methods.
- Cleaned up lib.rs.
- Removed the requirement that std::marker::PhantomData be in scope to use the impl_mac_runtime_task_actor and impl_mac_task_actor macros.
- Swapped the order of the $state_type and $interactor_type parameters in the impl_mac_runtime_task_actor and impl_mac_task_actor macros.

- Updated the documentation of impl_default_on_enter_async, impl_default_on_exit_async and impl_default_on_enter_and_exit_async.
- Added impl_not_dropped_on_enter_async and impl_not_dropped_on_enter_and_default_exit_async.
- Cleaned up the mod file of the tokio module.
- Changed the name of the generic parameter from “ST” to “SC” in RuntimeBlockingActor, RuntimeTaskActor and elsewhere.
- Removed the warning notices in the struct-level comments of RuntimeTaskActor and TaskActor.

1a5efc1ec1fda43e46c470622ecaba75f42ab33c

- Replaced the “messages” keyword in the Cargo.toml with “pipeline”.
- Added a package.metadata.docs.rs section to the Cargo.toml indicating that I want docs.rs to build all the features and create a configuration argument called “docsrs”.

- Added links and badges to the readme and other changes.

- Added #![cfg_attr(docsrs, feature(doc_auto_cfg))] near to the top of the lib.rs file to ensure that every conditional feature gets documentation indicating what flag is required and whether or not the item you’re looking at is optional on the docs.rs website.
- Decorated the macros module declaration with #[doc(hidden)] in lib.rs.
- Appended “ (Optional)” to the ends of both the std and the tokio module level documentations (In case the package.metadata.docs.rs in the Cargo.toml and the #![cfg_attr(docsrs, feature(doc_auto_cfg))] in the lib.rs file thing doesn’t work).
- Added #![doc = include_str!("../README.md")] to the top of the lib.rs file.

c043d280e378a64397981c508a887ef62219d9aa

-- Updated the readme with more details.

- Corrected the spelling of “Initial” in the change-log (under “Version 0.1.0”).

0ef9277654333f7771e971fa71e6fba70b0bfc69

-- Updated the comments for the impl_mac_runtime_task_actor and impl_mac_task_actor macros.



61e4c2d0cff4a6c5be1d27b1438970609de67071



- Corrected the HasInteractor documentation.

- Removed the notice about async traits being broken in the AsyncActorState documentation.


f48c920225d63955f6291c2df00792dd909360a3

## Version 0.1.0 (20/07/2023)

- Initial release
