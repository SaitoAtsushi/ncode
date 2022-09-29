//! 小説投稿サイト「[小説家になろう](https://syosetu.com/)」では各小説に番号が割り当てられています。
//! 番号は番号のままでで表される場合と[Nコード](https://syosetu.com/bbstopic/top/topicid/2733/)と呼ばれる符号で表される場合があります。
//! このライブラリはその変換を行います。
//! [`Ncode`](Ncode) は数値か文字列と相互変換可能な型です。
//! # サンプル
//! ```
//! use ncode::Ncode;
//! use std::str::FromStr;
//!
//! fn main() {
//!     // 数値から変換
//!     let n1 = Ncode::from(530947);
//!     // 文字列として出力
//!     println!("{}", n1);
//!     // Nコードとして符号化された文字列から変換
//!     let n2  = Ncode::from_str("N1000CB").unwrap();
//!     // 数値として出力
//!     println!("{}", n2.as_ref());
//!     // 表現が違っても同じ
//!     assert_eq!(n1, n2);
//! }
//! ```

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Ncode {
    value: u32,
}

impl AsRef<u32> for Ncode {
    fn as_ref(&self) -> &u32 {
        &self.value
    }
}

impl From<u32> for Ncode {
    fn from(x: u32) -> Self {
        Ncode { value: x }
    }
}

impl From<Ncode> for u32 {
    fn from(Ncode { value: x }: Ncode) -> Self {
        x
    }
}

use std::fmt::Display;
use std::fmt::Write;

impl Display for Ncode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut n = self.value;
        write!(f, "n{:04}", n % 9999)?;
        n /= 9999;

        let alphabet_part = std::iter::from_fn(move || {
            if n == 0 {
                None
            } else {
                let tmp = n % 26;
                n /= 26;
                Some(tmp)
            }
        })
        .collect::<Box<[u32]>>();

        alphabet_part.iter().rev().try_for_each(|&x| {
            f.write_char(
                char::from_u32(*b"abcdefghijklmnopqrstuvwxyz".get(x as usize).unwrap() as u32)
                    .unwrap(),
            )
        })
    }
}

impl From<Ncode> for String {
    fn from(x: Ncode) -> Self {
        x.to_string()
    }
}

use std::str::FromStr;

fn char_weight(ch: char) -> Option<u32> {
    Some(ch.to_digit(36)?.checked_sub(10)?)
}

#[derive(Debug)]
/// 文字列からの変換に失敗したときに返されるエラーの型です。
pub struct ParseNcodeError {}

impl Display for ParseNcodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseNcodeError")
    }
}

impl std::error::Error for ParseNcodeError {}

impl FromStr for Ncode {
    type Err = ParseNcodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut itr = s.chars().peekable();
        match itr.next().ok_or(ParseNcodeError {})? {
            'n' | 'N' => Some(()),
            _ => None,
        }
        .ok_or(ParseNcodeError {})?;
        let mut number_part: u32 = 0;
        for _ in 0..4 {
            number_part *= 10;
            number_part += itr
                .next()
                .ok_or(ParseNcodeError {})?
                .to_digit(10)
                .ok_or(ParseNcodeError {})?;
        }
        let mut alphabetic_part: u32 = 0;
        let _ = itr.try_for_each(|x| {
            alphabetic_part *= 26;
            alphabetic_part += char_weight(x).ok_or(ParseNcodeError {})?;
            Ok(())
        })?;
        Ok(Ncode::from(number_part + alphabetic_part * 9999))
    }
}

#[cfg(test)]
mod tests {
    use super::Ncode;

    #[test]
    fn it_works() {
        let n = Ncode::from(530947u32);
        assert_eq!(n.as_ref(), &530947u32);
        assert_eq!(n.to_string(), "n1000cb");

        let n: Ncode = "n1000cb".parse().unwrap();
        assert_eq!(n.as_ref(), &530947u32);
        assert_eq!(n.to_string(), "n1000cb");

        let n: Ncode = "N1000CB".parse().unwrap();
        assert_eq!(n.as_ref(), &530947u32);
        assert_eq!(n.to_string(), "n1000cb");
    }
}
