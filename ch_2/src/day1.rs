use std::collections::{hash_map, HashMap};

pub fn log() {
  let mut hashmap: HashMap<String, i32,> = HashMap::new();
  // Setting a key in the `HashMap`
  hashmap.insert("key".to_string(), 1,);
  // Getting a value from the `HashMap`
  // using the `copied()` method we have dreferenced the value and got a copy
  // instead of the reference so the type will be like
  // this `Option<i32>` instead of `Option<&i32>`
  // and also we use the `unwrap_or` method to set a default value
  hashmap.get("key",).copied().unwrap_or(0,);

  // Re assigning a hashmap with the same key and the same value
  hashmap.insert("key".to_string(), 2,);


  println!("{:?}", hashmap.get("key"));
}
