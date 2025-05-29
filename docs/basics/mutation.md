### Interior Mutability

The borrowing rules as introduced in the previous section are simple, but can be quite limiting—especially when multiple threads are involved. Following these rules makes communication between threads extremely limited and almost impossible, since no data that’s accessible by multiple threads can be mutated.
Luckily, there is an escape hatch: interior mutability. A data type with interior mutability slightly bends the borrowing rules. Under certain conditions, those types can allow mutation through an “immutable” reference.

In “Reference Counting”, we’ve already seen one subtle example involving interior mutability. Both Rc and Arc mutate a reference counter, even though there might be multiple clones all using the same reference counter.

As soon as interior mutable types are involved, calling a reference “immutable” or “mutable” becomes confusing and inaccurate, since some things can be mutated through both. The more accurate terms are “shared” and “exclusive”: a shared reference (&T) can be copied and shared with others, while an exclusive reference (&mut T) guarantees it’s the only exclusive borrowing of that T. For most types, shared references do not allow mutation, but there are exceptions.

### Cell

A std::cell::Cell<T> simply wraps a T, but allows mutations through a shared reference. To avoid undefined behavior, it only allows you to copy the value out (if T is Copy), or replace it with another value as a whole. In addition, it can only be used within a single thread.

### RefCell

Unlike a regular Cell, a std::cell::RefCell does allow you to borrow its contents, at a small runtime cost. A RefCell<T> does not only hold a T, but also holds a counter that keeps track of any outstanding borrows. If you try to borrow it while it is already mutably borrowed (or vice-versa), it will panic, which avoids undefined behavior. Just like a Cell, a RefCell can only be used within a single thread.
While Cell and RefCell can be very useful, they become rather useless when we need to do something with multiple threads. So let’s move on to the types that are relevant for concurrency.

### Mutex and RwLock

An RwLock or reader-writer lock is the concurrent version of a RefCell. An RwLock<T> holds a T and tracks any outstanding borrows. However, unlike a RefCell, it does not panic on conflicting borrows. Instead, it blocks the current thread—​putting it to sleep—​while waiting for conflicting borrows to disappear. We’ll just have to patiently wait for our turn with the data, after the other threads are done with it.
Borrowing the contents of an RwLock is called locking. By locking it we temporarily block concurrent conflicting borrows, allowing us to borrow it without causing data races.

A Mutex is very similar, but conceptually slightly simpler. Instead of keeping track of the number of shared and exclusive borrows like an RwLock, it only allows exclusive borrows.
