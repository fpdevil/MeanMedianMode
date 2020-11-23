use std::collections::HashMap;
use std::io::Write;
use std::str::FromStr;

fn main() {
    println!("* Statistical Calculation *");
    println!(">> Enter list of integers for Statistical calculation!");

    let mut ints = Vec::new();

    for arg in std::env::args().skip(1) {
        ints.push(i32::from_str(&arg)
            .expect("error parsing the argument!"));
    }

    if ints.len() == 0 {
        writeln!(std::io::stderr(), "Usage: meanmedianmode NUMBERS...").unwrap();
        std::process::exit(1);
    }

    let mean = match mean(&ints) {
        Some(x) => x.to_string(),
        None => "Empty List".to_string(),
    };
    println!("Mean: {}", mean);

    //
    // Testing the local copy of sort
    // let sorted_ints = &mut ints.clone();
    // sorted_ints.sort();
    // println!("sorted {:?}", sorted_ints);
    // println!("original {:?}", ints);

    let median = match median(&ints) {
        Some(x) => x.to_string(),
        None => "Empty List".to_string(),
    };
    println!("Median: {}", median);

    let mode = mode(&ints);
    println!("Mode: {:?}", mode);
}

fn mean(ints: &[i32]) -> Option<f32> {
    if ints.len() == 0 {
        return None;
    }

    let s = sum(ints) as f32;
    let len = ints.len() as f32;
    Some(s / len)
}

fn sum(ints: &[i32]) -> i32 {
    let mut s = 0;
    for i in ints {
        s += *i;
    }
    s
}

fn median(ints: &[i32]) -> Option<f32> {
    let mut sorted_ints = ints.to_vec();
    sorted_ints.sort();
    match ints.len() {
        0 => None,
        1 => Some(sorted_ints[0] as f32),
        v if v % 2 == 0 => mean(&vec![sorted_ints[v / 2 - 1], sorted_ints[v / 2]]),
        v => Some(sorted_ints[v / 2] as f32),
    }
}

// mode function calculates the statistical mode which is the
// value or values in the data set that occur most frequently.
fn mode(vals: &[i32]) -> Vec<f32> {
    let mut result: Vec<f32> = Vec::new();

    if vals.len() == 1 {
        result.push(vals[0] as f32);
    } else {
        let mut frequencies = HashMap::with_capacity(vals.len());
        for i in vals {
            let count = frequencies.entry(i).or_insert(0);
            *count += 1
        }
        // println!(
        //     "{:?} {:?}",
        //     &frequencies,
        //     &frequencies.values().max().unwrap()
        // );

        let max = &frequencies.values().max().unwrap();
        for (k, v) in &frequencies {
            if *v == **max {
                &result.push(**k as f32);
            }
        }
    }

    result
}
