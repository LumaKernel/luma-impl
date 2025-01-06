use commutative_ring::CommutativeRing;

/// CommutativeRingOrd
/// 順序環
/// - 環であり、全順序が定義されていて、の以下性質をすべて満たす
///   - a < b ならば a + c < b + c
///   - 0 < a かつ 0 < b ならば 0 < a * b
///   - 0 < 1
///   - a < b ならば -a > -b
pub trait CommutativeRingOrd: CommutativeRing + Ord {}
