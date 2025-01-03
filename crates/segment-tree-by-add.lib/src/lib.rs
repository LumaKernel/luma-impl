use add_monoid::AddMonoid;
use commutative_ring::CommutativeRing;
use segment_tree::segment_tree_new_transparent;
use segment_tree::SegmentTree;

/// # Usage
///
/// ```
/// use segment_tree_by_add::segment_tree_new_by_add;
/// let mut seg = segment_tree_new_by_add(vec![1, 4, 2, 3, 8, 3, 4]);
/// assert_eq!(seg.fold(1..5), 17);
/// ```
pub fn segment_tree_new_by_add<T>(
    v: Vec<T>,
) -> SegmentTree<
    AddMonoid<T>,
    T,
    fn(&AddMonoid<T>, &AddMonoid<T>) -> AddMonoid<T>,
    fn() -> AddMonoid<T>,
    fn(&AddMonoid<T>) -> T,
>
where
    T: CommutativeRing + Clone + 'static,
{
    segment_tree_new_transparent!(AddMonoid<T>, v)
}
