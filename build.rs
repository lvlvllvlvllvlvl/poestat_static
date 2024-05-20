use std::{collections::HashMap, env, fs::File, iter::FromIterator, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=data/trie.json");
    println!("cargo:rerun-if-changed=data/stats.json");
    println!("cargo:rerun-if-changed=data/handlers.json");
    let file = File::open("data/trie.json").unwrap();
    let trie: Trie = serde_json::from_reader(file).unwrap();
    let file = File::open("data/tokens.json").unwrap();
    let tokens: HashMap<String, Vec<TokenIn>> = serde_json::from_reader(file).unwrap();
    let tokens: HashMap<String, Vec<Token>> =
        HashMap::from_iter(tokens.into_iter().map(|(k, v)| {
            (
                k.clone(),
                v.iter()
                    .map(|t| match t {
                        TokenIn::Literal { value } => Token::Literal {
                            value: value.into(),
                        },
                        TokenIn::Number {
                            index,
                            stat,
                            stat_value_handlers,
                        } => Token::Number {
                            index: *index,
                            stat: stat.into(),
                            stat_value_handlers: stat_value_handlers.clone(),
                        },
                        TokenIn::Enum {
                            index,
                            stat,
                            stat_value_handler,
                        } => Token::Enum {
                            index: *index,
                            stat: stat.into(),
                            stat_value_handler: stat_value_handler.into(),
                        },
                        TokenIn::NestedStat { added_stat } => Token::NestedStat {
                            added_stat: added_stat.into(),
                        },
                        TokenIn::Unknown => Token::Unknown,
                    })
                    .collect(),
            )
        }));
    let file = File::open("data/implied.json").unwrap();
    let implied: HashMap<String, HashMap<String, i32>> = serde_json::from_reader(file).unwrap();
    let file = File::open("data/handlers.json").unwrap();
    let handlers: HashMap<String, Handler> = serde_json::from_reader(file).unwrap();

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("trie.rs");
    let mut uneval = uneval_static::ser::Uneval::new(File::create(path).unwrap());
    uneval.add_mapping("Trie".into(), "&Trie".into());
    uneval.serialize(trie).expect("Write failed");

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("tokens.rs");
    let mut uneval = uneval_static::ser::Uneval::new(File::create(path).unwrap());
    uneval.serialize(tokens).expect("Write failed");

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("implied.rs");
    let mut uneval = uneval_static::ser::Uneval::new(File::create(path).unwrap());
    uneval.serialize(implied).expect("Write failed");

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("handlers.rs");
    let mut uneval = uneval_static::ser::Uneval::new(File::create(path).unwrap());
    uneval.serialize(handlers).expect("Write failed");
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
struct Trie {
    #[serde(default)]
    child_map: HashMap<String, Trie>,
    num_child: Option<Box<Trie>>,
    any_child: Option<Box<Trie>>,
    stat_child: Option<Box<Trie>>,
    stat_value: Option<i32>,
    stat_id: Option<String>,
    terminal: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
struct Handler {
    #[serde(default = "add_id")]
    addend: f32,
    #[serde(default = "mul_id")]
    divisor: f32,
    #[serde(default = "mul_id")]
    multiplier: f32,
}

fn add_id() -> f32 {
    0.0
}
fn mul_id() -> f32 {
    1.0
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
enum TokenIn {
    #[serde(rename = "literal")]
    Literal { value: String },
    #[serde(rename = "number")]
    Number {
        index: i32,
        stat: String,
        #[serde(default)]
        stat_value_handlers: Vec<String>,
    },
    #[serde(rename = "enum")]
    Enum {
        index: i32,
        stat: String,
        stat_value_handler: String,
    },
    #[serde(rename = "nested")]
    NestedStat { added_stat: String },
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize)]
enum Token {
    Literal {
        value: String,
    },
    Number {
        index: i32,
        stat: String,
        #[serde(default)]
        stat_value_handlers: Vec<String>,
    },
    Enum {
        index: i32,
        stat: String,
        stat_value_handler: String,
    },
    NestedStat {
        added_stat: String,
    },
    Unknown,
}
