### Shared Ownership and Reference Counting

So far we’ve looked at transferring ownership of a value to a thread using a move closure (“Threads in Rust”) and borrowing data from longer-living parent threads (“Scoped Threads”). When sharing data between two threads where neither thread is guaranteed to outlive the other, neither of them can be the owner of that data. Any data shared between them will need to live as long as the longest living thread.

### Statics

There are several ways to create something that’s not owned by a single thread. The simplest one is a static value, which is “owned” by the entire program, instead of an individual thread.
A static item has a constant initializer, is never dropped, and already exists before the main function of the program even starts. Every thread can borrow it, since it’s guaranteed to always exist.

### Leaking

Another way to share ownership is by leaking an allocation. Using Box::leak, one can release ownership of a Box, promising to never drop it. From that point on, the Box will live forever, without an owner, allowing it to be borrowed by any thread for as long as the program runs.
Note how the 'static lifetime doesn’t mean that the value lived since the start of the program, but only that it lives to the end of the program. The past is simply not relevant. The downside of leaking a Box is that we’re leaking memory. We allocate something, but never drop and deallocate it. This can be fine if it happens only a limited number of times. But if we keep doing this, the program will slowly run out of memory.

### Reference Counting

To make sure that shared data gets dropped and deallocated, we can’t completely give up its ownership. Instead, we can share ownership. By keeping track of the number of owners, we can make sure the value is dropped only when there are no owners left. The Rust standard library provides this functionality through the std::rc::Rc type, short for “reference counted.” It is very similar to a Box, except cloning it will not allocate anything new, but instead increment a counter stored next to the contained value. Both the original and cloned Rc will refer to the same allocation; they share ownership.
Dropping an Rc will decrement the counter. Only the last Rc, which will see the counter drop to zero, will be the one dropping and deallocating the contained data.

As it turns out, Rc is not thread safe. If multiple threads had an Rc to the same allocation, they might try to modify the reference counter at the same time, which can give unpredictable results.
Instead, we can use std::sync::Arc, which stands for "atomically reference counted". It's identical to Rc, except it guarantees that modifications to the reference counter are indivisible atomic operations,
making it safe to use it with multiple threads.

### Borrowing and Data Races

- #### Immutable Borrowing

Borrowing something with & gives an immutable reference. Such a reference can be copied. Access to the data it references is shared between all copies of such a reference. As the name implies, the
compiler doesn’t normally allow you to mutate something through such a reference, since that might affect other code that’s currently borrowing the same data.

- #### Mutable borrowing

Mutable borrowing Borrowing something with &mut gives a mutable reference. A mutable borrow guarantees it’s the only active borrow of that data. This ensures that mutating the data will not change anything that other code is currently looking at.
