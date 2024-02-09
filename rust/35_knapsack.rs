#[derive(Debug, Clone)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let n = items.len();

    let mut dp_table = vec![vec![0; (max_weight + 1) as usize]; n + 1];

    for i in 1..=n {
        for j in 1..=max_weight {
            if items[i - 1].weight <= j {
                dp_table[i][j as usize] = dp_table[i - 1][j as usize]
                    .max(dp_table[i - 1][(j - items[i - 1].weight) as usize] + items[i - 1].value);
            } else {
                dp_table[i][j as usize] = dp_table[i - 1][j as usize];
            }
        }
    }

    dp_table[n][max_weight as usize]
}
