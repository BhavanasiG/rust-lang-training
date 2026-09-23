use std::collections::HashMap;

fn main() {
    // Example list of numbers:
    let mut num_list = vec![-1, -5, 10, 1, 2, 3, 3, 3, 10, 10, -1 , 1, 21, 67];

    num_list.sort();

    // Find and print the median value
    let median: f64;

    if num_list.len() % 2 != 0 {
        median = num_list[num_list.len()/2] as f64;
    } else {
        let before_median_position: i32 = num_list[num_list.len() / 2 - 1];
        let after_median_position: i32 = num_list[num_list.len() / 2];

        median = (before_median_position + after_median_position) as f64 / 2.0;
    }

    println!("The median value of the list is: {median}");

    // Find and print the mode value
    let mut mode_of_ints = HashMap::new();

    let mut mode_num:i32 = 0; // Placeholder value
    let mut mode_num_occurances: i32 = 0;

    for num in num_list {
        let count = mode_of_ints.entry(num).or_insert(0);
        *count += 1;

        if *count > mode_num_occurances {
            mode_num = num;
            mode_num_occurances = *count;
        }
    }

    println!("The mode integer is {mode_num} and the number of occurance it made is {mode_num_occurances}");
}
