# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) (post version 0.2.0),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Version 0.5.0 (__/04/2026)

### Added

commit cf78360c1935687f953b57a0d28c3df21d52213d

- Added the AsyncPanicHandler trait.

commit edaf998bfb93193ee7fd7dad293a903ab8dd6466

-- Renamed spawn_built_thread to build_spawn, spawn_built_thread_and_build_state to build_spawn_and_build_state, spawn_built_thread_and_catch_unwind to build_spawn_and_catch_unwind and spawn_built_thread_build_state_and_catch_unwind to build_spawn_build_state_and_catch_unwind in the std::ThreadActor implementation.

 Renamed - Added in this version - std::ThreadActor -

commit f02245736a6389088d77554feb057b32960f9bcb

- Added the ActorStateBuilderUnwindSafeAsync trait.

-- Renamed spawn_and_build method to spawn_and_build_state in the ThreadActor implementation.

 Renamed - Added in this version - std::ThreadActor -

-- Renamed spawn_build_and_catch_unwind method to spawn_build_state_and_catch_unwind in the ThreadActor implementation.

 Renamed - Added in this version - std::ThreadActor -

-- Added the spawn_built_thread, spawn_built_thread_and_build_state, spawn_built_thread_and_catch_unwind and the spawn_built_thread_build_state_and_catch_unwind methods to the ThreadActor implementation.

- Added the build_spawn, build_spawn_and_build_state, build_spawn_and_catch_unwind and build_spawn_build_state_and_catch_unwind methods to the ThreadActor implementation.

Renamed - Added in this version - std::ThreadActor -

commit d1d65d513c4144bd635412f91bd959d243e60ef0

- Added the ActorStateUnwindSafeAsync trait.

-- Added the spawn_catch_unwind and spawn_build_and_catch_unwind methods to the ThreadActor implementation.

- Added the spawn_catch_unwind and spawn_build_state_and_catch_unwind to the ThreadActor implementation.

Renamed - Added in this version - std::ThreadActor -

commit dfaaef3fc46038aa88e2722cc5da6271f52f5692

- Added the ActorStateFlexible, ActorStateFlexibleAsync, ActorStateFlexibleDefault and ActorStateFlexibleDefaultAsync traits.

-- Fixed an issue where the ActorStateAsync trait was being included in the actor_state_flow module without the async-trait feature having been activated.

Added in this version

commit c4d8dd59f3dd67e08751d8414115bd762d2d17e1

- Added the impl_pre_run_flow_async and impl_pre_run_flow_and_post_run_async macros.

-- Renamed the spawn_then_build_state method of the std::ThreadActor struct implementation to spawn_and_build and swapped the order of its generic parameters.

Renamed - Added in this version - std::ThreadActor -

commit 79e6f9500ae8f2f9bf4d010b3d993707f358e6f8

- Added the ActorStateFlow and ActorStateFlowAsync traits.

commit a8d9e3ca4505bdfe905d4d9318bca3220d0cc91c

-- Re-added and updated the async-trait dependency to version 0.1.89.

commit a8d9e3ca4505bdfe905d4d9318bca3220d0cc91c

- Added the ActorFlow enum.

-- Added the spawn_then_build_state method to the std::ThreadActor implementation.

- Added the spawn_and_build_state method to the std::ThreadActor implementation.

Renamed - Added in this version - std::ThreadActor -



### Changed

commit a21bce0b1487b3a21160b740215a6eb307a8abdc

- Moved the “proceed” bool declaration into the “if state.pre_run()” scope in the run method of the std::ThreadActor implementation.

commit cf78360c1935687f953b57a0d28c3df21d52213d

-- Changed the type of the err_fn parameter to &Arc<F> where F is an std::ops::Fn object in the spawn_catch_unwind, spawn_build_state_and_catch_unwind, build_spawn_and_catch_unwind and build_spawn_build_state_and_catch_unwind methods of the std::ThreadActor implementation.

Added in this version

commit edaf998bfb93193ee7fd7dad293a903ab8dd6466

- Replaced doc_auto_cfg with doc_cfg in the docsrs cfg_attr in the lib file.

commit f02245736a6389088d77554feb057b32960f9bcb

-- Removed the where clauses from the ActorState and ActorStateAsync traits.

- Removed the where clauses from the ActorState and ActorStateAsync traits and made them derive from Sized trait instead.

-- Made the run method of the ThreadActor implementation public.

- Made the run method of the std::ThreadActor implementation public.

commit dfaaef3fc46038aa88e2722cc5da6271f52f5692

-- Updated the package version to 0.5.0-alpha.

commit dfaaef3fc46038aa88e2722cc5da6271f52f5692

- Moved the std_tread_actor_test test function and related objects out of the std mod file and into the added thread_actor_tests file.

commit 79e6f9500ae8f2f9bf4d010b3d993707f358e6f8

-- Continued work on the ActorFlow enum.

commit a8d9e3ca4505bdfe905d4d9318bca3220d0cc91c

-- Re-added and updated the async-trait dependency to version 0.1.89.

- Updated the async-trait dependency to version 0.1.89.

-- Made the inclusion of features that depend on the async_trait dependency optional, based on the inclusion of this dependency as a feature.

- Made the async-trait dependency optional.

- Made the inclusion of features that depend on the async-trait dependency optional.

-- Made a note that the cfg_attr section is currently invalid.

- Removed the dead_code decoration from std::ThreadActor struct.

commit 2f75a488fdf88845407074cc8cf2f34900d38877

-- Disabled the async-trait dependancy.

-- Removed the CHANGELOG-Notes-v0.4.0.md file.

-- Replaced the async_trait decorating macro with a dynosaur decorating macro on ActorStateAsync trait.

commit 5c6bbe835e21dd983fe1ce25e423a070bab1eced

- Updated the readme

commit 3e00373f5e5846f92e6320c733ba61cabc394943

- Ran cargo update
    
-- Updated the changelog
    
-- Updated the readme



### Deprecated



### Removed

commit a8d9e3ca4505bdfe905d4d9318bca3220d0cc91c

-- Removed the dynosaur dependency.


### Fixed



### Security



## Version 0.4.0 (08/08/2025)

### Added

- Added "concurrency" to the categories array field in the Cargo.toml file.



### Changed

- Updated the crates description in the Cargo.toml file.

- Moved the impl_pre_run_async, impl_post_run_async and the impl_pre_and_post_run_async macro definitions out of the tokio sub-module and into the base module.

- Made the async-trait dependency non-optional.

- Updated the readme.

- ActorStateBuilderAsync is no longer dependant the removed tokio feature.



### Removed

- Removed the tokio dependency, feature and sub-module.



### Fixed

- Fixed the homepage URL in the Cargo.toml file.



## Version 0.3.0 (01/05/2025)

### Added

- The tokio/entering module has been added which contains functions for dealing with entering runtime contexts.

- Added impl_mac_task_actor_built_state, ActorStateBuilder and ActorStateBuilderAsync.

- Added the Cargo.lock file.

- Added get_input, get_input_with_err, get_input_with_errs, get_input_async, get_input_with_err_async and get_input_with_errs_async macros.

- Added the std TwoPlusTwoActorState ThreadActor test to std/mod.



### Changed

- Updated the documentation of impl_mac_task_actor to reflect the fact that it no longer requires std::sync::Arc to be in module scope when used.

- In tokio::BlockingActor, std::ThreadActor and tokio::TaskActor all struct level generic parameters and constraints have been removed. In each struct implementation all methods named “new” have been renamed to “spawn” with “ST” generic parameters added. The original generic constraint has been replaced with “ST: ActorStateAsync + Send + 'static” or “ST: ActorState + Send + 'static” for each method. Also the “run” methods in each struct implementation have had an identically named and constrained generic parameter added. 

- impl_mac_task_actor and its documentation has been changed to reflect TaskActor and the removal of the HasInteractor trait.

- The renamed post_run and post_run_async methods are now run regardless of whether the (also renamed) pre_run or pre_run_async methods return true. This is the case for all actor implementations and meta-implementations.

- impl_mac_task_actor now only takes the $actor_type parameter and derives the type name of the actor state from it. Its documentation has also been updated.

- Constrained Self to Sized in ActorState.

- Constrained Self to Sized in ActorStateAsync.

- Renamed AsyncActorState to ActorStateAsync and updated the relevant parts of the project to reflect this change.

- Made the inclusion of the async-trait dependency dependant on the presence of the tokio feature.

- Made ActorStateAsync only be included if the tokio feature is enabled.

- Renamed tokio sub-module macros impl_default_on_enter_async to impl_pre_run_async, impl_default_on_exit_async to impl_post_run_async and impl_default_on_enter_and_exit_async to impl_pre_and_post_run_async and made them compatible with ActorStateAsync.

- The tokio minimum version has been updated to 1.44.2 and the async-trait minimum version has been updated to 0.1.88. Related dependencies have also been updated.

- Updated the readme

- Updated documentation

- Disabled the test module in the lib file.

- All methods named on_enter and on_exit have been renamed to pre_run and post_run and on_enter_async and on_exit_async have been renamed to pre_run_async and post_run_async respectively, whether they be in trait declarations, implementation blocks, macros or documentation.

- Updated the MIT License copyright year.



### Removed

- Removed the Tokio runtime actors and macro.

- Drop implementation blocks have been removed from the impl_mac_task_actor macro as well as all actor modules.

- Removed the impl_not_dropped_on_enter_async and impl_not_dropped_on_enter_and_default_exit_async macros.

- Removed the ActorInteractor and HasInteractor traits.

- Removed the tokio/oneshot_at module.

- Removed ActorFrontend

- Removed DroppedDetector

- Removed the impl_pub_sender, impl_new_sender, impl_interactor_clone, impl_actor_interactor and impl_actor_interactor_async macros.

- Removed the interactors sub-module and it contents from std.

- Removed the io sub-module and it contents from tokio.

- Removed the futures and delegate dependencies.

- Removed a rouge where keyword from the tokio/TaskActor implementaion.



## Version 0.2.0 (20/05/2024)

- Added a FUNDING.yml file.
- Stopped the .vscode and .idea directories from being tracked in the repository.
- Removed a random Cargo copy.toml file.
- Updated the edition in the cargo.toml form “2018” to “2021”.
- Made the std and tokio modules optional and dependant on their respective features.
- Spell checked some documentation above ActorInteractor.
- Removed old code.
- Remamed the “get_interactor_ref” method to “interactor” in ActorFrontend and updated this elsewhere.
- Remamed the “get_interactor” method to “interactor” in ActorInteractor and updated this elsewhere.
- Removed the messages module.
- Renamed “get_sender_ref” to “sender” in SenderInteractor.
- Renamed “get_unbounded_sender_ref” to “sender” in UnboundedSenderInteractor.
- Removed the single_shot module.
- Removed crossbeam
- Added a macros module with impl_pub_sender, impl_new_sender, impl_interactor_clone and impl_actor_interactor macros.
- Added an interactors module with an mpsc sub-module containing sender_interactor (SenderInteractor, channel) and sync_sender_interactor (SyncSenderInteractor, sync_channel) modules to the std module.
- Decorated std::ThreadActor and tokio::BlockingActor with #[allow(dead_code)].
- Added a message at the top of the tokio/interactors/broadcast/mod file.
- Under tokio/interactors the name of the module “mspc” has been changed to “mpsc”.
- Under tokio::interactors::mpsc SenderInteractor and UnboundedSenderInteractor have been practically re-implemented using macros from the macros module.
- Changed Act_rs to Act.rs in the readme.
- Added a macro called impl_actor_interactor_async for interactors that use async senders.
- Cleaned up std::ThreadActor, tokio::BlockingActor, tokio::interactors::mpsc::UnboundedSenderInteractor, tokio::RuntimeBlockingActor, tokio::RuntimeTaskActor and tokio::TaskActor.
- Replaced the impl_actor_interactor macro call with impl_actor_interactor_async in tokio::interactors::mpsc::SenderInteractor to prevent a situation where the send method on the sender object doesn’t actually get called in the input_default method definition.
- Cleaned up and corrected the documentation in the tokio::mac_task_actors module (impl_mac_runtime_task_actor, impl_mac_task_actor).
- The interactor method of the HasInteractor trait now returns a reference instead of a value.
- The has_not_dropped method of the DroppedIndicator struct has been renamed to not_dropped.
- BlockingActor, impl_mac_runtime_task_actor, impl_mac_task_actor, RuntimeBlockingActor, RuntimeTaskActor and TaskActor stucts and macros have been updated to be compatible with the change in the HasInteractor trait.
- Updated std::ThreadActor to be compatible with the change in the HasInteractor trait.
- Added comments to the DroppedIndicator methods.
- Cleaned up lib.rs.
- Removed the requirement that std::marker::PhantomData be in scope to use the impl_mac_runtime_task_actor and impl_mac_task_actor macros.
- Swapped the order of the $state_type and $interactor_type parameters in the impl_mac_runtime_task_actor and impl_mac_task_actor macros.
- Updated the documentation of impl_default_on_enter_async, impl_default_on_exit_async and impl_default_on_enter_and_exit_async.
- Added impl_not_dropped_on_enter_async and impl_not_dropped_on_enter_and_default_exit_async.
- Cleaned up the mod file of the tokio module.
- Changed the name of the generic parameter from “ST” to “SC” in RuntimeBlockingActor, RuntimeTaskActor and elsewhere.
- Removed the warning notices in the struct-level comments of RuntimeTaskActor and TaskActor.
- Replaced the “messages” keyword in the Cargo.toml with “pipeline”.
- Added a package.metadata.docs.rs section to the Cargo.toml indicating that I want docs.rs to build all the features and create a configuration argument called “docsrs”.
- Added links and badges to the readme and other changes.
- Added #![cfg_attr(docsrs, feature(doc_auto_cfg))] near to the top of the lib.rs file to ensure that every conditional feature gets documentation indicating what flag is required and whether or not the item you’re looking at is optional on the docs.rs website.
- Decorated the macros module declaration with #[doc(hidden)] in lib.rs.
- Appended “ (Optional)” to the ends of both the std and the tokio module level documentations (In case the package.metadata.docs.rs in the Cargo.toml and the #![cfg_attr(docsrs, feature(doc_auto_cfg))] in the lib.rs file thing doesn’t work).
- Added #![doc = include_str!("../README.md")] to the top of the lib.rs file.
- Corrected the spelling of “Initial” in the change-log (under “Version 0.1.0”).
- Corrected the HasInteractor documentation.
- Removed the notice about async traits being broken in the AsyncActorState documentation.

## Version 0.1.0 (20/07/2023)

- Initial release
