use criterion::{black_box, criterion_group, criterion_main, Criterion};
use discreet_math_fib::main_old::{fib, FibContents, LIMIT};
use num_bigint::BigUint;

fn fib2(n: FibContents) -> Vec<BigUint> {
    let mut vec = vec![BigUint::from(1_u32), BigUint::from(1_u32)]; // initialize FibList vector with 1,1
    for i in 2..n {
        let new_number = vec.get(i as usize - 1).unwrap() + vec.get(i as usize - 2).unwrap(); // calculate new number to add to fibonacci list, cant fail because vec length is always >= 2

        vec.push(new_number);
    }
    // 1,1,2,3,5
    vec
}

fn fib3(nth: usize) -> (BigUint, BigUint) {
    // Break recursion if fib reaches zero
    if nth == 0 {
        return (0u8.into(), 1u8.into());
    }
    // Would number divide evenly by 2?
    let modulo_rem = nth % 2;
    // Subtract nth by modulo rem and divide by 2 to do floor division
    let (a, b): (BigUint, BigUint) = fib3((nth - modulo_rem) / 2);

    // Algorithm...
    let c = &a * (&b * 2u8 - &a);
    let d = &a * &a + &b * &b;

    if modulo_rem == 1 {
        let summed = c + &d;
        (d, summed)
    } else {
        (c, d)
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 1000, limit ", |b| {
        b.iter(|| {
            // fibonacci(black_box(20));
            fib(black_box(1000), black_box(LIMIT));
        })
    });

    c.bench_function("fib2 500", |b| {
        b.iter(|| {
            fib2(black_box(500));
        })
    });

    c.bench_function("fib3 500", |b| {
        b.iter(|| {
            fib3(black_box(500));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
