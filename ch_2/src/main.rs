fn main() {
  let list = vec![1, 2, 3, 4, 5, 6, 7, 8];
  let largest = get_greater(&list,);
  println!("{:?} is the largest number", largest);

  let list = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
  let largest = get_greater(&list,);
  println!("{:?} is the largest char", largest);
}


fn get_greater<T: std::cmp::PartialOrd,>(list: &[T],) -> &T {
  let mut largest = &list[0];

  for item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}
