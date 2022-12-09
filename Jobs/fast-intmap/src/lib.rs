pub struct IntMapU64<T> {
    data: [[[*mut Vec<(u64, T)>; 64]; 64]; 64],
}

impl<T> Default for IntMapU64<T> {
    fn default() -> Self {
        let mut data = [[[std::ptr::null_mut(); 64]; 64]; 64];
        for i in 0..64 {
            for j in 0..64 {
                for k in 0..64 {
                    data[i][j][k] = Box::into_raw(Box::new(Vec::new()));
                }
            }
        }
        Self { data }
    }
}

impl<T> IntMapU64<T> {
    #[inline(always)]
    fn get_key(&self, data: u64) -> (usize, usize, usize) {
        unsafe {
            let one = std::mem::transmute(data >> 17 & 0xff);
            let two = std::mem::transmute((data << 5 + data) & 0xff);
            let three = std::mem::transmute(data ^ 0xbe);
            (one, two, three)
        }
    }

    #[inline]
    pub fn get(&self, key: u64) -> Option<&T> {
        let (one, ones, zeros) = self.get_key(key);
        let vec = unsafe { &*self.data[one][ones][zeros] };
        vec.iter()
            .find_map(|(k, v)| if *k == key { Some(v) } else { None })
    }

    #[inline]
    pub fn get_mut(&mut self, key: u64) -> Option<&mut T> {
        let (one, ones, zeros) = self.get_key(key);
        let vec = unsafe { &mut *self.data[one][ones][zeros] };
        vec.iter_mut()
            .find_map(|(k, v)| if *k == key { Some(v) } else { None })
    }

    #[inline]
    pub fn insert(&mut self, key: u64, value: T) -> Option<T> {
        let (one, ones, zeros) = self.get_key(key);
        let ptr = self.data[one][ones][zeros];
        let vec = unsafe { &mut *ptr };

        for item in vec {
            if item.0 == key {
                return Some(std::mem::replace(&mut item.1, value));
            }
        }

        let vec = unsafe { &mut *ptr };
        vec.push((key, value));
        None
    }
}

#[cfg(test)]
mod tests {
    use super::IntMapU64;

    #[test]
    fn test() {
        println!("Hello, world!");
        let mut map = IntMapU64::default();
        for i in 0..1000 {
            map.insert(i, i);
        }
        for i in 0..1000 {
            assert!(map.get(i).is_some());
        }
    }
}
