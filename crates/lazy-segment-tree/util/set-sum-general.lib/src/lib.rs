use lazy_segment_tree::LazySegmentTree;
use lazy_segment_tree_util_new_with_len::lazy_segment_tree_new_with_len;
use lazy_segment_tree_util_type::lazy_seg_type;
use std::rc::Rc;

pub fn lazy_segment_tree_new_set_sum_general<T>(
    vec: Vec<T>,
    op: impl Fn(&T, &T) -> T,
    id: impl Fn() -> T,
) -> lazy_seg_type!(T = (T, usize), A = Option<T>, TFolded = T, TGetter = T, TSetter = T, ASetter = T)
where
    T: Clone,
{
    let op = Rc::new(op);
    let id = Rc::new(id);
    lazy_segment_tree_new_with_len(
        vec,
        {
            let op = op.clone();
            move |a, b| (*op)(a, b)
        },
        {
            let id = id.clone();
            move || (*id)()
        },
        move |x: &Option<T>, y: &Option<T>| {
            x.as_ref().map_or_else(|| y.clone(), |x| Some(x.clone()))
        },
        || None,
        move |x, y, len| {
            let x = match x {
                Some(x) => x,
                None => return y.clone(),
            };
            let mut v = id();
            let mut w = x.clone();
            let mut k = 1;
            while k < len {
                if len & k != 0 {
                    v = op(&v, &w);
                }
                w = op(&w, &w);
                k <<= 1;
            }
            v
        },
    )
    .set_action_setter(|x| Some(x))
}
