use rand::Rng;

pub fn gen_ran() -> u8 {
  let mut rng = rand::thread_rng();
  let n: u8 = rng.gen();
  n
}
