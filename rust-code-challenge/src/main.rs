fn main() {
    let list = vec![1.0, 4.0, 5.0];
    assert_eq!(median(list), Some(4.0))
    medium(list)
}

// Calculate median value 
fn median(mut a: Vec<f32>) -> Option<f32> {
    // - empty
    if a.is_empty() {
        return None;
    }

    // - sort a
    a.sort_by(|x: &f32, y: &f32| x.partial_cmp(y).unwrap());

    let number_of_elements: usize = a.len();
    let middle: usize = number_of_elements / 2;

    let med: f32 = if number_of_elements % 2 == 0 {
        // - even number of elements
        (a[middle - 1] + a[middle]) / 2.0
    } else {
        // - odd number of elements
        a[middle]
    };

    Some(med);
}
