
d0ecfc2e2b052d34aae6090a306a33598bccb58b

Removed the Tokio runtime actors and macro.



072e5de153bc4de63028908830b091e9d8f38acd

- ActorFrontend, HasInteractor, ActorState and AsyncActorState no longer…
… require their “IN” generic parameters to implement ActorInteractor.

- ActorInteractor has been deprecated.

- ActorInteractor has been removed as a constraint from all actors.

- A new module called “entering” has been added which contains functions for dealing with entering runtime contexts.

- Drop implementation blocks have been removed from the impl_mac_task_actor macro as well as all actor modules.



6353d01dd55bdc75edaeae9d383f00b8532163b6

- Deprecated impl_actor_interactor, impl_actor_interactor_async, SenderI…
…nteractor (under both tokio and std) , SyncSenderInteractor, UnboundedSenderInteractor, channel and unbounded_channel.

- Removed “impl_actor_interactor!(SenderInteractor<T>);” and “impl_actor_interactor_async!(SenderInteractor<T>);” from both (tokio and std) the SenderInteractors implementation blocks.

- Removed “impl_actor_interactor!(SyncSenderInteractor<T>);” from the SyncSenderInteractor implementation block.

- Added the “enter” and “enter_mut_param” macros to the “tokio/entering” module.

- Added ActorIOInteractorClient, ActorIOInteractorServer, actor_io_interactors and their unbounded counterparts to the “tokio/interactors/mspc” module.




c0058be56329fc4c9dd59f3cffb0998901d0d586

Cleaned up the actor_io_interactors module, UnboundedActorIOInteractorCl…
…ient now implements Clone.



1f0bb62e611691ab4a0a5bebe9919e6565415372

Added documentation to the actor_io_interactors objects.



fcc0faa72af604df0b2e61dc02469e8a28807780

- Removed the DroppedIndicator reference parameter declarations from the…
… of all the ActorState and AsyncActorState methods.

- Removed all DroppedIndicator references and instantiations, including its Arc instance type, from all actor types as well as the impl_mac_task_actor, impl_default_on_enter_async and impl_default_on_exit_async macros.

- Updated the documentation of impl_mac_task_actor to reflect the fact that it no longer requires std::sync::Arc to be in module scope when used.

- Removed the impl_not_dropped_on_enter_async and impl_not_dropped_on_enter_and_default_exit_async macros.

- Added “new_pair” and “one_arcd” methods to DroppedIndicator and updated its documentation.



5bee7b31b99aacf8f2c61613bd4496a673e19992

- Removed the ActorInteractor and HasInteractor traits.
- Renamed the methods “on_enter” to “beginning” in the ActorState trait and in its affected actor implementations (tokio::BlockingActor and std::ThreadActor).

- Renamed the methods “on_enter_async” to “beginning_async” in the AsyncActorState trait and in its affected actor implementations and macros (impl_mac_task_actor and task_actor).

- Renamed the methods “on_exit” to “ending” in the ActorState trait and in its affected actor implementations (tokio::BlockingActor and std::ThreadActor).

- Renamed the methods “on_exit_async” to “ending_async” in the AsyncActorState trait and in its affected actor implementations and macros (impl_mac_task_actor and task_actor).

- Added enum CurrentActorState.

- Renamed DroppedIndicator to DroppedDetector.

- In tokio::BlockingActor, std::ThreadActor and tokio::TaskActor all struct level generic parameters and constraints have been removed in addition to the ActorFrontend implementation. In each struct implementation all methods named “new” have been renamed to “spawn” with “ST” generic parameters added. The original generic constraint has been replaced with “ST: AsyncActorState + Send + 'static” for each method. Also the “run” methods in each struct implementation have had an identically named and constrained generic parameter added.

- the interactors sub-module of the tokio module has been renamed to “io”.

- ActorIOInteractorClient has been renamed to ActorIOClient.

- ActorIOInteractorServer has been renamed to ActorIOServer.

- actor_io_interactor has been renamed to actor_io.

- UnboundedActorIOInteractorClient has been renamed to UnboundedActorIOClient.

- UnboundedActorIOInteractorServer has been renamed to UnboundedActorIOServer.

- unbounded_actor_io_interactors has been renamed to  unbounded_actor_io.

- Removed the SenderInteractor and UnboundedSenderInteractor structs as well as the channel and unbounded_channel functions that were under tokio::interactors::mspc.

- impl_mac_task_actor and its documentation has been changed to reflect TaskActor and the removal of the HasInteractor trait.

- impl_default_on_enter_async has been renamed to impl_default_beginning_async.

- impl_default_on_exit_async has been renamed to impl_default_ending_async.

- impl_default_on_enter_and_exit_async has been renamed to impl_default_beginning_and_ending_async.

- Removed the oneshot_at module.



3329567a4c0305016446ac543d6d69316170b331

- In this revision the end and end_async methods are always called befor…
…e an actor terminates as opposed to the on_exit and on_exit_async methods which were only called if the on_start and on_start_async methods in each actor implementation returned true.

- Every method that was called “beginning” is now called “start” and every method that was called “ending” is now called “end”. The same situation is true for all of the async variants of these methods.

- The CurrentActorState has been renamed to CurrentActorPhase and its variants are now “Start”, “Run” and “End”. Also is_start, is_run and is_end methods have been added to its implementation.

- impl_mac_task_actor now only takes the $actor_type parameter and derives the type name of the actor state from it. Its documentation has also been updated.




8ddb0409cef0357a34fd9d7e91861a7460ef60ff

Disabled the broadcast sub-module in tokio/io.



8e77e9baf4b963709b907370a1afda89e6d2b145

Added impl_mac_task_actor_built_state, ActorStateBuilder and AsyncActorS…
…tateBuilder.



055e503bdf124d2a7d588ce59a2939e941c87c4a

- The enter macro now treats the provided expression as a literal expres…
…sion as opposed to a function.

- The enter_mut_param macro has been disabled.



af0ff5c8cd546531969868ee1eb385adad2ad2b4

The impl_mac_task_actor_built_state macro now requires that the build_as…
…ync method of the provided state_builder object returns an Option object containing the actor state.



61ca16b57c073ee3a32f7e76b7778359117ac006

- Removed ActorFrontend

- Updated ActorState: renamed the start method declaration to on_start and the end method declaration to on_end. Constrained Self to Sized. Updated dependant object definitions.

- Updated ActorSyncState: renamed the start_async method declaration to on_start_async and the end_async method declaration to on_end_async. Constrained Self to Sized. Updated dependant object definitions.

- Removed CurrentActorPhase

- The build method declaration of ActorStateBuilder now requires that Option<T> be returned.

- The build method declaration of AsyncActorStateBuilder now requires that Option<T> be returned and is now async.

- Removed DroppedDetector

- Removed the impl_pub_sender, impl_new_sender, impl_interactor_clone, impl_actor_interactor and impl_actor_interactor_async macros.

- Removed the interactors sub-module and it contents from std.

- Removed the io sub-module and it contents from tokio.

- Updated documentation



2609fd0f4ee923fbd4141b1184369199321b7ba7

- Renamed AsyncActorState to ActorStateAsync and updated the relevant pa…
…rts of the project to reflect this change.

- Renamed AsyncActorStateBuilder to ActorStateBuilderAsync.

- Renamed the tokio/mac_task_actor module to tokio/mac_task_actors.



c0f36d073ecb445e1ec153e5c1a096d363ac0286

- Added the Cargo.lock file.

- Updated the crates version string to "0.3.0-beta".

- Removed the futures and delegate dependencies and updated the remaining tokio and async-trait dependencies to versions 1.43.0 and 0.1.85 respectively.

- Made the inclusion of the async-trait dependency dependant on the presence of the tokio feature.

- Updated some documentation.

- In ActorState renamed on_start to on_started and on_end to on_ending and updated the relevant parts of the project to reflect these changes.

- In ActorStateAsync renamed on_start_async to on_started_async and on_end_async to on_ending_async and updated the relevant parts of the project to reflect these changes.

- Made ActorStateAsync and ActorStateBuilderAsync only be included if the tokio feature is enabled.

- Made impl_mac_task_actor and impl_mac_task_actor_built_state compatible with ActorStateAsync.

- Renamed impl_default_on_start_async to impl_on_started_async, impl_default_on_end_async to impl_on_ending_async and impl_default_on_start_and_end_async to impl_on_started_and_ending_async and made them compatible with ActorStateAsync.



7c2f409f93eb37c95f7608ad70ce2a3b6cf28f1c

- Removed another rouge where keyword.



f69287ed812d30a742cca518d5628feaae4d0002

- Added get_input, get_input_with_err, get_input_with_errs, get_input_as…
…ync, get_input_with_err_async and get_input_with_errs_async macros.



e8b7bf7403924985822d050f19157665c14e1a7e

- The tokio minimum version has been updated to 1.44.2 and the async-tra…
…it minimum version has been updated to 0.1.88. Related dependencies have also been updated.

- In the tokio/entering module runtime_enter_param function signature param is now passed by move. runtime_enter_param_ref has been added. runtime_enter_mut_param has been renamed to runtime_enter_param_mut. In the handle_enter_param function signature param is now passed by move. handle_enter_param_ref has been added. handle_enter_mut_param has been renamed to handle_enter_param_mut.

- Updated the documentation



8728324952b70d1c68e9af9212702275ff9f8dd1

- Updated the readme

- Updated documentation



c75418f0e274214b4059f07ad5ca4f295daac8bc

- Updated the readme

- Added a test in std/mod.



7994f5503a851003e8f624f98774f33f6bc238e0

- Updated the readme

- Disabled the test module in the lib file.

- Updated the std TwoPlusTwoActorState ThreadActor test in std/mod.



b9dee061de9fe5d69db2c54fe82532fe4c395682

- Updated the readme

- All methods named on_started and on_ending have been renamed to pre_run and post_run and on_started_async and on_ending_async have been renamed to pre_run_async and post_run_async respectively, whether they be in trait declarations, implementation blocks, macros or documentation.

- Updated the documentation



1c5007e4ea6897d1e3b8e4ecf7b171d87cfc32f0

- Updated the readme and the documentation.













