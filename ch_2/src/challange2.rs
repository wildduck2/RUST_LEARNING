// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!


fn main() {
  let word = String::from("first",);
  let chars = word.chars().collect::<Vec<char,>>();

  let final_string = format!(
    "{}-{}{}",
    chars[1..].iter().collect::<String>(),
    chars[0],
    "ay"
  );

  println!("{:?}", final_string);
}
