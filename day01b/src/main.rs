fn main() {
  let mut v: Vec<u32> = include_str!("../input.txt")
    .split("\n\n")
    .map(|e| e.lines().map(|n| n.parse::<u32>().unwrap()).sum::<u32>())
    .collect();

  v.sort_unstable_by(|a, b| b.cmp(a));

  println!("Sum of Largest 3 Calories: {}", v[0] + v[1] + v[2])
}
