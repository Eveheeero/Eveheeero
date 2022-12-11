use parking_lot::RwLock;

#[derive(Default, Debug)]
pub struct IntMapU64<T> {
    data: Box<Inner<T>>,
    depth: usize,
    key_box: RwLock<Vec<usize>>,
    size: usize,
    capacity: usize,
}

#[derive(Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Inner<T> {
    #[default]
    Empty,
    Level([Box<Inner<T>>; 16]),
    Data(Vec<(u64, T)>),
}

impl<T> IntMapU64<T> {
    #[inline]
    fn get_key(&self, data: u64) {
        let mut key_box = self.key_box.write();
        let data = data >> (64 - self.depth);
        key_box.resize(self.depth, Default::default());
        for i in 0..self.depth {
            key_box[i] = Self::cast(
                11400714819323198549u64
                    .wrapping_shr(Self::cast(i * 5))
                    .wrapping_mul(data),
            );
        }
    }

    #[inline(always)]
    fn cast<Z, X>(x: Z) -> X
    where
        Z: Sized,
        X: Sized,
    {
        unsafe { std::mem::transmute_copy::<Z, X>(&x) }
    }

    #[inline]
    pub fn get(&self, key: &u64) -> Option<&T> {
        let key = *key;
        self.get_key(key);

        let mut inner: &Box<Inner<T>> = &self.data;
        for key in self.key_box.read().iter() {
            inner = match inner.as_ref() {
                Inner::Level(level) => &level[*key],
                _ => unreachable!(),
            }
        }

        let vec = match inner.as_ref() {
            Inner::Data(data) => data,
            _ => unreachable!(),
        };
        vec.iter()
            .find_map(|(k, v)| if *k == key { Some(v) } else { None })
    }

    #[inline]
    pub fn insert(&mut self, key_o: u64, value: T) -> Option<T> {
        if self.size >= self.capacity {
            self.capacity = self.size.next_power_of_two();
            self.realloc();
        }
        self.get_key(key_o);

        let mut inner: &mut Box<Inner<T>> = &mut self.data;
        for key in self.key_box.read().iter() {
            if let Inner::Empty = inner.as_ref() {
                *inner = Box::new(Inner::Level([
                    Box::new(Inner::<T>::Empty),
                    Box::new(Inner::Empty),
                    Box::new(Inner::Empty),
                    Box::new(Inner::Empty),
                    Box::new(Inner::Empty),
                    Box::new(Inner::Empty),
                    Box::new(Inner::Empty),
                    Box::new(Inner::Empty),
                    Box::new(Inner::Empty),
                    Box::new(Inner::Empty),
                    Box::new(Inner::Empty),
                    Box::new(Inner::Empty),
                    Box::new(Inner::Empty),
                    Box::new(Inner::Empty),
                    Box::new(Inner::Empty),
                    Box::new(Inner::Empty),
                ]));
            };
            inner = if let Inner::Level(level) = inner.as_mut() {
                &mut level[*key]
            } else {
                unreachable!()
            };
        }

        let vec = match inner.as_mut() {
            Inner::Data(data) => data,
            Inner::Empty => {
                *inner = Box::new(Inner::Data(vec![(key_o, value)]));
                return None;
            }
            _ => unreachable!(),
        };

        if let Some((_, v)) = vec.iter_mut().find(|(k, _)| *k == key_o) {
            return Some(std::mem::replace(v, value));
        }

        vec.push((key_o, value));
        return None;
    }

    fn realloc(&mut self) {
        // depth 설정
        // 3depth에 있는 데이터에 대해 tree로 변경하고 4depth에 있는 데이터로 설정
        self.depth = self.capacity.next_power_of_two().trailing_zeros() as usize / 4;
        todo!()
    }
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
            assert!(map.get(&i).is_some());
        }
    }
}
