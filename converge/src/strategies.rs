/*! A collection of common strategies that might be of use for field types.
*/
/** Strategies for the Vec type.
*/
pub mod vec {
    /**
    concatenate lhs and rhs
    */
    pub fn concatenate<T>(mut lhs: Vec<T>, mut rhs: Vec<T>) -> Vec<T> {
        lhs.extend(rhs);
        lhs
    }

    /**
    Use the lhs, unless it is empty, then use the rhs.
     */
    pub fn replace_empty<T>(lhs: Vec<T>, rhs: Vec<T>) -> Vec<T> {
        match lhs.len() == 0 {
            false => lhs,
            true => rhs,
        }
    }
    fn converge_engine<T>(mut lhs: Vec<T>, mut rhs: Vec<T>, all_rhs: bool) -> Vec<T>
    where
        T: crate::Converge,
    {
        let require_capacity = std::cmp::max(lhs.len(), rhs.len());
        let mut output = Vec::with_capacity(require_capacity);
        let mut it_lhs = lhs.drain(..);
        let mut it_rhs = rhs.drain(..);
        loop {
            let value = match (it_lhs.next(), it_rhs.next()) {
                (Some(x), Some(y)) => x.converge(y),
                (Some(x), None) => x,
                (None, Some(y)) => {
                    if !all_rhs {
                        break;
                    }
                    y
                }
                (None, None) => break,
            };
            output.push(value);
        }
        output
    }

    /**
    Recursively converge each item on list. if lhs is longer take all remaining
    items. If rhs list is longer ignore the extra list items.
    */
    pub fn converge_on_intersection<T>(lhs: Vec<T>, rhs: Vec<T>) -> Vec<T>
    where
        T: crate::Converge,
    {
        converge_engine(lhs, rhs, false)
    }

    /**
    Recursively converge each item on list. If both lists are different lengths
    take all remaining items from the longer list.
    */
    pub fn converge_union<T>(lhs: Vec<T>, rhs: Vec<T>) -> Vec<T>
    where
        T: crate::Converge,
    {
        converge_engine(lhs, rhs, true)
    }
}
/** Strategies for the HashMap type.
*/
pub mod hashmap {
    use std::collections::HashMap;
    use std::hash::Hash;

    fn converge_engine<K, V>(
        mut lhs: HashMap<K, V>,
        mut rhs: HashMap<K, V>,
        all_rhs: bool,
    ) -> HashMap<K, V>
    where
        K: Eq + Hash,
        V: crate::Converge,
    {
        let require_capacity = std::cmp::max(lhs.len(), rhs.len());
        let mut output = HashMap::with_capacity(require_capacity);

        for (key, lhs_val) in lhs.drain() {
            match rhs.remove(&key) {
                Some(rhs_val) => output.insert(key, lhs_val.converge(rhs_val)),
                None => output.insert(key, lhs_val),
            };
        }
        if all_rhs {
            for (key, rhs_val) in rhs.drain() {
                output.insert(key, rhs_val);
            }
        }
        output
    }
    /**
    Recursively converge intersection of keys. Output only lhs of key values.
    */
    pub fn converge_on_intersection<K, V>(lhs: HashMap<K, V>, rhs: HashMap<K, V>) -> HashMap<K, V>
    where
        K: Eq + Hash,
        V: crate::Converge,
    {
        converge_engine(lhs, rhs, false)
    }

    /**
    Recursively converge intersection of keys. Output union of key values.
    */
    pub fn converge_union<K, V>(lhs: HashMap<K, V>, rhs: HashMap<K, V>) -> HashMap<K, V>
    where
        K: Eq + Hash,
        V: crate::Converge,
    {
        converge_engine(lhs, rhs, true)
    }
}
