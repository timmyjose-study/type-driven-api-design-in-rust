use std::cell::UnsafeCell;

struct SystemMutex;

impl SystemMutex {
    pub fn lock(&self) {}
    pub fn unlock(&self) {}
}

struct MutexGuard<'a, T> {
    lock: &'a Mutex<T>,
}

impl<'a, T> MutexGUard<'a, T> {
    pub fn new(lock: &'a Mutex<T>) -> Self {
        lock.system_mutex.lock();
        MutexGuard { lock }
    }

    pub fn get(&self) -> &mut T {
        &mut *self.lock.t
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.system_mutex.unlock();
    }
}

struct Mutex<T> {
    t: UnsafeCell<T>,
    system_mutex: SystemMutex,
}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Mutex {
            t: t.into(),
            system_mutex: SystemMutex,
        }
    }

    pub fn lock<'a>(&self) -> &'a MutexGuard<'a, T> {
        MutexGuard::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guard() {
        // both state and lifetime issue now solved
        let mu = Mutex::new(1);
        {
            let guard = mu.lock();
            let n = guard.get();
            *n = 1;
        }
        {
            let guard = mu.lock();
            let m = guard.get();
            *m = 2;
        }
    }
}
