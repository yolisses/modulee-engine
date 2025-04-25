# Module

Module does the multiple nodes processing. It's a performance sensitive task, so it depends on non trivial optimizations:

- The nodes are stored in a topologically sorted vector, so that every node is
  processed before its dependents (if any).
- The resulting values are saved in vector instead of a hash map decreasing the insertion time.

In general terms, allocating memory linearly increases performance by increasing the cache hits rate.
