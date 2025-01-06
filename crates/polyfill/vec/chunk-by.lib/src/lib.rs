use std::fmt;
use std::iter::FusedIterator;
use std::mem::{self};

/// An iterator over slice in (non-overlapping) chunks separated by a predicate.
///
/// This struct is created by the [`chunk_by`] method on [slices].
///
/// [`chunk_by`]: slice::chunk_by
/// [slices]: slice
pub struct ChunkBy<'a, T: 'a, P> {
    slice: &'a [T],
    predicate: P,
}

impl<'a, T: 'a, P> ChunkBy<'a, T, P> {
    pub(crate) fn new(slice: &'a [T], predicate: P) -> Self {
        ChunkBy { slice, predicate }
    }
}

impl<'a, T: 'a, P> Iterator for ChunkBy<'a, T, P>
where
    P: FnMut(&T, &T) -> bool,
{
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            None
        } else {
            let mut len = 1;
            let mut iter = self.slice.windows(2);
            while let Some([l, r]) = iter.next() {
                if (self.predicate)(l, r) {
                    len += 1
                } else {
                    break;
                }
            }
            let (head, tail) = self.slice.split_at(len);
            self.slice = tail;
            Some(head)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.slice.is_empty() {
            (0, Some(0))
        } else {
            (1, Some(self.slice.len()))
        }
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl<'a, T: 'a, P> DoubleEndedIterator for ChunkBy<'a, T, P>
where
    P: FnMut(&T, &T) -> bool,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            None
        } else {
            let mut len = 1;
            let mut iter = self.slice.windows(2);
            while let Some([l, r]) = iter.next_back() {
                if (self.predicate)(l, r) {
                    len += 1
                } else {
                    break;
                }
            }
            let (head, tail) = self.slice.split_at(self.slice.len() - len);
            self.slice = head;
            Some(tail)
        }
    }
}

impl<'a, T: 'a, P> FusedIterator for ChunkBy<'a, T, P> where P: FnMut(&T, &T) -> bool {}

impl<'a, T: 'a + fmt::Debug, P> fmt::Debug for ChunkBy<'a, T, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChunkBy")
            .field("slice", &self.slice)
            .finish()
    }
}

/// An iterator over slice in (non-overlapping) mutable chunks separated
/// by a predicate.
///
/// This struct is created by the [`chunk_by_mut`] method on [slices].
///
/// [`chunk_by_mut`]: slice::chunk_by_mut
/// [slices]: slice
pub struct ChunkByMut<'a, T: 'a, P> {
    slice: &'a mut [T],
    predicate: P,
}

impl<'a, T: 'a, P> ChunkByMut<'a, T, P> {
    pub(crate) fn new(slice: &'a mut [T], predicate: P) -> Self {
        ChunkByMut { slice, predicate }
    }
}

impl<'a, T: 'a, P> Iterator for ChunkByMut<'a, T, P>
where
    P: FnMut(&T, &T) -> bool,
{
    type Item = &'a mut [T];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            None
        } else {
            let mut len = 1;
            let mut iter = self.slice.windows(2);
            while let Some([l, r]) = iter.next() {
                if (self.predicate)(l, r) {
                    len += 1
                } else {
                    break;
                }
            }
            let slice = mem::take(&mut self.slice);
            let (head, tail) = slice.split_at_mut(len);
            self.slice = tail;
            Some(head)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.slice.is_empty() {
            (0, Some(0))
        } else {
            (1, Some(self.slice.len()))
        }
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl<'a, T: 'a, P> DoubleEndedIterator for ChunkByMut<'a, T, P>
where
    P: FnMut(&T, &T) -> bool,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            None
        } else {
            let mut len = 1;
            let mut iter = self.slice.windows(2);
            while let Some([l, r]) = iter.next_back() {
                if (self.predicate)(l, r) {
                    len += 1
                } else {
                    break;
                }
            }
            let slice = mem::take(&mut self.slice);
            let (head, tail) = slice.split_at_mut(slice.len() - len);
            self.slice = head;
            Some(tail)
        }
    }
}

impl<'a, T: 'a, P> FusedIterator for ChunkByMut<'a, T, P> where P: FnMut(&T, &T) -> bool {}

impl<'a, T: 'a + fmt::Debug, P> fmt::Debug for ChunkByMut<'a, T, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChunkByMut")
            .field("slice", &self.slice)
            .finish()
    }
}

pub trait VecChunkByPolyfill {
    type T;
    fn chunk_by<F>(&self, pred: F) -> ChunkBy<'_, Self::T, F>
    where
        F: FnMut(&Self::T, &Self::T) -> bool;
    fn chunk_by_mut<F>(&mut self, pred: F) -> ChunkByMut<'_, Self::T, F>
    where
        F: FnMut(&Self::T, &Self::T) -> bool;
}
impl<T> VecChunkByPolyfill for [T] {
    type T = T;
    /// Returns an iterator over the slice producing non-overlapping runs
    /// of elements using the predicate to separate them.
    ///
    /// The predicate is called for every pair of consecutive elements,
    /// meaning that it is called on `slice[0]` and `slice[1]`,
    /// followed by `slice[1]` and `slice[2]`, and so on.
    ///
    /// # Examples
    ///
    /// ```
    /// use polyfill_vec_chunk_by::*;
    /// let slice = &[1, 1, 1, 3, 3, 2, 2, 2];
    ///
    /// let mut iter = slice.chunk_by(|a, b| a == b);
    ///
    /// assert_eq!(iter.next(), Some(&[1, 1, 1][..]));
    /// assert_eq!(iter.next(), Some(&[3, 3][..]));
    /// assert_eq!(iter.next(), Some(&[2, 2, 2][..]));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// This method can be used to extract the sorted subslices:
    ///
    /// ```
    /// use polyfill_vec_chunk_by::*;
    /// let slice = &[1, 1, 2, 3, 2, 3, 2, 3, 4];
    ///
    /// let mut iter = slice.chunk_by(|a, b| a <= b);
    ///
    /// assert_eq!(iter.next(), Some(&[1, 1, 2, 3][..]));
    /// assert_eq!(iter.next(), Some(&[2, 3][..]));
    /// assert_eq!(iter.next(), Some(&[2, 3, 4][..]));
    /// assert_eq!(iter.next(), None);
    /// ```
    #[inline]
    fn chunk_by<F>(&self, pred: F) -> ChunkBy<Self::T, F>
    where
        F: FnMut(&Self::T, &Self::T) -> bool,
    {
        ChunkBy::new(self, pred)
    }

    /// Returns an iterator over the slice producing non-overlapping mutable
    /// runs of elements using the predicate to separate them.
    ///
    /// The predicate is called for every pair of consecutive elements,
    /// meaning that it is called on `slice[0]` and `slice[1]`,
    /// followed by `slice[1]` and `slice[2]`, and so on.
    ///
    /// # Examples
    ///
    /// ```
    /// use polyfill_vec_chunk_by::*;
    /// let slice = &mut [1, 1, 1, 3, 3, 2, 2, 2];
    ///
    /// let mut iter = slice.chunk_by_mut(|a, b| a == b);
    ///
    /// assert_eq!(iter.next(), Some(&mut [1, 1, 1][..]));
    /// assert_eq!(iter.next(), Some(&mut [3, 3][..]));
    /// assert_eq!(iter.next(), Some(&mut [2, 2, 2][..]));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// This method can be used to extract the sorted subslices:
    ///
    /// ```
    /// use polyfill_vec_chunk_by::*;
    /// let slice = &mut [1, 1, 2, 3, 2, 3, 2, 3, 4];
    ///
    /// let mut iter = slice.chunk_by_mut(|a, b| a <= b);
    ///
    /// assert_eq!(iter.next(), Some(&mut [1, 1, 2, 3][..]));
    /// assert_eq!(iter.next(), Some(&mut [2, 3][..]));
    /// assert_eq!(iter.next(), Some(&mut [2, 3, 4][..]));
    /// assert_eq!(iter.next(), None);
    /// ```
    #[inline]
    fn chunk_by_mut<F>(&mut self, pred: F) -> ChunkByMut<'_, T, F>
    where
        F: FnMut(&T, &T) -> bool,
    {
        ChunkByMut::new(self, pred)
    }
}
