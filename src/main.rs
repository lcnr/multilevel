use std::collections::hash_map;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(PartialEq, Eq, Hash)]
struct Animal(&'static str);
#[derive(PartialEq, Eq, Hash)]
struct Fruit(&'static str);

trait DeepEntry<'a, I>: Sized {
    type Entry;

    fn deep_entry(&'a mut self, key: I) -> Self::Entry;
}

impl<'a, K: 'a + Eq + Hash, V: 'a> DeepEntry<'a, K> for HashMap<K, V> {
    type Entry = hash_map::Entry<'a, K, V>;

    fn deep_entry(&'a mut self, key: K) -> Self::Entry {
        self.entry(key)
    }
}

impl<'a, K: Eq + Hash, V, I> DeepEntry<'a, (K, I)> for HashMap<K, V>
where
    V: DeepEntry<'a, I> + Default,
{
    type Entry = V::Entry;
    fn deep_entry(&'a mut self, (key, rest): (K, I)) -> Self::Entry {
        self.entry(key)
            .or_insert_with(Default::default)
            .deep_entry(rest)
    }
}

struct VecEntry<'a, T> {
    v: &'a mut Vec<T>,
    key: usize,
}

impl<'a, T> VecEntry<'a, T> {
    fn or_insert_with<F: FnMut() -> T>(self, default: F) -> &'a mut T {
        if self.v.len() <= self.key {
            self.v.resize_with(self.key + 1, default);
        }
        &mut self.v[self.key]
    }
}

impl<'a, T: 'a> DeepEntry<'a, usize> for Vec<T> {
    type Entry = VecEntry<'a, T>;
    fn deep_entry(&'a mut self, key: usize) -> Self::Entry {
        VecEntry { v: self, key }
    }
}

impl<'a, T, I> DeepEntry<'a, (usize, I)> for Vec<T>
where
    T: DeepEntry<'a, I> + Default,
{
    type Entry = T::Entry;
    fn deep_entry(&'a mut self, (key, rest): (usize, I)) -> Self::Entry {
        VecEntry { v: self, key }
            .or_insert_with(Default::default)
            .deep_entry(rest)
    }
}

fn main() {
    let mut data: HashMap<Animal, Vec<HashMap<Fruit, usize>>> = HashMap::new();

    *data
        .deep_entry((Animal("pig"), (6, Fruit("apply"))))
        .or_default() += 1;

    assert_eq!(
        *data
            .deep_entry((Animal("pig"), (6, Fruit("apply"))))
            .or_insert(0),
        1
    );
}
