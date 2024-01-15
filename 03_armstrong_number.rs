pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let len_of_num = num_str.len() as u32;

    let sum_of_num: u64 = num_str
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| (d as u64).pow(len_of_num))
        .sum();

    sum_of_num == num as u64
}
