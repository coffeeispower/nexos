use core::{cell::{Ref, RefCell, RefMut}, ops::Deref};

use alloc::{boxed::Box, vec::Vec};
use spin::Once;

use super::{current_core_id, number_of_cores};

pub struct CoreLocal<T> {
    inner: Once<Box<[RefCell<T>]>>,
    init: fn() -> T,
}

impl<T> CoreLocal<T> {
    pub const fn new(init: fn() -> T) -> Self {
        Self {
            inner: Once::new(),
            init
        }
    }
    fn ensure_initialized(&self) -> &[RefCell<T>] {
        self.inner.call_once(|| {
            let cores = number_of_cores();
            let mut cores_values = Vec::with_capacity(cores);
            for _ in 0..cores {
                cores_values.push((self.init)().into());
            }
            cores_values.into_boxed_slice()
        }).deref()
    }
    pub fn read(&self) -> Ref<'_, T> {
        self.ensure_initialized()[current_core_id()].borrow()
    }
    pub fn write(&self) -> RefMut<'_, T> {
        self.ensure_initialized()[current_core_id()].borrow_mut()
    }
}
unsafe impl<T> Sync for CoreLocal<T> {}
