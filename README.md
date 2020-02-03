Flaws:

* Requires multiple steps for each passing of component data between systems
    * Could do a 'Published' list for each system and have a scheduling/pipelining graph based on that
* Single-threaded
    * See above, execute in parallel when there are separate "Strands" in a system
* Each passing of data requires serialization and deserialization, even when all data is internal
    * Could perhaps use a different kind of mechanism like capnpproto
        * WASM needs to add something like shared memory between host and plugin
* Lots of DoS opportunities right now
    * Limiting of allocation space, preempting of WASM in threads, limitations on number of
    systems and messages (And subscriptions)

