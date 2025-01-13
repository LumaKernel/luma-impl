use max_exists::MaxExists;
use min_monoid::MinMonoid;
use segment_tree::segment_tree_new_transparent;
use segment_tree::SegmentTree;

/// # Usage
///
/// ```
/// use segment_tree_by_min::segment_tree_new_by_min;
/// let mut seg = segment_tree_new_by_min(vec![1, 4, 2, 3, 8, 3, 4]);
/// assert_eq!(seg.fold(1..5), 2);
/// ```
pub fn segment_tree_new_by_min<T>(
    v: Vec<T>,
) -> SegmentTree<
    MinMonoid<T>,
    T,
    fn(&MinMonoid<T>, &MinMonoid<T>) -> MinMonoid<T>,
    fn() -> MinMonoid<T>,
    fn(&MinMonoid<T>) -> T,
>
where
    T: PartialOrd + Ord + MaxExists + Clone + 'static,
{
    segment_tree_new_transparent!(MinMonoid<T>, v)
}
