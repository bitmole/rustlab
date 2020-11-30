use std::io;

fn main() {
    println!("Please enter the sequence size:");

    let mut size = String::new();

    io::stdin()
        .read_line(&mut size)
        .expect("Failed to read size.");

    let size: u32 = size.trim().parse().expect("Input number!");

    println!("Generating sequence of {} fibonacci numbers:", size);

    fibo(size);
}

// TODO: Let fibo generate a list; optimize
fn fibo(n: u32) {
    let mut a = 0;
    let mut b = 1;

    for i in 0..n {
        if i == 0 {
            println!("{}", a);
        } else if i == 1 {
            println!("{}", b);
        } else {
            let temp = a + b;
            a = b;
            b = temp;
            println!("{}", temp);
        }
    }
}
