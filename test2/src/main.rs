
fn sum(v: &[u32])-> (u32, &str){
  let mut j = 0;
  for i in v {
      j+=i;
  }
  if j >0 {
    return (j, "ok");
  } else {
    return (j, "none");
  }
}

fn main() {
  let v: [u32; 5] = [1, 2, 3, 4, 5];
  let _test = sum(&v);
  println!("{:?}", _test);
}
