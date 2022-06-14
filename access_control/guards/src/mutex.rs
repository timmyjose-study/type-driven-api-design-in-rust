use std::cell::UnsafeCell;

struct SystemMutex;

impl SystemMutex {
    pub fn lock(&self) {}

    pub fn unlock(&self) {}
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

    pub fn lock(&self) -> &mut T {
        self.system_mutex.lock();
        unsafe { &mut *self.t.get() }
    }

    pub fn unlock(&self) {
        self.system_mutex.unlock();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutex() {
        let mu = Mutex::new(1);
        let n = mu.lock();
        mu.unlock();
        mu.unlock(); // state problem

        let m = mu.lock();
        *n = 1; // lifetime problem
        *m = 2; // lifetime problem
    }
}
