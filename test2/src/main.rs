
fn sum(v: &[u32]){
  let mut j = 0;
  for i in v {
      j+=i;
  }
  println!("{}", j);
}

fn main() {
  let v: [u32; 5] = [1, 2, 3, 4, 5];
  let _test = sum(&v);
}
