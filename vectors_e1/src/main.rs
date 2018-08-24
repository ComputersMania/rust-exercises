use std::collections::HashMap;
use std::cmp::Ordering;

fn main() {
    let list: Vec<usize> = vec![28, 18, 9, 7, 115, 7, 674, 89, 115, 115];
    println!("{:?}", list);
    println!("The mean of this list is {}", mean(&list[..]));
    println!("The median of this list is {}", median(&list[..]));
    println!("The mode is {}", mode(&list[..]));
}

fn mean(list: &[usize]) -> usize {
    let mut sum = 0_usize;

    for &item in list {
        sum += item;
    }
    sum / list.len()
}

fn median(list: &[usize]) -> usize {
    let mut clone: Vec<usize> = Vec::new();
    clone.extend_from_slice(list);

    let sorted = &mut clone[..];
    sorted.sort();

    let len = sorted.len();

    if len % 2 == 1 {
        sorted[len/2+1 -1]
    } else {
        (sorted[len/2 -1] + sorted[len/2+1 -1]) /2
    }
}

fn mode(list: &[usize]) -> usize {
    let mut map: HashMap<usize, usize> = HashMap::new();

    for &item in list {
        let mut entry = map.entry(item).or_insert(0);
        *entry += 1;
    };

    let mut candidates: Vec<usize> = Vec::new();
    let mut max_instances = 0usize;
    for (&value, &instances) in &map {
        match instances.cmp(&max_instances) {
            Ordering::Less => continue,
            Ordering::Equal => candidates.push(value),
            Ordering::Greater => {
                candidates.clear();
                max_instances = instances;
                candidates.push(value);
            },
        };
    };

    mean(&candidates[..])
    // println!("{:?}", map);
}
