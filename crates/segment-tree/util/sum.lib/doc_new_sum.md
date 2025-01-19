```
use segment_tree_util_sum::segment_tree_new_sum;
let mut seg = segment_tree_new_sum(vec![1, 4, 2, 3, 8, 3, 4]);
assert_eq!(seg.fold(1..5), 17);
assert_eq!(seg.fold(..), 25);
assert_eq!(seg.fold(0), 1);
seg.set(2, 5);
assert_eq!(seg.fold(1..5), 20);
assert_eq!(seg.fold(..), 28);
assert_eq!(seg.fold(0), 1);
```
