use std::collections::HashMap;

fn main() {
  // NOTE: the the `hashmap` in rust uses the `SipHash` designed
  // by "Jean-Philippe Aumasson" and "Daniel J. Bernstein" in 2012.
  let mut hashmap: HashMap<String, i32,> = HashMap::new();
  // Setting a key in the `HashMap`
  hashmap.insert("key".to_string(), 1,);
  // Getting a value from the `HashMap`
  // using the `copied()` method we have dreferenced the value and got a copy
  // instead of the reference so the type will be like
  // this `Option<i32>` instead of `Option<&i32>`
  // and also we use the `unwrap_or` method to set a default value
  hashmap.get("key",).copied().unwrap_or(0,);

  // Re assigning a `HashMap` with the same key and the same value
  hashmap.insert("key".to_string(), 2,);
  hashmap.insert("key2".to_string(), 2,);
  hashmap.insert("key1".to_string(), 2,);

  // This has logged a value of `Some(2)`
  println!("{:?}", hashmap.get("key"));

  // Another techniech is to check if the key already exist if
  // yes insert a key with value if not just stop.
  hashmap.entry(String::from("key",),).or_insert(76,);

  // This will long only the `Some(2)` hence the value has't changed at all
  println!("{:?}", hashmap.get("key"));

  // you can get the keys of the hashmap by the keys :LAMO i already know.
  let keys = hashmap.keys();
  println!("{:?}", keys);


  let text = "hello world wonderful world";

  let mut map = HashMap::new();

  for word in text.split_whitespace() {
    let count = map.entry(word,).or_insert(0,);
    *count += 1;
  }

  println!("{map:?}");
}
