use std::{collections::HashMap, hash::Hash};

pub fn unique_element_counts<T>(text: impl Iterator<Item = T>) -> HashMap<T, u32>
where
    T: Eq,
    T: Hash,
{
    let mut counts = HashMap::new();
    for char in text {
        let entry = counts.entry(char).or_insert(0);
        *entry += 1;
    }
    counts
}
