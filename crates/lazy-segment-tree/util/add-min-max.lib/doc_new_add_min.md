遅延セグメントツリー add + min
範囲加算(add)と範囲最小値(min)

```
use lazy_segment_tree_util_add_min_max::lazy_segment_tree_new_add_min;
let mut seg = lazy_segment_tree_new_add_min(vec![1_i32, -1, 5, 3, 2]);
assert_eq!(seg.fold(..), -1);
seg.act(3.., -3);
assert_eq!(seg.fold(..3), -1);
assert_eq!(seg.fold(..), -1);
assert_eq!(seg.get(4), -1);
seg.set(0, -1);
assert_eq!(seg.fold(..), -1);
seg.act(0, -1);
assert_eq!(seg.fold(..), -2);
```
