use std::ops;

#[allow(non_camel_case_types)]
pub trait PermutationInternal_ToUsize {
    fn to_usize(&self) -> usize;
}

macro_rules! impl_to_usize {
    ($($t:ty),*) => {
        $(
            impl PermutationInternal_ToUsize for $t {
                fn to_usize(&self) -> usize {
                    *self as usize
                }
            }
        )*
    };
}
impl_to_usize!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

// TODO: .get to .get_unchecked
/// # 順列
/// 対称群の元とも見做せる。
/// 同じ大きさの順列に限定すれば、モノイドを成す。
/// 0-indexedで扱う必要がある。
#[derive(Debug, PartialEq, Eq)]
pub struct Permutation {
    // 0-indexed permutation
    p: Vec<usize>,
}
impl Permutation {
    pub fn into_vec(self) -> Vec<usize> {
        self.p
    }
    pub fn as_vec(&self) -> &Vec<usize> {
        &self.p
    }

    /// # 長さ
    pub fn size(&self) -> usize {
        self.p.len()
    }

    /// # 逆元
    ///
    /// 順列Pについて、Q[P[i]] = i であるようなただひとつの順列Qを返す
    /// 対称群としては逆元に相当する
    /// P[Q[i]] = i でもある
    /// Pが恒等関数(単位元)の場合、P=Q となる
    pub fn inv(&self) -> Permutation {
        let mut q = vec![0; self.size()];
        for (i, r) in self.p.iter().enumerate() {
            q[*r] = i;
        }
        Permutation { p: q }
    }

    /// # 順列の合成
    /// 順列P, Qについて、R[i] = P[Q[i]] となるような順列Rを返す
    pub fn compose(&self, other: &Permutation) -> Permutation {
        if self.size() != other.size() {
            panic!("different length: {} != {}", self.size(), other.size());
        }
        let mut q = vec![0; self.size()];
        for (i, r) in self.p.iter().enumerate() {
            q[i] = other.p[*r];
        }
        Permutation { p: q }
    }

    /// # すべてのループの取得
    ///
    /// 順列はいくつかの独立なループのみによって構成される。
    /// 長さ k のループ v は以下を満たす。
    /// `P[v[0]] = v[1], P[v[1]] = v[2], ..., P[v[k-1]] = v[0]`
    pub fn loops(&self) -> Vec<Vec<usize>> {
        let mut used = vec![false; self.size()];
        let mut loops = vec![];
        for i in 0..self.size() {
            if used[i] {
                continue;
            }
            let mut one_loop = vec![i];
            let mut j = self.p[i];
            used[i] = true;
            while i != j {
                one_loop.push(j);
                used[j] = true;
                j = self.p[j];
            }
            loops.push(one_loop);
        }
        loops
    }

    /// # iを含むループの取得
    ///
    /// 長さ k のループ v は以下を満たす。
    /// `P[v[0]] = v[1], P[v[1]] = v[2], ..., P[v[k-1]] = v[0]`
    ///
    /// ## Panic-free Preconditions
    ///
    /// - i < self.size()
    pub fn loop_of(&self, i: usize) -> Vec<usize> {
        assert!(i < self.size(), "i={} >= size={}", i, self.size());
        let mut one_loop = vec![i];
        let mut j = self.p[i];
        while j != i {
            one_loop.push(j);
            j = self.p[j];
        }
        one_loop
    }

    /// TODO: 別のライブラリへ
    /// # 位数
    ///
    /// $P^k = I$ となるような最小の $k$ を返す
    /// すべてのループ長の最小公倍数になる
    //pub fn rank(&self) -> usize {
    //    let mut rank = 1;
    //    for loop_ in self.loops() {
    //        rank = rank.lcm(&loop_.len());
    //    }
    //    rank
    //}

    /// # 互換
    ///
    /// `i` と `j` の互換となるような順列を返す
    pub fn swap(size: usize, i: usize, j: usize) -> Permutation {
        assert!(i < size, "i={} >= size={}", i, size);
        assert!(j < size, "j={} >= size={}", j, size);
        let mut p = (0..size).collect::<Vec<_>>();
        p[i] = j;
        p[j] = i;
        Permutation { p }
    }

    /// # ループから順列を生成
    ///
    /// 長さkの配列vについて、 `P[v[0]] = v[1], P[v[1]] = v[2], ..., P[v[k-1]] = v[0]` となるような順列Pを返す。
    ///
    /// # Panic-free Preconditions
    ///
    /// - `v` は空でない
    /// - `v` の要素はすべて0以上size未満
    /// - `v` の要素はすべて相異なる
    pub fn from_loop(size: usize, v: &[usize]) -> Permutation {
        {
            assert!(!v.is_empty(), "v is empty");
            let mut used = vec![false; size];
            for &i in v {
                assert!(i < size, "i={} >= size={}", i, size);
                assert!(!used[i], "i={} is duplicated", i);
                used[i] = true;
            }
        }
        let mut p = (0..size).collect::<Vec<_>>();
        if v.len() == 1 {
            return Permutation { p };
        }
        for i in 1..v.len() {
            p[v[i - 1]] = v[i];
        }
        p[v[v.len() - 1]] = v[0];
        Permutation { p }
    }

    /// # 恒等順列
    pub fn identity(size: usize) -> Permutation {
        Permutation {
            p: (0..size).collect(),
        }
    }
}
impl<T: PermutationInternal_ToUsize> TryFrom<&[T]> for Permutation {
    type Error = &'static str;
    /// 配列は0-indexedな順列である必要がある
    ///
    /// ## Panic-free Preconditions
    ///
    /// - 配列の要素はすべて $0$ 以上 $N$ 未満
    fn try_from(v: &[T]) -> Result<Self, Self::Error> {
        if !is_permutation0(v) {
            return Err("not a permutation");
        }
        Ok(Permutation {
            p: v.iter().map(|x| x.to_usize()).collect(),
        })
    }
}
impl<T: PermutationInternal_ToUsize> TryFrom<&Vec<T>> for Permutation {
    type Error = &'static str;
    /// 配列は0-indexedな順列である必要がある
    ///
    /// ## Panic-free Preconditions
    ///
    /// - 配列の要素はすべて $0$ 以上 $N$ 未満
    fn try_from(v: &Vec<T>) -> Result<Self, Self::Error> {
        if !is_permutation0(v) {
            return Err("not a permutation");
        }
        Ok(Permutation {
            p: v.iter().map(|x| x.to_usize()).collect(),
        })
    }
}
impl TryFrom<Vec<usize>> for Permutation {
    type Error = &'static str;
    /// 配列は0-indexedな順列である必要がある
    ///
    /// ## Panic-free Preconditions
    ///
    /// - 配列の要素はすべて $0$ 以上 $N$ 未満
    fn try_from(v: Vec<usize>) -> Result<Self, Self::Error> {
        if !is_permutation0(&v) {
            return Err("not a permutation");
        }
        Ok(Permutation { p: v })
    }
}
impl From<Permutation> for Vec<usize> {
    fn from(p: Permutation) -> Self {
        p.p
    }
}

impl ops::Mul for Permutation {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self.compose(&rhs)
    }
}
impl ops::Mul<&Permutation> for Permutation {
    type Output = Permutation;
    fn mul(self, rhs: &Permutation) -> Permutation {
        self.compose(rhs)
    }
}
impl ops::Mul<Permutation> for &Permutation {
    type Output = Permutation;
    fn mul(self, rhs: Permutation) -> Permutation {
        self.compose(&rhs)
    }
}
impl ops::Mul<&Permutation> for &Permutation {
    type Output = Permutation;
    fn mul(self, rhs: &Permutation) -> Permutation {
        self.compose(rhs)
    }
}
impl ops::MulAssign for Permutation {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.compose(&rhs);
    }
}
impl ops::MulAssign<&Permutation> for Permutation {
    fn mul_assign(&mut self, rhs: &Permutation) {
        *self = self.compose(rhs);
    }
}
impl ops::Index<usize> for Permutation {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        &self.p[index]
    }
}
impl ops::IndexMut<usize> for Permutation {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.p[index]
    }
}

/// # 順列か判定する (0-indexed)
///
/// 長さ $N$ の配列に対し、 $O(N)$ で判定する
///
/// ## Panic-free Preconditions
///
/// - 配列の要素はすべて $0$ 以上 $N$ 未満
pub fn is_permutation0<T: PermutationInternal_ToUsize>(v: &[T]) -> bool {
    let n = v.len();
    let mut used = vec![false; n];
    for x in v {
        let x: usize = x.to_usize();
        match used.get_mut(x) {
            Some(b) => {
                if *b {
                    return false;
                }
                *b = true;
            }
            None => return false,
        }
    }
    true
}

#[cfg(test)]
mod permutation_test;
