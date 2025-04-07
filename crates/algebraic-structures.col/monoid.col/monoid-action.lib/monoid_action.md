## 記法

- `id() := 0`
- `op(a, b) := a . b`
- `act_id() := !`
- `act_op(t, u) := t . u`
- `act_app(t, a) := t * a`

## 要件

- `id` と `op` はモノイドを成す
- `act_id` と `act_op` はモノイドを成す
- 単位元の作用: `! * a = a`
- 分配律1: `(t . u) * a = t * (u * a)`
- 分配律2: `t * (a . b) = (t * a) . b`
