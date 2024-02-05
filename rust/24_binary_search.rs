pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let mut low: usize = 0;
    let mut high: usize = array.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        if array[mid] == key {
            return Some(mid);
        }

        if array[mid] < key {
            low = mid + 1;
        } else {
            // Check if mid is 0 to avoid underflow
            if mid == 0 {
                break;
            }

            high = mid - 1;
        }
    }

    None
}
