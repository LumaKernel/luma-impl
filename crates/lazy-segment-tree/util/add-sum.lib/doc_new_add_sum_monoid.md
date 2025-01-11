遅延セグメントツリー add + sum
範囲加算(add)と範囲和(sum)
任意のモノイド上で実現する。
Binary Exponentiation (二分累乗) を利用して任意のサイズに対しての累積を求める。

ビルトインの整数型の加算や、modintの加算に関してはBinExpより高速な方法での累積が計算できるため、 `lazy_segment_tree_new_add_sum` を単に代わりに利用すべきだろう。

TODO: 行列による例
