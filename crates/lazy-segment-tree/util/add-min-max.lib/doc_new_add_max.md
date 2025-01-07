遅延セグメントツリー add + max
範囲加算(add)と範囲最大値(max)

```
use lazy_segment_tree_util_add_min_max::lazy_segment_tree_new_add_max;
let mut seg = lazy_segment_tree_new_add_max(vec![1_i32, -1, 5, 3, 2]);
dbg!("aaa");
assert_eq!(seg.fold(..), 5);
seg.act(3.., 5);
assert_eq!(seg.fold(..3), 5);
assert_eq!(seg.fold(..), 8);
assert_eq!(seg.get(4), 7);
seg.set(0, 8);
assert_eq!(seg.fold(..), 8);
seg.act(0, 1);
assert_eq!(seg.fold(..), 9);
```
