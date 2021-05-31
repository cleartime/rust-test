

fn sum(x:&[u32])->Option<u32> {
    let opt = Option::Some(x);
    match opt {
        Option::Some(something) =>{
            let mut j;
            for i in something.iter() {
                println!("i = {}", i)
            }
            return j
        },
        Option::None => {
            return None
        }
    }
}

fn main() {
    let a:[u32] = [2123];
    let test = sum(a);
    println!("sum = {}", test)
}
