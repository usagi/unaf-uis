// build.rs 開発用。 lib.rs から読み込まれる事で build.rs にも rust-analyzer などが通る。
// 有効なままでも最終的には使用されない private として最適化されて消えるので問題ない
include!("build.rs");

use once_cell::sync::Lazy;
use std::collections::HashMap;

pub type KeyType = &'static str;
pub type ValueType = &'static [u8];
pub type DictionaryType = HashMap<KeyType, ValueType>;
pub type LazyDictionaryType = Lazy<DictionaryType>;

/// uis のフレーム構造
// pub const LIB_RS: &[u8] = include_bytes!("lib.rs");
/// 基本的に main ビルドプリプロセスで更新される .build/resource.includable.rs を
/// 元にした HashMap を保持する。必要に応じて手動でリソースを追加しても構わない。
pub static RESOURCE: LazyDictionaryType = LazyDictionaryType::new(|| include!("resource.includable.rs"));

#[test]
fn __()
{
 use crate::RESOURCE;
 println!("len={}", RESOURCE.len());
 for (&key, &value) in RESOURCE.iter()
 {
  let top20 = &value[..20.min(value.len())];
  println!("{key}; {bytes:?} => {top20:?}", bytes = value.len());
 }
}
