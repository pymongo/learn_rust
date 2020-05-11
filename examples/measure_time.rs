fn main() {
  let now = std::time::Instant::now();

  std::thread::sleep(std::time::Duration::from_millis(200));

  println!("{:?}", now.elapsed());
}