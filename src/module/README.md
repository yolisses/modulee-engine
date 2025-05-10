# Module

The module processes multiple nodes. This is a performance-sensitive task, so it
relies on non-trivial optimizations:

- The nodes are stored in a topologically sorted vector, ensuring that each node
  is processed before its dependents (if any).
- The resulting values are stored in a vector instead of a hash map, reducing
  insertion time.

In general, allocating memory linearly improves performance by increasing the
cache hit rate.

Due to these additional constraints, a module passed to the system may be in an
invalid state. One possible solution is to sort the module's nodes on every
update. However, it seems simpler to ensure a valid state before the update,
given the possibility of recursion in modules. It is done with the methods named
with "prepare" Time will tell if this was the right choice.
