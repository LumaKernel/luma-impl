use crate::*;

#[test]
fn test_u32_unchecked() {
    unsafe {
        assert_eq!(0, ceil_log2_u32_unchecked(1));
        assert_eq!(1, ceil_log2_u32_unchecked(2));
        assert_eq!(2, ceil_log2_u32_unchecked(3));
        assert_eq!(2, ceil_log2_u32_unchecked(4));
        assert_eq!(3, ceil_log2_u32_unchecked(5));
        assert_eq!(3, ceil_log2_u32_unchecked(6));
        assert_eq!(3, ceil_log2_u32_unchecked(7));
        assert_eq!(3, ceil_log2_u32_unchecked(8));
        assert_eq!(4, ceil_log2_u32_unchecked(9));
        assert_eq!(30, ceil_log2_u32_unchecked(1 << 30));
        assert_eq!(31, ceil_log2_u32_unchecked((1 << 30) + 1));

        assert_eq!(32, ceil_log2_u32_unchecked(u32::MAX));
    }
}

#[test]
fn test_u32() {
    assert_eq!(0, ceil_log2_u32(0));
    assert_eq!(0, ceil_log2_u32(1));
    assert_eq!(1, ceil_log2_u32(2));
    assert_eq!(2, ceil_log2_u32(3));
    assert_eq!(2, ceil_log2_u32(4));
    assert_eq!(3, ceil_log2_u32(5));
    assert_eq!(3, ceil_log2_u32(6));
    assert_eq!(3, ceil_log2_u32(7));
    assert_eq!(3, ceil_log2_u32(8));
    assert_eq!(4, ceil_log2_u32(9));
    assert_eq!(30, ceil_log2_u32(1 << 30));
    assert_eq!(31, ceil_log2_u32((1 << 30) + 1));

    assert_eq!(32, ceil_log2_u32(u32::MAX));
}

#[test]
fn test_u64_unchecked() {
    unsafe {
        assert_eq!(0, ceil_log2_u64_unchecked(1));
        assert_eq!(1, ceil_log2_u64_unchecked(2));
        assert_eq!(2, ceil_log2_u64_unchecked(3));
        assert_eq!(2, ceil_log2_u64_unchecked(4));
        assert_eq!(3, ceil_log2_u64_unchecked(5));
        assert_eq!(3, ceil_log2_u64_unchecked(6));
        assert_eq!(3, ceil_log2_u64_unchecked(7));
        assert_eq!(3, ceil_log2_u64_unchecked(8));
        assert_eq!(4, ceil_log2_u64_unchecked(9));
        assert_eq!(30, ceil_log2_u64_unchecked(1 << 30));
        assert_eq!(31, ceil_log2_u64_unchecked((1 << 30) + 1));
        assert_eq!(40, ceil_log2_u64_unchecked(1 << 40));
        assert_eq!(41, ceil_log2_u64_unchecked((1 << 40) + 1));

        assert_eq!(64, ceil_log2_u64_unchecked(u64::MAX));
    }
}

#[test]
fn test_u64() {
    assert_eq!(0, ceil_log2_u64(0));
    assert_eq!(0, ceil_log2_u64(1));
    assert_eq!(1, ceil_log2_u64(2));
    assert_eq!(2, ceil_log2_u64(3));
    assert_eq!(2, ceil_log2_u64(4));
    assert_eq!(3, ceil_log2_u64(5));
    assert_eq!(3, ceil_log2_u64(6));
    assert_eq!(3, ceil_log2_u64(7));
    assert_eq!(3, ceil_log2_u64(8));
    assert_eq!(4, ceil_log2_u64(9));
    assert_eq!(30, ceil_log2_u64(1 << 30));
    assert_eq!(31, ceil_log2_u64((1 << 30) + 1));
    assert_eq!(40, ceil_log2_u64(1 << 40));
    assert_eq!(41, ceil_log2_u64((1 << 40) + 1));

    assert_eq!(64, ceil_log2_u64(u64::MAX));
}

#[test]
fn test_u128_unchecked() {
    unsafe {
        assert_eq!(0, ceil_log2_u128_unchecked(1));
        assert_eq!(1, ceil_log2_u128_unchecked(2));
        assert_eq!(2, ceil_log2_u128_unchecked(3));
        assert_eq!(2, ceil_log2_u128_unchecked(4));
        assert_eq!(3, ceil_log2_u128_unchecked(5));
        assert_eq!(3, ceil_log2_u128_unchecked(6));
        assert_eq!(3, ceil_log2_u128_unchecked(7));
        assert_eq!(3, ceil_log2_u128_unchecked(8));
        assert_eq!(4, ceil_log2_u128_unchecked(9));
        assert_eq!(30, ceil_log2_u128_unchecked(1 << 30));
        assert_eq!(31, ceil_log2_u128_unchecked((1 << 30) + 1));
        assert_eq!(40, ceil_log2_u128_unchecked(1 << 40));
        assert_eq!(41, ceil_log2_u128_unchecked((1 << 40) + 1));

        assert_eq!(128, ceil_log2_u128_unchecked(u128::MAX));
    }
}

#[test]
fn test_u128() {
    assert_eq!(0, ceil_log2_u128(0));
    assert_eq!(0, ceil_log2_u128(1));
    assert_eq!(1, ceil_log2_u128(2));
    assert_eq!(2, ceil_log2_u128(3));
    assert_eq!(2, ceil_log2_u128(4));
    assert_eq!(3, ceil_log2_u128(5));
    assert_eq!(3, ceil_log2_u128(6));
    assert_eq!(3, ceil_log2_u128(7));
    assert_eq!(3, ceil_log2_u128(8));
    assert_eq!(4, ceil_log2_u128(9));
    assert_eq!(30, ceil_log2_u128(1 << 30));
    assert_eq!(31, ceil_log2_u128((1 << 30) + 1));
    assert_eq!(40, ceil_log2_u128(1 << 40));
    assert_eq!(41, ceil_log2_u128((1 << 40) + 1));

    assert_eq!(128, ceil_log2_u128(u128::MAX));
}

#[test]
fn test_usize_unchecked() {
    unsafe {
        assert_eq!(0, ceil_log2_usize_unchecked(1));
        assert_eq!(1, ceil_log2_usize_unchecked(2));
        assert_eq!(2, ceil_log2_usize_unchecked(3));
        assert_eq!(2, ceil_log2_usize_unchecked(4));
        assert_eq!(3, ceil_log2_usize_unchecked(5));
        assert_eq!(3, ceil_log2_usize_unchecked(6));
        assert_eq!(3, ceil_log2_usize_unchecked(7));
        assert_eq!(3, ceil_log2_usize_unchecked(8));
        assert_eq!(4, ceil_log2_usize_unchecked(9));
        assert_eq!(30, ceil_log2_usize_unchecked(1 << 30));
        assert_eq!(31, ceil_log2_usize_unchecked((1 << 30) + 1));
    }
}

#[test]
fn test_usize() {
    assert_eq!(0, ceil_log2_usize(0));
    assert_eq!(0, ceil_log2_usize(1));
    assert_eq!(1, ceil_log2_usize(2));
    assert_eq!(2, ceil_log2_usize(3));
    assert_eq!(2, ceil_log2_usize(4));
    assert_eq!(3, ceil_log2_usize(5));
    assert_eq!(3, ceil_log2_usize(6));
    assert_eq!(3, ceil_log2_usize(7));
    assert_eq!(3, ceil_log2_usize(8));
    assert_eq!(4, ceil_log2_usize(9));
    assert_eq!(30, ceil_log2_usize(1 << 30));
    assert_eq!(31, ceil_log2_usize((1 << 30) + 1));
}
