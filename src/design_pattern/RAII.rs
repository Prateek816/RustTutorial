use std::sync::Mutex;

fn main() {
    let chest = Mutex::new(0);

    {
        // `lock()` attempts to acquire the mutex.
        // It returns a `LockResult<MutexGuard<T>>`.
        let key = chest.lock().unwrap();

        // `key` is a MutexGuard.
        // It gives us mutable access to the value protected by the Mutex.
        let mut data = key;

        *data += 1;

        // IMPORTANT: RAII
        // When `data` goes out of scope at the end of this block,
        // its destructor (`Drop`) is automatically called.
        //
        // The MutexGuard's Drop implementation automatically
        // unlocks the Mutex.
        //
        // We don't have to manually write something like:
        // chest.unlock();
    }

    println!("Value: {}", *chest.lock().unwrap());
}