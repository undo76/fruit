#[inline(always)]
fn equation(a: u64, b: u64 , c: u64) -> bool {
    a*a + b*b + 5 == c*c*c + a*b*c
}

fn main() {
    let mut result = None;
    'outer:
    for a in 1..=5000 {
        for b in 1..=5000 { 
            for c in 1..=5000 {
                if equation(a, b, c) {
                    result = Some((a, b, c));
                    break 'outer;
                }
            } 
        }
    };
   
    println!("{:?}", &result);
}