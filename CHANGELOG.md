# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) (post version 0.2.0),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Version 0.3.0 (__/04/2025)

### Added

072e5de153bc4de63028908830b091e9d8f38acd

-- A new module called “entering” has been added which contains functions for dealing with entering runtime contexts.

- The tokio/entering module has been added which contains functions for dealing with entering runtime contexts.

6353d01dd55bdc75edaeae9d383f00b8532163b6

-- Added the “enter” and “enter_mut_param” (Removed) macros to the “tokio/entering” module.



-- Added ActorIOInteractorClient (Removed), ActorIOInteractorServer (Removed), actor_io_interactors (Removed) and their unbounded counterparts to the “tokio/interactors/mspc” module (Removed).



1f0bb62e611691ab4a0a5bebe9919e6565415372

-- Added documentation to the actor_io_interactors objects (Removed).



fcc0faa72af604df0b2e61dc02469e8a28807780

-- Added “new_pair” and “one_arcd” methods to DroppedIndicator and updated its documentation. (Removed)



5bee7b31b99aacf8f2c61613bd4496a673e19992

-- Added enum CurrentActorState. (Removed)



8e77e9baf4b963709b907370a1afda89e6d2b145

-- Added impl_mac_task_actor_built_state, ActorStateBuilder and AsyncActorStateBuilder (Renamed to ActorStateBuilderAsync: 
2609fd0f4ee923fbd4141b1184369199321b7ba7, point 2).

- Added impl_mac_task_actor_built_state, ActorStateBuilder and ActorStateBuilderAsync.



c0f36d073ecb445e1ec153e5c1a096d363ac0286

- Added the Cargo.lock file.

f69287ed812d30a742cca518d5628feaae4d0002

- Added get_input, get_input_with_err, get_input_with_errs, get_input_async, get_input_with_err_async and get_input_with_errs_async macros.

c75418f0e274214b4059f07ad5ca4f295daac8bc

-- Added a test in std/mod.

- Added the std TwoPlusTwoActorState ThreadActor test to std/mod.



### Changed

072e5de153bc4de63028908830b091e9d8f38acd

-- ActorFrontend (Removed), HasInteractor (Removed), ActorState and AsyncActorState (Renamed to ActorStateAsync) no longer require their “IN” generic parameters to implement ActorInteractor. (ActorState and ActorStateAsync are no longer generic) (Remove)



-- ActorInteractor has been deprecated. (Removed)



6353d01dd55bdc75edaeae9d383f00b8532163b6

-- Deprecated impl_actor_interactor (Removed), impl_actor_interactor_async (Removed), SenderInteractor (under both tokio and std) (Removed), SyncSenderInteractor (Removed), UnboundedSenderInteractor (Removed), channel (Removed) and unbounded_channel (Removed).



c0058be56329fc4c9dd59f3cffb0998901d0d586

-- Cleaned up the actor_io_interactors module (Removed), UnboundedActorIOInteractorClient now implements Clone (Removed).



fcc0faa72af604df0b2e61dc02469e8a28807780

- Updated the documentation of impl_mac_task_actor to reflect the fact that it no longer requires std::sync::Arc to be in module scope when used.



5bee7b31b99aacf8f2c61613bd4496a673e19992

-- Renamed the methods “on_enter” to “beginning” in the ActorState trait and in its affected actor implementations (tokio::BlockingActor and std::ThreadActor) (Now pre_run).



-- Renamed the methods “on_enter_async” to “beginning_async” in the AsyncActorState trait and in its affected actor implementations and macros (impl_mac_task_actor and task_actor) (Now ActorStateAsync and pre_run_async).



-- Renamed the methods “on_exit” to “ending” in the ActorState trait and in its affected actor implementations (tokio::BlockingActor and std::ThreadActor) (Now post_run).



-- Renamed the methods “on_exit_async” to “ending_async” in the AsyncActorState trait and in its affected actor implementations and macros (impl_mac_task_actor and task_actor) (Now ActorStateAsync and post_run_async).



-- Renamed DroppedIndicator to DroppedDetector (Removed).



-- In tokio::BlockingActor, std::ThreadActor and tokio::TaskActor all struct level generic parameters and constraints have been removed in addition to the ActorFrontend (Removed) implementation. In each struct implementation all methods named “new” have been renamed to “spawn” with “ST” generic parameters added. The original generic constraint has been replaced with “ST: AsyncActorState (Renamed) + Send + 'static” (Or “ST: ActorState (Renamed) + Send + 'static”) for each method. Also the “run” methods in each struct implementation have had an identically named and constrained generic parameter added. (Re-write)

- In tokio::BlockingActor, std::ThreadActor and tokio::TaskActor all struct level generic parameters and constraints have been removed. In each struct implementation all methods named “new” have been renamed to “spawn” with “ST” generic parameters added. The original generic constraint has been replaced with “ST: ActorStateAsync + Send + 'static” or “ST: ActorState + Send + 'static” for each method. Also the “run” methods in each struct implementation have had an identically named and constrained generic parameter added. 



-- the interactors sub-module of the tokio module has been renamed to “io”. (Removed)



-- ActorIOInteractorClient has been renamed to ActorIOClient. (Removed)



-- ActorIOInteractorServer has been renamed to ActorIOServer. (Removed)



-- actor_io_interactor has been renamed to actor_io. (Removed)



-- UnboundedActorIOInteractorClient has been renamed to UnboundedActorIOClient.  (Removed)



-- UnboundedActorIOInteractorServer has been renamed to UnboundedActorIOServer. (Removed)



-- unbounded_actor_io_interactors has been renamed to  unbounded_actor_io. (Removed)



- impl_mac_task_actor and its documentation has been changed to reflect TaskActor and the removal of the HasInteractor trait.



-- impl_default_on_enter_async has been renamed to impl_default_beginning_async. (Now impl_pre_run_async)



-- impl_default_on_exit_async has been renamed to impl_default_ending_async. (Now impl_post_run_async)



-- impl_default_on_enter_and_exit_async has been renamed to impl_default_beginning_and_ending_async. (Now impl_pre_and_post_run_async)



3329567a4c0305016446ac543d6d69316170b331

-- In this revision the end and end_async methods are always called before an actor terminates as opposed to the on_exit and on_exit_async methods which were only called if the on_start and on_start_async methods in each actor implementation returned true. (Renamed Methods etc)

- The renamed post_run and post_run_async methods are now run regardless of whether the (also renamed) pre_run or pre_run_async methods return true. This is the case for all actor implementations and meta-implementations.

-- Every method that was called “beginning” is now called “start” and every method that was called “ending” is now called “end”. The same situation is true for all of the async variants of these methods. (now pre_run and pre_run_async, post_run and post_run_async)



-- The CurrentActorState has been renamed to CurrentActorPhase and its variants are now “Start”, “Run” and “End”. Also is_start, is_run and is_end methods have been added to its implementation. (Removed)



- impl_mac_task_actor now only takes the $actor_type parameter and derives the type name of the actor state from it. Its documentation has also been updated.



8ddb0409cef0357a34fd9d7e91861a7460ef60ff

-- Disabled the broadcast sub-module in tokio/io. (See Removed)



055e503bdf124d2a7d588ce59a2939e941c87c4a

-- The enter macro now treats the provided expression as a literal expression as opposed to a function. (Irrelevant, added in this version.)



-- The enter_mut_param macro has been disabled. (Added in this version.)



af0ff5c8cd546531969868ee1eb385adad2ad2b4

-- The impl_mac_task_actor_built_state macro now requires that the build_async method of the provided state_builder object returns an Option object containing the actor state. (Added in this version.)



61ca16b57c073ee3a32f7e76b7778359117ac006

-- Updated ActorState: renamed the start method declaration to on_start and the end method declaration to on_end. Constrained Self to Sized. Updated dependant object definitions. (Address method declaration name changes elsewhere.)

- Constrained Self to Sized in ActorState.

-- Updated ActorSyncState (AsyncActorState?): renamed the start_async method declaration to on_start_async and the end_async method declaration to on_end_async. Constrained Self to Sized. Updated dependant object definitions. (Address method declaration name changes elsewhere.)

- Constrained Self to Sized in ActorStateAsync.

-- The build method declaration of ActorStateBuilder now requires that Option<T> be returned. (Added in this version.)

-- The build method declaration of AsyncActorStateBuilder (Now ActorStateBuilderAsync) now requires that Option<T> be returned and is now async. (Added in this version.)

-- Updated documentation (Addressed later)



2609fd0f4ee923fbd4141b1184369199321b7ba7

- Renamed AsyncActorState to ActorStateAsync and updated the relevant parts of the project to reflect this change.

-- Renamed AsyncActorStateBuilder to ActorStateBuilderAsync. (Added in this version.)



-- Renamed the tokio/mac_task_actor module to tokio/mac_task_actors. (Shows up as mac_task_actors in v0.2.0. Remove)



c0f36d073ecb445e1ec153e5c1a096d363ac0286

-- Updated the crates version string to "0.3.0-beta". (Irrelevant; remove)



- Made the inclusion of the async-trait dependency dependant on the presence of the tokio feature.



-- Updated some documentation. (Addressed later)



-- In ActorState renamed on_start to on_started and on_end to on_ending and updated the relevant parts of the project to reflect these changes. (Now pre_run and post_run, Addressed later.)



-- In ActorStateAsync renamed on_start_async to on_started_async and on_end_async to on_ending_async and updated the relevant parts of the project to reflect these changes. (Now pre_run_async and post_run_async, Addressed later.)



-- Made ActorStateAsync and ActorStateBuilderAsync (Added in this verion) only be included if the tokio feature is enabled.

- Made ActorStateAsync only be included if the tokio feature is enabled.



-- Made impl_mac_task_actor and impl_mac_task_actor_built_state compatible with ActorStateAsync. (A bit vague.)



-- Renamed impl_default_on_start_async (Was impl_default_on_enter_async)  to impl_on_started_async (Now impl_pre_run_async), impl_default_on_end_async (Was impl_default_on_exit_async) to impl_on_ending_async (Now impl_post_run_async) and impl_default_on_start_and_end_async (Was impl_default_on_enter_and_exit_async) to impl_on_started_and_ending_async (Now impl_pre_and_post_run_async) and made them compatible with ActorStateAsync. (Re-written below as latest changes not are mentioned elsewhere)

- Renamed tokio sub-module macros impl_default_on_enter_async to impl_pre_run_async, impl_default_on_exit_async to impl_post_run_async and impl_default_on_enter_and_exit_async to impl_pre_and_post_run_async and made them compatible with ActorStateAsync.



e8b7bf7403924985822d050f19157665c14e1a7e

- The tokio minimum version has been updated to 1.44.2 and the async-trait minimum version has been updated to 0.1.88. Related dependencies have also been updated.

-- In the tokio/entering module runtime_enter_param function signature param is now passed by move. runtime_enter_param_ref has been added. runtime_enter_mut_param has been renamed to runtime_enter_param_mut. In the handle_enter_param function signature param is now passed by move. handle_enter_param_ref has been added. handle_enter_mut_param has been renamed to handle_enter_param_mut. (Splitup between this category and the Added section.) (Added in this version.)

-- In the tokio/entering module runtime_enter_param function signature param is now passed by move. runtime_enter_mut_param has been renamed to runtime_enter_param_mut. In the handle_enter_param function signature param is now passed by move. handle_enter_mut_param has been renamed to handle_enter_param_mut. (Added in this version.)

-- In the tokio/entering module runtime_enter_param_ref and handle_enter_param_ref have been added. (Added in this version.)

-- Updated the documentation (Addressed later)



8728324952b70d1c68e9af9212702275ff9f8dd1

(I'll go with these:)

- Updated the readme

- Updated documentation



c75418f0e274214b4059f07ad5ca4f295daac8bc

-- Updated the readme (Remove)



7994f5503a851003e8f624f98774f33f6bc238e0

-- Updated the readme (Remove)

- Disabled the test module in the lib file.

-- Updated the std TwoPlusTwoActorState ThreadActor test in std/mod. (Remove)



b9dee061de9fe5d69db2c54fe82532fe4c395682

-- Updated the readme (Remove)

-- All methods named on_started and on_ending have been renamed to pre_run and post_run and on_started_async and on_ending_async have been renamed to pre_run_async and post_run_async respectively, whether they be in trait declarations, implementation blocks, macros or documentation.

- All methods named on_enter and on_exit have been renamed to pre_run and post_run and on_enter_async and on_exit_async have been renamed to pre_run_async and post_run_async respectively, whether they be in trait declarations, implementation blocks, macros or documentation.



-- Updated the documentation (Remove)



1c5007e4ea6897d1e3b8e4ecf7b171d87cfc32f0

-- Updated the readme and the documentation. (Remove)



### Removed

d0ecfc2e2b052d34aae6090a306a33598bccb58b

- Removed the Tokio runtime actors and macro.

072e5de153bc4de63028908830b091e9d8f38acd

-- ActorInteractor has been removed as a constraint from all actors. (Removed)



- Drop implementation blocks have been removed from the impl_mac_task_actor macro as well as all actor modules.



6353d01dd55bdc75edaeae9d383f00b8532163b6

-- Removed “impl_actor_interactor!(SenderInteractor<T>);” and “impl_actor_interactor_async!(SenderInteractor<T>);” from both (tokio and std) the SenderInteractors implementation blocks (Removed).



-- Removed “impl_actor_interactor!(SyncSenderInteractor<T>);” from the SyncSenderInteractor implementation block (Removed).

fcc0faa72af604df0b2e61dc02469e8a28807780

-- Removed the DroppedIndicator (Removed) reference parameter declarations from the of all the ActorState and AsyncActorState methods.



-- Removed all DroppedIndicator (Removed) references and instantiations, including its Arc instance type, from all actor types as well as the impl_mac_task_actor, impl_default_on_enter_async and impl_default_on_exit_async macros.



- Removed the impl_not_dropped_on_enter_async and impl_not_dropped_on_enter_and_default_exit_async macros.



5bee7b31b99aacf8f2c61613bd4496a673e19992

- Removed the ActorInteractor and HasInteractor traits.



-- Removed the SenderInteractor and UnboundedSenderInteractor structs as well as the channel and unbounded_channel functions that were under tokio::interactors::mspc (Removed).



-- Removed the oneshot_at module.

- Removed the tokio/oneshot_at module.



8ddb0409cef0357a34fd9d7e91861a7460ef60ff

-- Removed the broadcast sub-module in tokio/io. (See 61ca16b57c073ee3a32f7e76b7778359117ac006)



61ca16b57c073ee3a32f7e76b7778359117ac006

- Removed ActorFrontend

-- Removed CurrentActorPhase (Irrelevant; originally CurrentActorState, added in this version.)

- Removed DroppedDetector

- Removed the impl_pub_sender, impl_new_sender, impl_interactor_clone, impl_actor_interactor and impl_actor_interactor_async macros.

- Removed the interactors sub-module and it contents from std.

- Removed the io sub-module and it contents from tokio.



c0f36d073ecb445e1ec153e5c1a096d363ac0286

-- Removed the futures and delegate dependencies and updated the remaining tokio and async-trait dependencies to versions 1.43.0 and 0.1.85 respectively. (Updated dependancies addressed later.)

- Removed the futures and delegate dependencies.



7c2f409f93eb37c95f7608ad70ce2a3b6cf28f1c

-- Removed another rouge where keyword. (Where was the other one?)

- Removed a rouge where keyword from the tokio/TaskActor implementaion.




### Fixed




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
