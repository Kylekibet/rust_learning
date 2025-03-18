fn main() {
    nth_fib(1000);
}

fn nth_fib(y: usize) {
    let mut z: usize;
    let mut x = 0;
    let mut i = 1;
    loop {
        print!("{x}, ");
        z = x + i;
        x = i;
        i = z;
        if z > y {
            break;
        }
    }
}
