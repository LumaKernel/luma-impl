macro_rules! impl_fn {
    ($t:ty, $fn_name:ident, $fn_name_unchecked:ident) => {
        // NOTE: https://github.com/rust-lang/rust/issues/85122
        // NOTE: It's stable in 1.79.0, which cannot be used in AtCoder
        // /// ceil(log2(n))
        // ///
        // /// # Examples
        // ///
        // /// ```
        // #[doc = concat!("use ceil_log2::", stringify!($fn_name_unchecked), ";")]
        // /// unsafe {
        // #[doc = concat!("    assert_eq!(", stringify!($fn_name_unchecked), "(0b0001), 0);")]
        // #[doc = concat!("    assert_eq!(", stringify!($fn_name_unchecked), "(0b0010), 1);")]
        // #[doc = concat!("    assert_eq!(", stringify!($fn_name_unchecked), "(0b0011), 2);")]
        // #[doc = concat!("    assert_eq!(", stringify!($fn_name_unchecked), "(0b0100), 2);")]
        // #[doc = concat!("    assert_eq!(", stringify!($fn_name_unchecked), "(0b0101), 3);")]
        // #[doc = concat!("    assert_eq!(", stringify!($fn_name_unchecked), "(0b1000), 3);")]
        // #[doc = concat!("    assert_eq!(", stringify!($fn_name_unchecked), "(0b1001), 4);")]
        // /// }
        // /// ```
        // ///
        // /// # Safety
        // ///
        // /// n must be greater than 0.
        // #[inline]
        // pub unsafe fn $fn_name_unchecked(n: $t) -> usize {
        //     debug_assert!(n > 0);
        //     const fn bit_length() -> usize {
        //         8 * std::mem::size_of::<$t>()
        //     }
        //     bit_length() - n.unchecked_sub(1).leading_zeros() as usize
        // }

        /// ceil(log2(n))
        ///
        /// # Examples
        ///
        /// ```
        #[doc = concat!("use ceil_log2::", stringify!($fn_name), ";")]
        #[doc = concat!("assert_eq!(", stringify!($fn_name), "(0b0000), 0);")]
        #[doc = concat!("assert_eq!(", stringify!($fn_name), "(0b0001), 0);")]
        #[doc = concat!("assert_eq!(", stringify!($fn_name), "(0b0010), 1);")]
        #[doc = concat!("assert_eq!(", stringify!($fn_name), "(0b0011), 2);")]
        #[doc = concat!("assert_eq!(", stringify!($fn_name), "(0b0100), 2);")]
        #[doc = concat!("assert_eq!(", stringify!($fn_name), "(0b0101), 3);")]
        #[doc = concat!("assert_eq!(", stringify!($fn_name), "(0b1000), 3);")]
        #[doc = concat!("assert_eq!(", stringify!($fn_name), "(0b1001), 4);")]
        /// ```
        #[inline]
        pub fn $fn_name(n: $t) -> usize {
            const fn bit_length() -> usize {
                8 * std::mem::size_of::<$t>()
            }
            bit_length() - n.saturating_sub(1).leading_zeros() as usize
        }
    };
}

impl_fn!(usize, ceil_log2_usize, ceil_log2_usize_unchecked);
impl_fn!(u8, ceil_log2_u8, ceil_log2_u8_unchecked);
impl_fn!(u16, ceil_log2_u16, ceil_log2_u16_unchecked);
impl_fn!(u32, ceil_log2_u32, ceil_log2_u32_unchecked);
impl_fn!(u64, ceil_log2_u64, ceil_log2_u64_unchecked);
impl_fn!(u128, ceil_log2_u128, ceil_log2_u128_unchecked);

#[cfg(test)]
mod ceil_log2_test;
