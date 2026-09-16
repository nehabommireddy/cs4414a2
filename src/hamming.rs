//! Tune word comparisons via Hamming distance

/// Step 0: Baseline computation --- nothing clever here
pub mod basic_str {

    /// Naive computation of Hamming distance between two strings
    pub fn dist(s1: &str, s2: &str) -> isize {
        s1.chars()
            .zip(s2.chars())
            .map(|(c1, c2)| if c1 == c2 { 0 } else { 1 })
            .sum::<isize>()
            + ((s1.len() as isize) - (s2.len() as isize)).abs()
    }

    /// Compute vector of mean distances for all strings
    pub fn mean_dists(dict: &[String]) -> Vec<f64> {
        let mut dists: Vec<isize> = vec![0; dict.len()];
        for (i, w1) in dict.iter().enumerate() {
            for w2 in dict.iter() {
                dists[i] += dist(w1, w2);
            }
        }
        dists
            .iter()
            .map(|d| (*d as f64) / (dict.len() as f64))
            .collect()
    }

    /// Standardized interface
    pub fn mean_dists_dict(dict: &[String]) -> Vec<f64> {
        mean_dists(dict)
    }
}

/// Step 1: Blocked computation
pub mod block_str {

    use super::basic_str::dist; // We can use the naive string compare
    const BSIZE: usize = 500; // Probably want a constant block size param

    /// Compute vector of mean distances for all strings (you may change
    /// the interface if you want)
    fn block_updates(d1: &[String], d2: &[String], counts: &mut [isize]) {
        todo!()
    }

    /// Blocked computation
    pub fn mean_dists(dict: &[String]) -> Vec<f64> {
        todo!();
    }

    pub fn mean_dists_dict(dict: &[String]) -> Vec<f64> {
        mean_dists(dict)
    }
}

/// Step 2: Removing indirection
pub mod basic_word {

    const WSIZE: usize = 28; // You may want to fiddle with this

    /// Word storage
    pub struct Word([u8; WSIZE]);

    impl Word {
        /// Create a Word from a string
        pub fn new(s: &str) -> Self {
            todo!()
        }
    }

    /// Re-pack the dictionary in more condensed form
    fn pack_dict(dict: &[String]) -> Vec<Word> {
        dict.iter().map(|s| Word::new(s)).collect()
    }

    /// Compute the Hamming distance between two Words
    fn dist(w1: &Word, w2: &Word) -> isize {
        // You may change the output type
        todo!()
    }

    /// Compute vector of mean distances for all words (pre-packed)
    pub fn mean_dists(dict: &[Word]) -> Vec<f64> {
        todo!()
    }

    /// Compute vector of mean distances for all words    
    pub fn mean_dists_dict(dict: &[String]) -> Vec<f64> {
        let pdict = pack_dict(dict);
        mean_dists(&pdict)
    }
}

/// Step 3: SIMD Within a Register
pub mod swar_word {

    /// Word storage
    pub struct Word([u32; 6]); // You may change the internals (eg [u64; 3])

    impl Word {
        /// Create packed word
        pub fn new(s: &str) -> Self {
            todo!()
        }
    }

    /// Compute the Hamming distance between two Words
    fn dist(w1: &Word, w2: &Word) -> isize {
        // You may change the signature
        todo!()
    }

    /// Compute vector of mean distances for all words (pre-packed)
    pub fn mean_dists(dict: &[Word]) -> Vec<f64> {
        todo!()
    }

    /// Re-pack the dictionary in more condensed form
    fn pack_dict(dict: &[String]) -> Vec<Word> {
        dict.iter().map(|s| Word::new(s)).collect()
    }

    pub fn mean_dists_dict(dict: &[String]) -> Vec<f64> {
        let pdict = pack_dict(dict);
        mean_dists(&pdict)
    }
}

/// Step 4: Optimized version!
pub mod optimized {

    pub fn mean_dists_dict(dict: &[String]) -> Vec<f64> {
        todo!()
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_naive_dist() {
        use super::basic_str::dist;
        assert_eq!(dist("aa", "aaaaa"), 3);
        assert_eq!(dist("aaaaa", "aa"), 3);
        assert_eq!(dist("test", "tilt"), 2);
    }

    // TODO: Add your own module tests!
}
