fn main() {
  println!(
    "Strategy Guide Score: {}",
    include_bytes!("../input.txt")
      .split(|&b| b == b'\n')
      .map(|l| ((l[0] - b'A') as u16, (l[2] - b'X') as u16))
      .map(|(a, b)| match (3 + b - a) % 3 {
        0 => 4,
        1 => 7,
        2 => 1,
        _ => unreachable!()
      } + b)
      .sum::<u16>()
  )
}
