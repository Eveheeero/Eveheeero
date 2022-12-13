#![allow(dead_code)]

use std::cell::RefCell;

pub struct Map<T> {
    data: RefCell<Box<[Option<T>]>>,
    keys: RefCell<Box<[Option<usize>]>>,
    mask: usize,
    size: usize,
}

impl<T> Map<T> {
    pub fn new(predict: usize) -> Self {
        let length = predict.next_power_of_two();
        let mask = length - 1;
        let data = std::iter::repeat_with(|| None)
            .take(length)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let keys = std::iter::repeat_with(|| None)
            .take(length)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Self {
            data: data.into(),
            keys: keys.into(),
            size: 0,
            mask,
        }
    }

    #[inline]
    fn resize(&mut self, size: usize) {
        self.mask = size - 1;

        let data = std::iter::repeat_with(|| None)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let keys = std::iter::repeat_with(|| None)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let data_origin = std::mem::replace(&mut self.data, data.into());
        let data_origin = data_origin.into_inner().into_vec();
        let keys_origin = std::mem::replace(&mut self.keys, keys.into());
        let keys_origin = keys_origin.into_inner().into_vec();

        for (key, value) in keys_origin.into_iter().zip(data_origin.into_iter()) {
            if let Some(key) = key {
                let mut slot = key & self.mask;
                while self.keys.borrow()[slot].is_some() {
                    slot = (slot + 1) & self.mask;
                }
                self.keys.borrow_mut()[slot] = Some(key);
                self.data.borrow_mut()[slot] = value;
            }
        }
    }

    #[inline]
    pub fn get(&self, key: &usize) -> Option<&T> {
        let mut slot = key & self.mask;
        let keys = &self.keys;

        while keys.borrow()[slot].is_some() && keys.borrow()[slot] != Some(*key) {
            slot = (slot + 1) & self.mask;
        }
        if keys.borrow()[slot].is_none() {
            None
        } else {
            unsafe { Some(&(*self.data.as_ptr())[slot].as_ref().unwrap()) }
        }
    }

    #[inline]
    pub fn insert(&mut self, key: usize, value: T) -> Option<T> {
        let mut slot = key & self.mask;
        let keys = &self.keys;

        while keys.borrow()[slot].is_some() {
            match keys.borrow()[slot] {
                Some(k) if k == key => {
                    let old = std::mem::replace(&mut self.data.borrow_mut()[slot], Some(value));
                    return old;
                }
                _ => {
                    slot = (slot + 1) & self.mask;
                }
            }
        }

        if keys.borrow()[slot].is_none() {
            self.size += 1;
        }
        self.keys.borrow_mut()[slot] = Some(key);
        self.data.borrow_mut()[slot] = Some(value);
        if self.size.wrapping_shl(1) >= self.mask {
            self.resize((self.mask + 2).next_power_of_two());
        }
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn maptest() {
        let mut map = super::Map::new(4);
        for i in 0..100 {
            map.insert(i, i);
        }
        for i in 0..100 {
            assert_eq!(map.get(&i), Some(&i));
        }
    }
}
