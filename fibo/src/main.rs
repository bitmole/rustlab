use std::io;

fn main() {
    println!("Please enter the sequence size:");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read n.");

    let n: u32 = n.trim().parse().expect("Input number!");

    let sequence = fibo(n);

    for (i, result) in sequence.iter().enumerate() {
        println!("fibo({}) -> {}", i, result);
    }

    println!("Recursive:");
    println!("fibo({}) -> {}", n, fibo_recur(n));
}

fn fibo(n: u32) -> Vec<u32> {
    let mut results = Vec::new();
    let mut a = 0;
    let mut b = 1;

    for i in 0..n {
        if i < 2 { 
            results.push(i);
        } else {
            let temp = a + b;
            a = b;
            b = temp;
            results.push(b);
        }
    }

    results
}

// TODO: implement tail recursion
fn fibo_recur(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fibo_recur(n - 1) + fibo_recur(n - 2)
    }
}
