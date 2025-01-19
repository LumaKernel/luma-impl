
```
use segment_tree_util_min_max::segment_tree_new_min;
let mut seg = segment_tree_new_min(vec![1, 4, 2, 3, 8, 3, 4]);
assert_eq!(seg.fold(1..5), 2);
```
