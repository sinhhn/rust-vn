fn main() {
    fizzbuzz_to(1000);
    format_print();
}
fn is_devided_by(lhs: u32, rhs: u32) -> bool {
     if rhs == 0 {
         return false;
     }
     lhs % rhs == 0
     
}


fn fizzbuzz(n: u32) {
    if is_devided_by(n, 15) {
        println!("fizzbuzz");
    } else if is_devided_by(n, 3) {
        println!("fizz");
    } else {
        println!("buzz");
    }
}

fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        fizzbuzz(n);
    }
}

fn format_print() {
    println!("{} days in {}", 31, "January");
}
