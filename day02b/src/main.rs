fn main() {
  println!(
    "Strategy Guide Score: {}",
    include_bytes!("../input.txt")
      .split(|&b| b == b'\n')
      .map(|l| ((l[0] - b'A') as u16, (l[2] - b'X') as u16))
      .map(|(a, b)| (2 + a + b) % 3 + (b * 3) + 1)
      .sum::<u16>()
  )
}
