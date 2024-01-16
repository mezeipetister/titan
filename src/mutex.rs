use std::cell::UnsafeCell;
use std::hint::spin_loop;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};
struct Mutex<T> {
    is_acquired: AtomicBool,
    data: UnsafeCell<T>,
}
impl<T> Mutex<T> {
    fn new(data: T) -> Mutex<T> {
        Mutex {
            is_acquired: AtomicBool::default(),
            data: UnsafeCell::new(data),
        }
    }
    fn acquire(&self) -> MutexGuard<'_, T> {
        while !self.is_acquired.swap(true, Ordering::AcqRel) {
            spin_loop() // Now we signals the processor that it is inside a busy-wait spin-loop
        }
        MutexGuard { mutex: &self }
    }
    fn release(&self) {
        self.is_acquired.store(false, Ordering::Release);
    }
}
struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}
impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}
impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}
impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.release()
    }
}
// We need to force Send and Sync traits because our mutex has
// UnsafeCell, which don't realize it
// As long as T: Send, it's fine to send and share Mutex<T> between threads.
unsafe impl<T> Send for Mutex<T> where T: Send {}
unsafe impl<T> Sync for Mutex<T> where T: Send {}
unsafe impl<T> Send for MutexGuard<'_, T> where T: Send {}
unsafe impl<T> Sync for MutexGuard<'_, T> where T: Send + Sync {}
