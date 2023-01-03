use std::time::{Duration, Instant};
use std::collections::HashMap;

// Following functions for calculating the average, median and mode
// adapted from: https://codereview.stackexchange.com/a/173437

fn average(durations: &[Duration]) -> Duration {
    let sum = durations.iter().sum::<Duration>();
    Duration::from_nanos(sum.as_nanos() as u64 / durations.len() as u64)
}

fn median(durations: &mut [Duration]) -> Duration {
    durations.sort();
    let middle = durations.len() / 2;
    durations[middle]
}

fn mode(durations: &[Duration]) -> Duration {
    let mut counts = HashMap::new();

    durations.iter().copied().max_by_key(|&n| {
        let count = counts.entry(n).or_insert(0);
        *count += 1;
        *count
    }).unwrap()
}

fn get_last1(vec: &Vec<usize>) -> usize {
    vec[vec.len()-1]
}

fn get_last2(vec: &Vec<usize>) -> usize {
    *vec.last().unwrap()
}

fn main() {
    let mut vec = vec![];
    let size = std::env::args()
        .nth(1)
        .unwrap_or("10000".to_string())
        .parse::<usize>()
        .unwrap();
    let iters = std::env::args()
        .nth(2)
        .unwrap_or("100000".to_string())
        .parse::<usize>()
        .unwrap();

    println!("Running benchmark with size {} for {} iterations.", size, iters);

    for i in 0..size {
        vec.push(i);
    }

    let mut durations1 = Vec::with_capacity(100000);
    let mut durations2 = Vec::with_capacity(100000);

    for _ in 0..iters {
        let start = Instant::now();
        let _last1 = get_last1(&vec);
        durations1.push(start.elapsed());
    }

    for _ in 0..iters {
        let start = Instant::now();
        let _last2 = get_last2(&vec);
        durations2.push(start.elapsed());
    }

    let avg1 = average(&durations1);
    let avg2 = average(&durations2);
    let median1 = median(&mut durations1);
    let median2 = median(&mut durations2);
    let mode1 = mode(&durations1);
    let mode2 = mode(&durations2);

    println!("Average duration of ' vec[vec.len()-1] '     : {:?}", avg1);
    println!("Median duration of  ' vec[vec.len()-1] '     : {:?}", median1);
    println!("Mode duration of    ' vec[vec.len()-1] '     : {:?}", mode1);
    println!("Average duration of ' *vec.last().unwrap() ' : {:?}", avg2);
    println!("Median duration of  ' *vec.last().unwrap() ' : {:?}", median2);
    println!("Mode duration of    ' *vec.last().unwrap() ' : {:?}", mode2);
}
