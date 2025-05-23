// use std::fmt::Display;
//
// trait List<T: std::cmp::PartialOrd,> {
//   fn get_greater(&self,) -> &T;
// }
//
// impl<T: std::cmp::PartialOrd,> List<T,> for Vec<T,> {
//   fn get_greater(&self,) -> &T {
//     let mut largest = &self[0];
//
//     for item in self {
//       if item > largest {
//         largest = item;
//       }
//     }
//
//     largest
//   }
// }
//
// pub trait Print {
//   fn print(&self,) -> String;
// }
// trait Scope {
//   fn get_scope(&self,) -> String;
// }
//
// impl Print for String {
//   fn print(&self,) -> String {
//     self.to_string()
//   }
// }
//
// pub fn some_function<T, U,>(_t: &T, _u: &U,) -> i32
// where
//   T: Display + Clone,
//   U: Clone + Print,
// {
//   0
// }
//

// fn main() {
//   let s = String::from("hello",);
//   println!("{}", s.print());
//
//
//   let list = vec![1, 2, 3, 4, 5, 6, 7, 8];
//   let largest = List::get_greater(&list,);
//   println!("{:?} is the largest number", largest);
//
//   let list = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
//   let largest = List::get_greater(&list,);
//   println!("{:?} is the largest char", largest);
// }

// NOTE: We introduced explicit life-times to manage borrowing correctly.
// When returning a reference that could point to one of multiple input references,
// the compiler needs to know how their life-times relate.
// Without specifying life-times, Rust cannot guarantee the returned reference
// will be valid after the function returns, leading to errors.
// By adding the life-time parameter `'a` to the input references and the return type,
// we tell the compiler that the returned reference will live at least as long as the inputs,
// preventing dangling references and ensuring memory safety.
//
// `**TL;DR**` The returned reference will be valid as long as both the parameters are valid.
fn longest<'a>(x: &'a String, y: &'a String) -> &'a String {
  if x.len() > y.len() {
    return x;
  }
  y
}

fn main() {
  let first_name = String::from("welcome to the paradie!!");
  let second_name = String::from("welcome to the paradie!!2");

  let result = longest(&first_name, &second_name);

  println!("[[ \"{}\" ]] is the longest string", result);

  let string1 = String::from("long string is long");

  {
    let string2 = String::from("xyz");
    let result = longest(&string1, &string2);
    println!("The longest string is {result}");
  }

  let first_w = first_word("hi from paradice");
  println!(
    "this is the first word in the string [[ \"hi from paradice\" ]], {} ",
    first_w
  );
}

/// NOTE: There's also a concept called `lifetime elision`.
/// It's a set of predefined rules in Rust that allow you to omit explicit
/// lifetime annotations in common scenarios. This feature exists mainly
/// for historical reasons—back in the pre-1.0 days of Rust, you had to
/// explicitly write lifetimes even in simple cases like this.
/// Over time, the Rust team noticed that developers were repeating
/// the same lifetime patterns in almost every function, so they decided
/// to build those patterns directly into the language.
fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}
