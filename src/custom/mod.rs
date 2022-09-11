mod handel_errors;
use handel_errors::VekError;

use num_traits::{
    ops::checked::{CheckedAdd, CheckedMul, CheckedSub},
    Num,
};

use std::{
    alloc::{self, Layout},
    ptr::NonNull,
};

enum PushStatus {
    ZeroCapcity,
    NeedCapcity,
    OkCapacity,
}

pub struct Vek<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> Vek<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        handel_errors::unwraper(self.zero_type(), VekError::ZeroType("no zero type"));
        let status = self.push_status();
        match status {
            PushStatus::ZeroCapcity => {
                let ptr = self.allocating_new_memory(4);
                unsafe { ptr.as_ptr().write(item) };
                self.ptr = ptr;
                self.capacity = 4;
                self.len = 1;
            }

            PushStatus::NeedCapcity => {
                let ptr = self.reallocating_memory();
                unsafe { ptr.as_ptr().add(self.len).write(item) }
                self.len += 1;
                self.capacity *= 2;
                self.ptr = ptr;
            }

            PushStatus::OkCapacity => {
                handel_errors::unwraper(
                    self.check_memory_overflow(),
                    VekError::OverFlow("Can not write on memory cause overflow on memory!"),
                );
                unsafe { self.ptr.as_ptr().add(self.len).write(item) };
                self.len += 1;
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        Some(unsafe { &*self.ptr.as_ptr().add(index) })
    }

    pub fn get_mut<'a>(&self, index: usize) -> Option<&'a mut T> {
        if index >= self.len {
            return None;
        }

        Some(unsafe { &mut *self.ptr.as_ptr().add(index) })
    }

    fn get_ptr_slice(&self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }

        Some(unsafe { self.ptr.as_ptr().add(index).read() })
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        Some(unsafe { self.ptr.as_ptr().add(self.len).read() })
    }

    pub fn clear(&mut self) {
        unsafe {
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len));
        }
        self.dealloc_all();
        self.capacity = 0;
        self.len = 0;
    }

    pub fn is_empty(&self) -> bool {
        if self.len == 0 {
            return false;
        };

        true
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    fn push_status(&self) -> PushStatus {
        if self.capacity == 0 {
            PushStatus::ZeroCapcity
        } else if self.capacity <= self.len {
            PushStatus::NeedCapcity
        } else {
            PushStatus::OkCapacity
        }
    }

    fn zero_type(&self) -> Result<(), VekError> {
        if std::mem::size_of::<T>() == 0 {
            return Err(VekError::ZeroType(
                "can not allocating memory for zero type data",
            ));
        }

        Ok(())
    }

    fn allocating_new_memory(&self, size: usize) -> NonNull<T> {
        let layout = handel_errors::unwraper(
            Layout::array::<T>(size),
            VekError::Allocating("Error while building layout"),
        );

        let ptr = unsafe { alloc::alloc(layout) as *mut T };

        let ptr = NonNull::new(ptr);

        let ptr = match ptr {
            Some(p) => Ok(p),
            None => Err(VekError::Allocating("couldn't allowcation memory")),
        };

        handel_errors::unwraper(
            ptr,
            VekError::Allocating("Error While allocating memory 'NonNull::new()' faild"),
        )
    }

    fn reallocating_memory(&self) -> NonNull<T> {
        let align = handel_errors::unwraper(
            self.get_aling(),
            VekError::Allocating("error align size is invalid"),
        );

        let round_size = handel_errors::unwraper(
            self.round_to_nearest_multiple(align),
            VekError::OverFlow("round_to_nearest_multiple"),
        );

        let layout = Layout::from_size_align(round_size, align);
        let layout =
            handel_errors::unwraper(layout, VekError::Allocating("Error while building layout"));

        let new_capacity = handel_errors::unwraper(
            Self::safe_multiply(self.capacity, 2),
            VekError::OverFlow("multiply 'capacity * 2'"),
        );

        let new_size = handel_errors::unwraper(
            Self::safe_multiply(new_capacity, std::mem::size_of::<T>()),
            VekError::OverFlow("multiply 'capacity * size_of'"),
        );

        let ptr =
            unsafe { alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_size) as *mut T };

        let ptr = NonNull::new(ptr);

        let ptr = match ptr {
            Some(p) => Ok(p),
            None => Err(VekError::Allocating("couldn't reallocating memory")),
        };

        handel_errors::unwraper(
            ptr,
            VekError::Allocating("Error While reallocating memory 'NonNull::new()' faild"),
        )
    }

    fn get_aling(&self) -> Result<usize, VekError> {
        let align = std::mem::align_of::<T>();

        if align == 0 {
            return Err(VekError::Allocating("align size can not be zero"));
        }

        if align & (align - 1) != 0 {
            return Err(VekError::Allocating("align size must be power of the 2"));
        }
        Ok(align)
    }

    fn round_to_nearest_multiple(&self, multiply: usize) -> Result<usize, VekError> {
        let size = Self::safe_multiply(std::mem::size_of::<T>(), self.capacity);

        let size =
            handel_errors::unwraper(size, VekError::OverFlow("overflow 'size_of * capacity'"));

        if size < multiply {
            return Ok(multiply);
        }

        match size.checked_rem(multiply) {
            None => Ok(size),
            Some(rhs) => {
                let offset = handel_errors::unwraper(
                    Self::safe_sub(multiply, rhs),
                    VekError::OverFlow("overflow 'multiply - rhs' in round_size function"),
                );

                let new_size = handel_errors::unwraper(
                    Self::safe_add(size, offset),
                    VekError::OverFlow("overflow 'size + offset' in round_size function"),
                );

                Ok(new_size)
            }
        }
    }

    fn safe_multiply<U: Copy + Num + CheckedMul>(num1: U, num2: U) -> Result<U, VekError> {
        match num1.checked_mul(&num2) {
            None => Err(VekError::OverFlow("overflow when multiply two numbers")),
            Some(value) => Ok(value),
        }
    }

    fn safe_add<U: Copy + Num + CheckedAdd + PartialEq>(num1: U, num2: U) -> Result<U, VekError> {
        match num1.checked_add(&num2) {
            None => Err(VekError::OverFlow("overflow when add two numbers")),
            Some(value) => Ok(value),
        }
    }

    fn safe_sub<U: Copy + Num + CheckedSub + PartialEq>(num1: U, num2: U) -> Result<U, VekError> {
        match num1.checked_sub(&num2) {
            None => Err(VekError::OverFlow("overflow when sub two numbers")),
            Some(value) => Ok(value),
        }
    }

    fn check_memory_overflow(&self) -> Result<(), VekError> {
        let check_overflow = (self.len).checked_mul(std::mem::size_of::<T>());
        match check_overflow {
            Some(offset) => {
                if offset < isize::MAX as usize {
                    Ok(())
                } else {
                    Err(VekError::OverFlow(
                        "Can not write on memory cause overflow on memory",
                    ))
                }
            }
            None => Err(VekError::OverFlow("can not reach memory location")),
        }
    }

    fn dealloc_all(&mut self) {
        unsafe {
            let layout = alloc::Layout::from_size_align_unchecked(
                std::mem::size_of::<T>() * self.capacity,
                std::mem::align_of::<T>(),
            );

            alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout)
        }
    }
}

impl<T> Drop for Vek<T> {
    fn drop(&mut self) {
        if self.capacity == 0usize {
            return;
        }

        unsafe {
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len))
        };
        self.dealloc_all();
    }
}

impl<T> Default for Vek<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> IntoIterator for &'a Vek<T> {
    type Item = &'a T;
    type IntoIter = VekIteratorRef<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        VekIteratorRef {
            vek: self,
            index: 0,
        }
    }
}

impl<'a, T> IntoIterator for &'a mut Vek<T> {
    type Item = &'a mut T;
    type IntoIter = VekIteratorMutRef<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        VekIteratorMutRef {
            vek: self,
            index: 0,
        }
    }
}

impl<T> IntoIterator for Vek<T> {
    type Item = T;
    type IntoIter = VekIterator<T>;
    fn into_iter(self) -> VekIterator<T> {
        VekIterator {
            vek: self,
            index: 0,
        }
    }
}

pub struct VekIteratorRef<'a, T> {
    vek: &'a Vek<T>,
    index: usize,
}

pub struct VekIteratorMutRef<'a, T> {
    vek: &'a mut Vek<T>,
    index: usize,
}

pub struct VekIterator<T> {
    vek: Vek<T>,
    index: usize,
}

impl<'a, T> Iterator for VekIteratorRef<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let current_index = self.index;
        let current_item = self.vek.get(current_index);
        self.index += 1;
        current_item
    }
}

impl<'a, T> Iterator for VekIteratorMutRef<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        let current_index = self.index;
        let current_item = self.vek.get_mut(current_index);
        self.index += 1;
        current_item
    }
}

impl<T> Iterator for VekIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let current_index = self.index;
        let current_item = self.vek.get_ptr_slice(current_index);
        self.index += 1;
        current_item
    }
}
