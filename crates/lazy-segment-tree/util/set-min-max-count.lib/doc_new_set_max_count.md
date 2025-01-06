遅延セグメントツリー set + max count
範囲セット(set)と範囲最大値カウント(max count)

```
use lazy_segment_tree_util_set_min_max_count::lazy_segment_tree_new_set_max_count;
use min_max_count::MaxCount;
let mut seg = lazy_segment_tree_new_set_max_count(vec![1_i32, -1, 5, 3, 2]);
// [1, -1, 5, 3, 2]
assert_eq!(seg.fold(..), MaxCount { max: 5, count: 1 });
seg.act(3.., 5);
// [1, -1, 5, 5, 5]
assert_eq!(seg.fold(..3), MaxCount { max: 5, count: 1 });
assert_eq!(seg.fold(..), MaxCount { max: 5, count: 3 });
assert_eq!(seg.get(4), 5);
seg.set(0, 8);
// [8, -1, 5, 5, 5]
assert_eq!(seg.fold(..), MaxCount { max: 8, count: 1 });
seg.act(0, 1);
// [1, -1, 5, 5, 5]
assert_eq!(seg.fold(..), MaxCount { max: 5, count: 3 });
```
