// fn main() {
//     let mult_x = |x| x * 2;
//
//     for i in 0..=10 {
//         println!("{}", mult_x(i));
//     }
// }
//

fn main() {
    let r;
    {
        let n = 0;
        r = &n;
    }
    println!("r: {}", r);
}
