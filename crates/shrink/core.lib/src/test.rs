use std::ops::RangeBounds;

use crate::shrink;

macro_rules! s_simple_1 {
    () => {
        // ...  1  2  3  ...
        // -->  ^  ^  ^  <--
        //  ^   |  |  |   ^
        //  |   |  |  |   |
        //  X   0  1  2  (3)
        shrink(vec![1, 2, 3])
    };
}

#[test]
fn simple_1() {
    let s = s_simple_1!();
    assert_eq!(s.shrinked_len(), 3);
    assert_eq!(s.shrink(..1), ..0);
    assert_eq!(s.shrink(1), 0_usize);
    assert_eq!(s.shrink(2), 1_usize);
    assert_eq!(s.shrink(3), 2_usize);
    assert_eq!(s.shrink(4..), 3..);

    assert_eq!(s.shrink(..2), ..1);
    assert_eq!(s.shrink(1..), 0..);
    assert_eq!(s.shrink(1..2), 0..1);
    assert_eq!(s.shrink(..), ..);
}

#[test]
#[should_panic]
fn simple_1_shrink_panic_1() {
    let s = s_simple_1!();
    s.shrink(0);
}

#[test]
#[should_panic]
fn simple_1_shrink_panic_2() {
    let s = s_simple_1!();
    s.shrink(0);
}

#[test]
#[should_panic]
fn simple_1_unshrink_panic_1() {
    let s = s_simple_1!();
    s.unshrink(4);
}

macro_rules! s_simple_2 {
    () => {
        // ...  1  2 .. 9  10  ...
        // -->  ^  <---->  ^  <--
        //  ^   |    ^     |   ^
        //  |   |    |     |   |
        //  X   0    1     2  (3)
        shrink(vec![1, 1, 10, 1])
    };
}

#[test]
fn simple_2() {
    let s = s_simple_2!();
    assert_eq!(s.shrinked_len(), 3);
    assert_eq!(s.shrink(..=1), ..=0);
    assert_eq!(s.shrink(..1), ..0);
    assert_eq!(s.shrink(1), 0_usize);
    assert_eq!(s.shrink(2..=9), 1..=1);
    assert_eq!(s.shrink(2..10), 1..2);
    assert_eq!(s.shrink(10), 2_usize);
    assert_eq!(s.shrink(10..), 2..);
    assert_eq!(s.shrink(11..), 3..);

    assert!(!s.unshrink(0).contains(&0));
    assert!(s.unshrink(0).contains(&1));
    assert!(!s.unshrink(0).contains(&2));

    assert!(!s.unshrink(1).contains(&1));
    assert!(s.unshrink(1).contains(&2));
    assert!(s.unshrink(1).contains(&3));
    assert!(s.unshrink(1).contains(&9));
    assert!(!s.unshrink(1).contains(&10));

    assert_eq!(s.unshrink(0).count(), 1_u32);
    assert_eq!(s.unshrink(1).count(), 8_u32);
    assert_eq!(s.unshrink(2).count(), 1_u32);
}

#[test]
#[should_panic]
fn simple_2_unshrink_panic_1() {
    let s = s_simple_2!();
    s.unshrink(3).count();
}

#[test]
fn simple_3() {
    // ...  1  2  3  4  5  6  7  8  9  ...
    // -->  ^  ^  ^  ^  ^  <-->  ^  ^  <--
    //  ^   |  |  |  |  |   ^    |  |   ^
    //  |   |  |  |  |  |   |    |  |   |
    //  X   0  1  2  2  4   5    6  7  (8)
    let s = shrink(vec![1, 5, 3, 9, 3, 8, 5]);
    assert_eq!(s.shrinked_len(), 8);
    assert_eq!(s.shrink(..=1), ..=0);
    assert_eq!(s.shrink(..1), ..0);
    assert_eq!(s.shrink(1), 0_usize);
    assert_eq!(s.shrink(2), 1_usize);
    assert_eq!(s.shrink(3), 2_usize);
    assert_eq!(s.shrink(4), 3_usize);
    assert_eq!(s.shrink(5), 4_usize);
    assert_eq!(s.shrink(6..=7), 5..=5);
    assert_eq!(s.shrink(6..8), 5..6);
    assert_eq!(s.shrink(8), 6_usize);
    assert_eq!(s.shrink(9), 7_usize);
    assert_eq!(s.shrink(10..), 8..);

    assert_eq!(s.size_of_shrinked(0), 1_u32);
    assert_eq!(s.size_of_shrinked(1), 1_u32);
    assert_eq!(s.size_of_shrinked(2), 1_u32);
    assert_eq!(s.size_of_shrinked(3), 1_u32);
    assert_eq!(s.size_of_shrinked(4), 1_u32);
    assert_eq!(s.size_of_shrinked(5), 2_u32);
    assert_eq!(s.size_of_shrinked(6), 1_u32);
    assert_eq!(s.size_of_shrinked(7), 1_u32);
}

macro_rules! s_wide_1 {
    () => {
        // ...  -123456789 [..]  987654321  ...
        // -->       ^     <-->      ^      <--
        //  ^        |      ^        |       ^
        //  |        |      |        |       |
        //  X        0      1        2      (3)
        shrink(vec![-123456789, 987654321])
    };
}

#[test]
fn wide_1() {
    let s = s_wide_1!();
    assert_eq!(s.shrinked_len(), 3);
    assert_eq!(s.shrink(-123456789), 0_usize);
    assert_eq!(s.shrink(-123456789..), 0..);
    assert_eq!(s.shrink(-123456788..), 1..);
    assert_eq!(s.shrink(987654321), 2_usize);
    assert_eq!(s.shrink(987654322..), 3..);

    assert!(!s.unshrink(1).contains(&-123456789));
    assert!(s.unshrink(1).contains(&-123456788));
    assert!(s.unshrink(1).contains(&0));
    assert!(s.unshrink(1).contains(&987654320));
    assert!(!s.unshrink(1).contains(&987654321));
}

#[test]
#[should_panic]
fn wide_1_shrinkk_panic_1() {
    let s = s_wide_1!();
    s.shrink(-123456790..);
}

#[test]
fn corner() {
    let s = shrink(vec![u8::MIN, u8::MAX]);
    assert_eq!(s.shrinked_len(), 3);
    assert_eq!(s.shrink(0), 0_usize);
    assert_eq!(s.shrink(..0), ..0);
    assert_eq!(s.shrink(..=0), ..=0);
    assert_eq!(s.shrink(u8::MAX), 2_usize);
    assert_eq!(s.shrink(..u8::MAX), ..2);
    assert_eq!(s.shrink(..=u8::MAX), ..=2);
    assert_eq!(s.shrink(u8::MIN..u8::MAX), 0..2);
    assert_eq!(s.shrink(u8::MIN..=u8::MAX), 0..=2);
    assert_eq!(s.shrink(u8::MAX..), 2..);
}
