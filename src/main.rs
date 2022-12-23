use rand;
fn main() {
    let mut array = [0; 100];
    for i in 0..100 {
        array[i] = rand::random::<u32>() % 100;
    }

    let min = *array.iter().min().unwrap();
    let max = *array.iter().max().unwrap();

    let mut counts = vec![0; (max - min) as usize + 1];

    for value in array {
        counts[(value - min) as usize] += 1;
    }

    let sorted_arr: Vec<_> = counts
        .iter()
        .enumerate()
        .flat_map(|(i, &count)| std::iter::repeat(i as u32 + min).take(count))
        .collect();

    println!("{:?}", sorted_arr);
}
