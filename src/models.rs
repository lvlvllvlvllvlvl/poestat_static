pub struct Trie {
    pub child_map: phf::Map<&'static str, &'static Trie>,
    pub num_child: Option<&'static Trie>,
    pub any_child: Option<&'static Trie>,
    pub stat_child: Option<&'static Trie>,
    pub stat_value: Option<i32>,
    pub stat_id: Option<&'static str>,
    pub terminal: Option<&'static str>,
}

pub struct Handler {
    pub addend: f32,
    pub divisor: f32,
    pub multiplier: f32,
}

impl Handler {
    pub fn reverse(&self, n: f32) -> f32 {
        ((n - self.addend) * self.divisor) / self.multiplier
    }
}

pub enum Token {
    Literal {
        value: &'static str,
    },
    Number {
        index: i32,
        stat: &'static str,
        stat_value_handlers: &'static [&'static str],
    },
    Enum {
        index: i32,
        stat: &'static str,
        stat_value_handler: &'static str,
    },
    NestedStat {
        added_stat: &'static str,
    },
    Unknown,
}
