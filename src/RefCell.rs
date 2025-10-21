use std::{cell::UnsafeCell, ops::{Deref, DerefMut}};
use crate::Cell;

#[derive(Copy, Clone)]
enum RefType {
    Unshared,
    Shared(usize),
    Exclusive,
}

pub struct RefCell<T>{
value: UnsafeCell<T>,
reference: Cell<RefType>
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        RefCell {
            value: UnsafeCell::new(value),
            reference: Cell::new(RefType::Unshared)
        }
    }

     pub fn borrow(&self) -> Option<Ref<'_,T>> {
       match self.reference.get() {
         RefType::Unshared => {
            self.reference.set(RefType::Shared(1));
            Some(Ref { refCell: self })
         }, 
         RefType::Shared(n) => {
            self.reference.set(RefType::Shared(n+1));
            Some(Ref { refCell: self })
         },
         RefType::Exclusive => {
            None
         }
       }
     }

     pub fn borrow_mut(&self) -> Option<RefMut<'_,T>> {
        if let RefType::Unshared = self.reference.get() {
            self.reference.set(RefType::Exclusive);
            Some(RefMut { refCell: self })
        } else {
            None
        }
     }
     
}

pub struct Ref<'refCell,T> {
    refCell: &'refCell RefCell<T>,
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refCell.reference.get() {
            RefType::Unshared | RefType::Exclusive => unreachable!(),
            RefType::Shared(1) => {
                self.refCell.reference.set(RefType::Unshared);
            },
            RefType::Shared(n) => {
                self.refCell.reference.set(RefType::Shared(n-1));
            }
        }
    }
}

impl<T> std::ops::Deref for Ref<'_,T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.refCell.value.get()
        }
    }
}

pub struct RefMut<'refCell, T> {
    refCell: &'refCell RefCell<T>,
}

impl<T> Drop for RefMut<'_,T> {
    fn drop(&mut self) {
        match self.refCell.reference.get() {
            RefType::Unshared | RefType::Shared(_) => unreachable!(),
            RefType::Exclusive => {
                self.refCell.reference.set(RefType::Unshared);
            }
        }
    }
}

impl <T> Deref for RefMut<'_,T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.refCell.value.get()
        }
    }
}

impl <T> DerefMut for RefMut<'_, T> {
// type Target = T;
fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe {
        &mut *self.refCell.value.get()
}
}
}

// impl<'RefCell> 
    
// }
