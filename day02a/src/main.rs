fn main() {
  println!(
    "Strategy Guide Score: {}",
    include_bytes!("../input.txt")
      .split(|&b| b == b'\n')
      .map(|l| ((l[0] - b'A') as u16, (l[2] - b'X') as u16))
      .map(|(a, b)| (4 + b - a) % 3 * 3 + (b + 1))
      .sum::<u16>()
  )
}
