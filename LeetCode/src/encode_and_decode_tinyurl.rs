use std::{
  cell::{Cell, RefCell},
  collections::HashMap,
};

pub struct Codec {
  store: RefCell<HashMap<String, String>>,
  counter: Cell<u64>,
  alpha: Vec<char>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
  pub fn new() -> Self {
    let alpha: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
      .chars()
      .collect();

    Codec {
      alpha: alpha,
      store: RefCell::new(HashMap::new()),
      counter: Cell::new(1),
    }
  }

  // Encodes a URL to a shortened URL.
  pub fn encode(&self, long_url: String) -> String {
    let mut str = String::from("");
    let mut n = self.counter.get();
    self.counter.set(n + 1);

    while n > 0 {
      str = String::from(format!(
        "{}{}",
        str,
        self.alpha[n as usize % self.alpha.len()].to_string()
      ));

      n -= 1;
    }

    self
      .store
      .borrow_mut()
      .entry(str.clone())
      .or_insert(long_url);

    str
  }

  // Decodes a shortened URL to its original URL.
  pub fn decode(&self, short_url: String) -> String {
    match short_url.strip_prefix("") {
      Some(value) => {
        return self.store.borrow().get(value).cloned().unwrap_or(short_url);
      }
      _ => String::from(""),
    }
  }
}

/*
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let s: String = obj.encode(strs);
 * let ans: VecVec<String> = obj.decode(s);
 */

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_encoding_and_decoding() {
    let codec = Codec::new();

    let original = "https://example.com".to_string();
    let short = codec.encode(original.clone());
    let decoded = codec.decode(short);

    assert_eq!(decoded, original);
  }

  #[test]
  fn test_multiple_urls() {
    let codec = Codec::new();

    let url1 = "https://example.com/page1".to_string();
    let url2 = "https://example.com/page2".to_string();

    let short1 = codec.encode(url1.clone());
    let short2 = codec.encode(url2.clone());

    assert_ne!(short1, short2);
    assert_eq!(codec.decode(short1), url1);
    assert_eq!(codec.decode(short2), url2);
  }

  #[test]
  fn test_repeat_encoding_same_url() {
    let codec = Codec::new();

    let url = "https://repeat.com".to_string();

    let short1 = codec.encode(url.clone());
    let short2 = codec.encode(url.clone());

    assert_ne!(short1, short2); // different short URLs
    assert_eq!(codec.decode(short1), url);
    assert_eq!(codec.decode(short2), url);
  }

  #[test]
  fn test_nonexistent_short_url() {
    let codec = Codec::new();
    let fake_short = "http://tinyurl.com/999".to_string();
    let decoded = codec.decode(fake_short.clone());

    // Assuming the decode returns the input if not found
    // (update this if your logic differs)
    assert_eq!(decoded, fake_short);
  }
}
