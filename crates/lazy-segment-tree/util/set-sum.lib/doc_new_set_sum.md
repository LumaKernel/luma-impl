遅延セグメントツリー add + sum
範囲加算(add)と範囲和(sum)

```
use lazy_segment_tree_util_set_sum::lazy_segment_tree_new_set_sum;
let mut seg = lazy_segment_tree_new_set_sum(vec![1_i32, -1, 5, 3, 2]);
assert_eq!(seg.fold(..), 10);
seg.act(3.., 7);
assert_eq!(seg.fold(..3), 5);
assert_eq!(seg.fold(..), 19);
assert_eq!(seg.get(4), 7);
seg.set(0, 100);
assert_eq!(seg.fold(..), 118);
```
