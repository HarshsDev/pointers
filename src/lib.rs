mod RefCell;
mod rc;
use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>
}

impl<T> Cell<T> {
 pub fn new(value: T) -> Self {
    Cell{
        value: UnsafeCell::new(value)
    }
 } 

 pub fn set(&self, value: T)  {
    // SAFETY: we know no one else is modifying the cells value concurrently
    // SAFETY: cause koi bhi refernce ko invalide nhi kr rahe cause hum kisko woh grant  nhi kar rahe using only copy
    unsafe {
        *self.value.get() = value;
    }
 }

 pub fn get(&self) -> T 
 where T:Copy {
    unsafe {
        *self.value.get()
    }
 }

}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
