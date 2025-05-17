fn main() {
  let hello = "Здравствуйте";

  // Collect characters into a Vec to index them
  let chars: Vec<char,> = hello.chars().collect();

  // Get the second character (index 1)
  let x = chars[1];

  println!("{}", x); // will print 'д'
                     // println!("{}", hello[1]); // will panic
}
