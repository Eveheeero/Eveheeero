use std::ops::BitAnd;

pub struct IntMapU64<T> {
    data: Vec<Vec<Vec<Vec<(u64, T)>>>>,
}

impl<T> Default for IntMapU64<T> {
    fn default() -> Self {
        // iter::repeat를 사용하여 객체생성
        let mut data = vec![];
        for _ in 0..256 {
            let mut data2 = vec![];
            for _ in 0..256 {
                let mut data3 = vec![];
                for _ in 0..256 {
                    data3.push(Vec::new());
                }
                data2.push(data3);
            }
            data.push(data2);
        }

        Self { data }
    }
}

impl<T> IntMapU64<T> {
    #[inline]
    fn get_key(&self, data: u64) -> (usize, usize, usize) {
        let one = data.rotate_right(17);
        let two = data.rotate_left(5) + data;
        let three = data ^ 0xbe;
        (cast(one), cast(two), cast(three))
    }

    #[inline]
    pub fn get(&self, key: u64) -> Option<&T> {
        let (one, two, three) = self.get_key(key);
        let vec = &*self.data[one][two][three];
        vec.iter()
            .find_map(|(k, v)| if *k == key { Some(v) } else { None })
    }

    #[inline]
    pub fn get_mut(&mut self, key: u64) -> Option<&mut T> {
        let (one, two, three) = self.get_key(key);
        let vec = &mut *self.data[one][two][three];
        vec.iter_mut()
            .find_map(|(k, v)| if *k == key { Some(v) } else { None })
    }

    #[inline]
    pub fn insert(&mut self, key: u64, value: T) -> Option<T> {
        let (one, two, three) = self.get_key(key);
        let ptr = &mut self.data[one][two][three];
        let vec = &mut *ptr;

        for item in vec {
            if item.0 == key {
                return Some(std::mem::replace(&mut item.1, value));
            }
        }

        let vec = &mut *ptr;
        vec.push((key, value));
        None
    }
}

#[inline(always)]
fn cast<T, Y>(x: T) -> Y
where
    T: Sized,
    Y: Sized + BitAnd<Output = Y> + From<u8>,
{
    unsafe { std::mem::transmute_copy::<T, Y>(&x) & Y::from(0xff) }
}

#[cfg(test)]
mod tests {
    use super::IntMapU64;

    #[test]
    fn test() {
        let mut map: IntMapU64<u64> = IntMapU64::default();
        for i in 0..1000 {
            map.insert(i, i);
        }
        for i in 0..1000 {
            assert!(map.get(i).is_some());
        }
    }
}
