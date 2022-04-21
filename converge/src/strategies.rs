pub mod vec {
    pub fn replace_empty(a: Vec<i16>, b: Vec<i16>) -> Vec<i16> {
        match a.len() == 0 {
            false => a,
            true => b,
        }
    }
}
