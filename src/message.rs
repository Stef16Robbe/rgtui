use core::time;
use serde_derive::Deserialize;

// From: https://github.com/BurntSushi/ripgrep/blob/master/tests/json.rs
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum Message {
    Begin(Begin),
    End(End),
    Match(Match),
    Context(Context),
    Summary(Summary),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Begin {
    pub path: Option<Data>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct End {
    pub path: Option<Data>,
    pub binary_offset: Option<u64>,
    pub stats: Stats,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Summary {
    pub elapsed_total: Duration,
    pub stats: Stats,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Match {
    pub path: Option<Data>,
    pub lines: Data,
    pub line_number: Option<u64>,
    pub absolute_offset: u64,
    pub submatches: Vec<SubMatch>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Context {
    pub path: Option<Data>,
    pub lines: Data,
    pub line_number: Option<u64>,
    pub absolute_offset: u64,
    pub submatches: Vec<SubMatch>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct SubMatch {
    #[serde(rename = "match")]
    pub m: Data,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Data {
    Text { text: String },
    // This variant is used when the data isn't valid UTF-8. The bytes are
    // base64 encoded, so using a String here is OK.
    Bytes { bytes: String },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Stats {
    pub elapsed: Duration,
    pub searches: u64,
    pub searches_with_match: u64,
    pub bytes_searched: u64,
    pub bytes_printed: u64,
    pub matched_lines: u64,
    pub matches: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Duration {
    #[serde(flatten)]
    pub duration: time::Duration,
    pub human: String,
}
