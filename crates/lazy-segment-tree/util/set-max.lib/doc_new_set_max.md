遅延セグメントツリー set + max
範囲セット(set)と範囲最大値(max)
PartialOrd を要求するが、実際には対象のデータは全順序であるべき。

```
use lazy_segment_tree_util_set_min_max::lazy_segment_tree_new_set_max;
let mut seg = lazy_segment_tree_new_set_max(vec![1_i32, -1, 5, 3, 2]);
assert_eq!(seg.fold(..), 5);
seg.act(3.., 7);
assert_eq!(seg.fold(..3), 5);
assert_eq!(seg.fold(..), 7);
assert_eq!(seg.get(4), 7);
seg.set(0, 100);
assert_eq!(seg.fold(..), 100);
```
