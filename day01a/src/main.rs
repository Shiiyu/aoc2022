fn main() {
  println!(
    "Highest Amount of Calories Carried: {}",
    include_str!("../input.txt")
      .split("\n\n")
      .map(|e| e.lines().map(|n| n.parse::<u32>().unwrap()).sum::<u32>())
      .max()
      .unwrap()
  )
}
