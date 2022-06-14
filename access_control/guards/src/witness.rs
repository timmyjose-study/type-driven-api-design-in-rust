use std::cell::UnsafeCell;

struct MutexIsLocked;

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

    pub fn lock(&self) -> MutexIsLocked {
        self.system_mutex.lock();
        MutexIsLocked
    }

    pub fn unlock(&self, _witness: MutexIsLocked) {
        self.system_mutex.unlock();
    }

    pub fn get(&self, _witness: &MutexIsLocked) -> &mut T {
        unsafe { &mut *self.t.get() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_witness() {
        let mu = Mutex::new(1);
        let witness = mu.lock();
        let n = mu.get(&witness);
        mu.unlock(witness);
        // not possible now
        //mu.unlock(witness);

        let witness2 = mu.lock();
        let m = mu.get(&witness2);
        mu.unlock(witness2);

        *n = 100; // lifetime problem
        *m = 200; // lifetime problem
    }
}
