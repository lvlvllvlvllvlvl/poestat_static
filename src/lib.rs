mod models;
pub use crate::models::*;

pub static TRIE: &Trie = include!(concat!(env!("OUT_DIR"), "/trie.rs"));
pub static TOKENS: phf::Map<&'static str, &'static [Token]> =
    include!(concat!(env!("OUT_DIR"), "/tokens.rs"));
pub static IMPLIED: phf::Map<&'static str, phf::Map<&'static str, i32>> =
    include!(concat!(env!("OUT_DIR"), "/implied.rs"));
pub static HANDLERS: phf::Map<&'static str, Handler> =
    include!(concat!(env!("OUT_DIR"), "/handlers.rs"));
