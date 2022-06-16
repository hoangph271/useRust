struct SuckassMap<K: std::cmp::PartialEq, V> {
    entries: Vec<(K, V)>,
}

impl<K: std::cmp::PartialEq, V> SuckassMap<K, V> {
    fn new() -> SuckassMap<K, V> {
        SuckassMap {
            entries: Vec::new(),
        }
    }

    fn insert(&mut self, key: K, value: V) {
        self.entries.push((key, value));
    }

    fn set(&mut self, key: K, value: V) {
        for (k, v) in self.entries.iter_mut() {
            if *k == key {
                *v = value;
                return;
            }
        }

        self.insert(key, value);
    }

    fn get (&mut self, key: K) -> Option<&V> {
        for (k, v) in self.entries.iter() {
            if *k == key {
                return Some(v);
            }
        }

        None
    }
}

struct SuckassMapIter<K: std::cmp::PartialEq, V> {
    suckass_map: SuckassMap<K, V>,
}

impl<K: std::cmp::PartialEq, V> Iterator for SuckassMapIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<(K, V)> {
        if self.suckass_map.entries.len() != 0 {
            let (key, value) = self.suckass_map.entries.remove(0);

            Some((key, value))
        } else {
            None
        }
    }
}

impl<K: std::cmp::PartialEq, V> IntoIterator for SuckassMap<K, V> {
    type Item = (K, V);
    type IntoIter = SuckassMapIter<K, V>;

    fn into_iter(self) -> SuckassMapIter<K, V> {
        SuckassMapIter { suckass_map: self }
    }
}

fn main() {
    let mut map: SuckassMap<String, usize> = SuckassMap::new();
    map.insert("foo".to_owned(), 1);
    map.insert("bar".to_owned(), 2);
    map.insert("baz".to_owned(), 3);

    map.set("foo".to_owned(), 0);

    println!("{:?}", map.get("foo".to_owned()));
    println!("{:?}", map.get("NONE".to_owned()));

    for entry in map {
        println!("{:?}", entry);
    }
}
