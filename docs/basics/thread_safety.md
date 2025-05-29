### Atomics

The atomic types represent the concurrent version of a Cell, and are the main topic of Chapters 2 and 3. Like a Cell, they avoid undefined behavior by making us copy values in and out as a whole, without letting us borrow the contents directly. Unlike a Cell, though, they cannot be of arbitrary size. Because of this, there is no generic Atomic<T> type for any T, but there are only specific atomic types such as AtomicU32 and AtomicPtr<T>. Which ones are available depends on the platform, since they require support from the processor to avoid data races. (We’ll dive into that in Chapter 7.) Since they are so limited in size, atomics often don’t directly contain the information that needs to be shared between threads. Instead, they are often used as a tool to make it possible to share other—​often bigger—​things between threads. When atomics are used to say something about other data, things can get surprisingly complicated.

### UnsafeCell

An UnsafeCell is the primitive building block for interior mutability. An UnsafeCell<T> wraps a T, but does not come with any conditions or restrictions to avoid undefined behavior. Instead, its get() method just gives a raw pointer to the value it wraps, which can only be meaningfully used in unsafe blocks. It leaves it up to the user to use it in a way that does not cause any undefined behavior.
Most commonly, an UnsafeCell is not used directly, but wrapped in another type that provides safety through a limited interface, such as Cell or Mutex. All types with interior mutability—​including all types discussed above—​are built on top of UnsafeCell.

### Thread Safety: Send and Sync

The language uses two special traits to keep track of which types can be safely used across threads:

- #### Send

  A type is Send if it can be sent to another thread. In other words, if ownership of a value of that type can be transferred to another thread. For example, Arc<i32> is Send, but Rc<i32> is not.

- #### Sync
  A type is Sync if it can be shared with another thread. In other words, a type T is Sync if and only if a shared reference to that type, &T, is Send. For example, an i32 is Sync, but a Cell<i32> is not. (A Cell<i32> is Send, however.)

All primitive types such as i32, bool, and str are both Send and Sync. Both of these traits are auto traits, which means that they are automatically implemented for your types based on their fields. A struct with fields that are all Send and Sync, is itself also Send and Sync. The way to opt out of either of these is to add a field to your type that does not implement the trait. For that purpose, the special std::marker::PhantomData<T> type often comes in handy. That type is treated by the compiler as a T, except it doesn’t actually exist at runtime. It’s a zero-sized type, taking no space.

Raw pointers (*const T and *mut T) are neither Send nor Sync, since the compiler doesn’t know much about what they represent. The way to opt in to either of the traits is the same as with any other trait; use an impl block to implement the trait for your type. Note how implementing these traits requires the unsafe keyword, since the compiler cannot check for you if it’s correct. It’s a promise you make to the compiler, which it will just have to trust.

### Locking: Mutexes and RwLocks

The most commonly used tool for sharing (mutable) data between threads is a mutex, which is short for “mutual exclusion.” The job of a mutex is to ensure threads have exclusive access to some data by temporarily blocking other threads that try to access it at the same time.
Conceptually, a mutex has only two states: locked and unlocked. When a thread locks an unlocked mutex, the mutex is marked as locked and the thread can immediately continue. When a thread then attempts to lock an already locked mutex, that operation will block. The thread is put to sleep while it waits for the mutex to be unlocked. Unlocking is only possible on a locked mutex, and should be done by the same thread that locked it. If other threads are waiting to lock the mutex, unlocking will cause one of those threads to be woken up, so it can try to lock the mutex again and continue its course. Protecting data with a mutex is simply the agreement between all threads that they will only access the data when they have the mutex locked. That way, no two threads can ever access that data concurrently and cause a data race.
