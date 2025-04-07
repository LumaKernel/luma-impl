遅延セグメントツリー set + min
範囲セット(set)と範囲最大値(min)
PartialOrd を要求するが、実際には対象のデータは全順序であるべき。

```
use lazy_segment_tree_util_set_min_max::lazy_segment_tree_new_set_min;
let mut seg = lazy_segment_tree_new_set_min(vec![1_i32, -1, 5, 3, 2]);
// [1, -1, 5, 3, 2]
assert_eq!(seg.fold(..), -1);
seg.act(3.., 7);
// [1, -1, 5, 7, 7]
assert_eq!(seg.fold(..3), -1);
assert_eq!(seg.fold(..), -1);
assert_eq!(seg.get(4), 7);
seg.set(0, -100);
// [-100, -1, 5, 7, 7]
assert_eq!(seg.fold(..), -100);
```
