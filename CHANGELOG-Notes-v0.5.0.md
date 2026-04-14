commit a21bce0b1487b3a21160b740215a6eb307a8abdc -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Apr 8 15:00:13 2026 +1200

    - Moved the “proceed” bool declaration into the “if state.pre_run()” scope in the run method of the std::ThreadActor implementation.

commit cf78360c1935687f953b57a0d28c3df21d52213d -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Apr 7 19:03:41 2026 +1200

    - Added the AsyncPanicHandler trait.
    
    - Changed the type of the err_fn parameter to &Arc<F> where F is an std::ops::Fn object in the spawn_catch_unwind, spawn_build_state_and_catch_unwind, build_spawn_and_catch_unwind and build_spawn_build_state_and_catch_unwind methods of the std::ThreadActor implementation.

commit edaf998bfb93193ee7fd7dad293a903ab8dd6466 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Thu Mar 26 17:41:12 2026 +1300

    - Replaced doc_auto_cfg with doc_cfg in the docsrs cfg_attr in the lib file.
    
    - Renamed spawn_built_thread to build_spawn, spawn_built_thread_and_build_state to build_spawn_and_build_state, spawn_built_thread_and_catch_unwind to build_spawn_and_catch_unwind and spawn_built_thread_build_state_and_catch_unwind to build_spawn_build_state_and_catch_unwind in the std::ThreadActor implementation.

commit f02245736a6389088d77554feb057b32960f9bcb -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Sat Mar 21 18:03:33 2026 +1300

    - Removed the where clauses from the ActorState and ActorStateAsync traits.
    
    - Added the ActorStateBuilderUnwindSafeAsync trait.
    
    - Renamed spawn_and_build method to spawn_and_build_state in the ThreadActor implementation.
    
    - Renamed spawn_build_and_catch_unwind method to spawn_build_state_and_catch_unwind in the ThreadActor implementation.
    
    - Added the spawn_built_thread, spawn_built_thread_and_build_state, spawn_built_thread_and_catch_unwind and the spawn_built_thread_build_state_and_catch_unwind methods to the ThreadActor implementation.
    
    - Made the run method of the ThreadActor implementation public.

commit d1d65d513c4144bd635412f91bd959d243e60ef0 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Mar 20 19:20:39 2026 +1300

    - Added the ActorStateUnwindSafeAsync trait.
    
    - Added the spawn_catch_unwind and spawn_build_and_catch_unwind methods to the ThreadActor implementation.

commit dfaaef3fc46038aa88e2722cc5da6271f52f5692 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Mon Mar 16 15:27:15 2026 +1300

    - Updated the package version to 0.5.0-alpha.
    
    - Added the ActorStateFlexible, ActorStateFlexibleAsync, ActorStateFlexibleDefault and ActorStateFlexibleDefaultAsync traits.
    
    - Fixed an issue where the ActorStateAsync trait was being included in the actor_state_flow module without the async-trait feature having been activated.
    
    - Moved the std_tread_actor_test test function and related objects out of the std mod file and into the added thread_actor_tests file.

commit c4d8dd59f3dd67e08751d8414115bd762d2d17e1 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Sat Mar 14 17:34:56 2026 +1300

    - Added the impl_pre_run_flow_async and impl_pre_run_flow_and_post_run_async macros.
    
    - Renamed the spawn_then_build_state method of the std::ThreadActor struct implementation to spawn_and_build and swapped the order of its generic parameters.

commit 79e6f9500ae8f2f9bf4d010b3d993707f358e6f8 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Mar 3 18:39:04 2026 +1300

    - Continued work on the ActorFlow enum.
    
    - Added the ActorStateFlow and ActorStateFlowAsync traits.

commit a8d9e3ca4505bdfe905d4d9318bca3220d0cc91c -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Feb 25 19:16:24 2026 +1300

    - Removed the dynosaur dependency.
    
    - Re-added and updated the async-trait dependency to version 0.1.89.
    
    - Added the ActorFlow enum.
    
    - Made the inclusion of features that depend on the async_trait dependency optional, based on the inclusion of this dependency as a feature.
    
    - Made a note that the cfg_attr section is currently invalid.
    
    - Removed the dead_code decoration from std::ThreadActor struct.
    
    - Added the spawn_then_build_state method to the std::ThreadActor implementation.

commit 2f75a488fdf88845407074cc8cf2f34900d38877 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Sep 5 17:40:47 2025 +1200

    - Disabled the async-trait dependancy.
    
    - Removed the CHANGELOG-Notes-v0.4.0.md file.
    
    - Replaced the async_trait decorating macro with a dynosaur decorating macro on ActorStateAsync trait.
