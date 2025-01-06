遅延セグメントツリー add + min count
範囲加算(add)と範囲最小値カウント(min count)

```
use lazy_segment_tree_util_add_min_max_count::lazy_segment_tree_new_add_min_count;
use min_max_count::MinCount;
let mut seg = lazy_segment_tree_new_add_min_count(vec![1_i32, -1, 5, 3, 2]);
assert_eq!(seg.fold(..), MinCount { min: -1, count: 1 });
seg.act(3.., -3);
assert_eq!(seg.fold(..3), MinCount { min: -1, count: 1 });
assert_eq!(seg.fold(..), MinCount { min: -1, count: 2 });
assert_eq!(seg.get(4), -1);
seg.set(0, -1);
assert_eq!(seg.fold(..), MinCount { min: -1, count: 3 });
seg.act(0, -1);
assert_eq!(seg.fold(..), MinCount { min: -2, count: 1 });
```
