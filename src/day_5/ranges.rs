use std::cmp::max;
use tracing::{debug, instrument};

/// Describes a defragmented union of ranges.
pub struct Ranges {
    ranges: Vec<(usize, usize)>,
}

impl Ranges {
    pub fn new(mut ranges: Vec<(usize, usize)>) -> Self {
        ranges.sort_by_key(|r| r.0);
        let mut ranges = Ranges { ranges };
        ranges.defrag();
        ranges
    }
    /// Defragment the ranges, compacting intersections as needed.
    /// We can imagine four possible scenarios when dealing with unifying two ranges:
    /// 1. the two ranges have an empty intersection;
    /// 2. one range is fully contained within the other;
    /// 3. one range partially covers the first half of the other range;
    /// 4. one range partially covers the latter half of the other range.
    ///
    /// The algorithm works by identifying which of the four scenarios we're in.
    /// To do so, we first sort the ranges by their starting point. This is an invariant implicit to
    /// this struct, which must be still true after the defragmentation happens.
    ///
    /// Then, we can apply a reduction step:
    /// 1. we take a pair of ranges to analyze. This step can result in either both ranges being kept, or only the merge being kept.
    /// 2. we compare the start of the second range with the end of the first. If the former is greater, the intersection between the two ranges is empty and they both must be kept.
    /// 3. otherwise, the second range overlaps with the first, and we must merge them.
    ///     a. the starting point of the merged range must be the one of the first, as it's the minimum of the two by construction.
    ///     b. the ending point will be the maximum between the two ranges.
    /// 4. we proceed analyzing the next pair, for which the first range will either be the newly merged range, or the original second range we were trying to merge.
    #[instrument(skip(self))]
    fn defrag(&mut self) {
        self.ranges = self.ranges.iter().fold(
            Vec::with_capacity(self.ranges.len()),
            |mut acc, el| match acc.last_mut() {
                Some((_, end)) => {
                    // +1 to consider inclusiveness of ranges
                    if el.0 > *end + 1 {
                        debug!(?el, "disjointed");
                        acc.push(*el);
                        acc
                    } else {
                        debug!(?end, ?el, "merge");
                        *end = max(*end, el.1);
                        acc
                    }
                }
                None => {
                    debug!(?el, "empty acc");
                    acc.push(*el);
                    acc
                }
            },
        );
    }

    pub fn contains(&self, x: usize) -> bool {
        self.ranges.iter().any(|r| r.0 <= x && x <= r.1)
    }

    pub fn count(&self) -> usize {
        self.ranges.iter().fold(0, |acc, x| acc + x.1 - x.0 + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_defrag_single_long_range() {
        let mut ranges = Ranges::new(vec![(1, 3), (4, 5), (6, 7), (8, 10), (11, 12)]);
        assert_eq!(ranges.ranges, vec![(1, 12)]);
    }

    #[test]
    fn test_defrag_fully_contained() {
        let mut ranges = Ranges::new(vec![(1, 12), (2, 4)]);
        assert_eq!(ranges.ranges, vec![(1, 12)]);
    }

    #[test]
    fn test_defrag_partially_contained() {
        let mut ranges = Ranges::new(vec![(1, 5), (3, 7)]);
        assert_eq!(ranges.ranges, vec![(1, 7)]);
    }

    #[test]
    fn test_defrag_interruptions() {
        let mut ranges = Ranges::new(vec![(1, 3), (6, 8), (7, 12)]);
        assert_eq!(ranges.ranges, vec![(1, 3), (6, 12)]);
    }

    #[test]
    fn test_defrag_noop() {
        let mut ranges = Ranges::new(vec![(1, 3), (6, 8), (7, 12)]);
        assert_eq!(ranges.ranges, vec![(1, 3), (6, 12)]);
        ranges.defrag();
        assert_eq!(ranges.ranges, vec![(1, 3), (6, 12)]);
    }

    #[test]
    fn test_unordered_ranges() {
        let mut ranges = Ranges::new(vec![(7, 12), (1, 3), (6, 8)]);
        assert_eq!(ranges.ranges, vec![(1, 3), (6, 12)]);
    }

    #[test]
    fn test_contains() {
        let ranges = Ranges::new(vec![(1, 3), (6, 8), (7, 9)]);
        assert!(ranges.contains(1));
        assert!(ranges.contains(2));
        assert!(ranges.contains(3));
        // NOTs
        assert!(!ranges.contains(4));
        assert!(!ranges.contains(5));

        assert!(ranges.contains(6));
        assert!(ranges.contains(7));
        assert!(ranges.contains(8));
        assert!(ranges.contains(9));
    }

    #[test]
    fn test_count() {
        let ranges = Ranges::new(vec![(1, 3), (6, 8), (7, 9), (6, 7)]);
        assert_eq!(ranges.count(), 7);
    }
}
