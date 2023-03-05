use std::collections::HashMap;

pub fn median(v: &Vec<i32>) -> Result<i32, &str> {
    if v.len() == 0 {
        return Err("Empty array");
    }

    let mut sorted_vec = v.clone();
    sorted_vec.sort();
    let middle_pos = ((sorted_vec.len() as f32) / 2.0).floor() as usize;

    Ok(sorted_vec[middle_pos])
}

pub fn mode(v: &Vec<i32>) -> Result<i32, &str> {
    if v.len() == 0 {
        return Err("Empty array.")
    }

    let mut counter_map = HashMap::new();
    for element in v {
        let counter = counter_map.entry(element).or_insert(0);
        *counter += 1;
    }

    let mut max = 0;
    let mut mode = Ok(0);
    for (key, value) in counter_map {
        if value > max {
            mode = Ok(*key);
            max = value;
        }
    }

    mode
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_1() {
        let v = vec![1, 3, 3, 6, 7, 8, 9];
        let med = median(&v);
        if let Ok(num) = med {
            assert_eq!(num, 6);
        }
    }

    #[test]
    fn test_median_2() {
        let v = vec![1, 6, 6, 7, 8, 9];
        let med = median(&v);
        if let Ok(result) = med {
            assert_eq!(result, 7);
        }
    }

    #[test]
    fn test_mode_1() {
        let v = vec![1, 3, 3, 6, 6, 6, 7, 8, 9];
        let mode = mode(&v);

        if let Ok(num_mode) = mode {
            assert_eq!(num_mode, 6);
        }
    }

    #[test]
    fn test_mode_2() {
        let v = vec![1, 6, 3, 7, 8, 3, 9];
        let mode = mode(&v);

        if let Ok(num_mode) = mode {
            assert_eq!(num_mode, 3);
        }
    }

    #[test]
    fn empty_mode() {
        let v = Vec::new();
        let mode = mode(&v);
        if let Err(msg) = mode {
            assert_eq!(msg, "Empty array.");
        }
    }
}
