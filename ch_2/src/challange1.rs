use std::collections::HashMap;

use rand::{rng, seq::SliceRandom};
fn main() {
  // we have made the random test case now let's rock on.
  let mut numbers: Vec<i32,> = vec![
    1, 2, 2, 4, 4, 9, 9, 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
  ];
  let mut ran = rng();
  numbers.shuffle(&mut ran,);

  // 1- sorting the vector.
  numbers.sort();


  let len = numbers.len();
  let median = if len % 2 == 1 {
    numbers[len / 2]
  } else {
    (numbers[len / 2 - 1] + numbers[len / 2]) / 2
  };

  println!("This is the median: {:?}", median);

  // 2- Get the mod of the vecor i will use the hashmap for this.
  let mut hashmap: HashMap<i32, i32,> = HashMap::new();

  for number in numbers {
    // Here i check if the number exist so i can increment it or set it to 0 if it's the
    // firsttime..
    let count = hashmap.entry(number,).or_insert(0,);
    *count += 1;
  }

  let max_number = hashmap.values().max().unwrap_or(&0,);
  let mod_of_hashmap: Vec<i32,> = hashmap
    .iter()
    .filter_map(|(&number, &value,)| {
      if value == *max_number {
        Some(number,)
      } else {
        None
      }
    },)
    .collect();

  println!("this is the mod {:?}", mod_of_hashmap);
}
