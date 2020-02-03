# Hacktor engine
Experiment/Personal project to try to build a type of game engine. 
Basically it's a game engine made for real-time multiplayer modding; there is no game at all without mods.
Mods are hotloaded WASM executables.

Current flaws and possible solutions:
* Requires multiple steps for each passing of component data between systems
    * Could do a 'Published' list for each system and have a scheduling/pipelining graph based on that
* Single-threaded
    * See above, execute in parallel when there are separate "Strands" in a system
* Each passing of data requires serialization and deserialization, even when all data is internal
    * Could perhaps use a different kind of mechanism like capnpproto
        * WASM needs to add something like shared memory between host and plugin
* Lots of DoS opportunities right now
    * Limiting of allocation space, preempting of WASM in threads, limitations on number of
    systems and messages (and subscriptions)
* How the hell would you do Audio? Other game resources? Streaming data?
    * Most are game resources downloaded and executed on the client.
    * Streaming from server to client is tricky, but not impossible. Needs more thought. 
* There is currently only one interactable "World" and no permissions
    * Yeah yeah I'm getting to it. Perhaps every user gets their own EC database and they have to explicitly enable permissions for others to write to or read from it.
* The name of the project sucks; it's too generic and edgy
    * Get good

Roadmap:
* Implement a really basic physics and rendering demo using Kiss3D and nalgebra
* Look into solving some of the flaws above
* Split into a client-server architecture, with message passing and client-side plugins
