#![allow(clippy::type_complexity)]
use max_monoid::MaxMonoid;
use min_exists::MinExists;
use segment_tree::segment_tree_new_transparent;
use segment_tree::SegmentTree;

/// # Usage
///
/// ```
/// use segment_tree_by_max::segment_tree_new_by_max;
/// let mut seg = segment_tree_new_by_max(vec![1, 4, 2, 3, 8, 3, 4]);
/// assert_eq!(seg.fold(1..5), 8);
/// ```
pub fn segment_tree_new_by_max<T>(
    v: Vec<T>,
) -> SegmentTree<
    MaxMonoid<T>,
    T,
    fn(&MaxMonoid<T>, &MaxMonoid<T>) -> MaxMonoid<T>,
    fn() -> MaxMonoid<T>,
    fn(&MaxMonoid<T>) -> T,
>
where
    T: PartialOrd + Ord + MinExists + Clone + 'static,
{
    segment_tree_new_transparent!(MaxMonoid<T>, v)
}
