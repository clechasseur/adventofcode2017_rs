use std::collections::HashMap;
use std::hash::Hash;

pub struct Loop {
    pub start: usize,
    pub end_exclusive: usize,
}

impl Loop {
    pub fn new(start: usize, end_exclusive: usize) -> Self {
        Self { start, end_exclusive }
    }

    pub fn find<I>(seq: I) -> Option<Self>
    where
        I: Iterator,
        <I as Iterator>::Item: Eq + Hash,
    {
        let mut seen = HashMap::new();
        
        for (i, v) in seq.enumerate() {
            match seen.get(&v) {
                Some(&start) => return Some(Self::new(start, i)),
                None => {
                    seen.insert(v, i);
                },
            }
        }
        
        None
    }
}
