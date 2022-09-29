小説投稿サイト「[小説家になろう](https://syosetu.com/)」では各小説に番号が割り当てられています。
番号は番号のままでで表される場合と[Nコード](https://syosetu.com/bbstopic/top/topicid/2733/)と呼ばれる符号で表される場合があります。
このライブラリはその変換を行います。
[`Ncode`](Ncode) は数値か文字列と相互変換可能な型です。

# サンプル
```rust
use ncode::Ncode;
use std::str::FromStr;

fn main() {
    // 数値から変換
    let n1 = Ncode::from(530947);
    // 文字列として出力
    println!("{}", n1);
    // Nコードとして符号化された文字列から変換
    let n2  = Ncode::from_str("N1000CB").unwrap();
    // 数値として出力
    println!("{}", n2.as_ref());
    // 表現が違っても同じ
    assert_eq!(n1, n2);
}
```
