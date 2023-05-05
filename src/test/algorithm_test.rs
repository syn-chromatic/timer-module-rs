use rand::rngs::ThreadRng;
use rand::Rng;

pub fn generate_array(size: usize) -> Vec<i32> {
    let mut array: Vec<i32> = Vec::new();
    let mut rng: ThreadRng = rand::thread_rng();

    for i in 0..size {
        let random_step: usize = rng.gen_range(1..=size);
        let value: i32 = (i * random_step) as i32;
        if !array.contains(&value) {
            array.push(value);
        }
    }

    array.sort();
    let random_split: usize = rng.gen_range(1..array.len());
    let output: Vec<i32> = [&array[random_split..], &array[..random_split]].concat();

    output
}

fn binary_search_recursive(array: &[i32], value: i32, mut low_idx: i32, mut high_idx: i32) -> i32 {
    let mid_idx: i32 = (low_idx + high_idx) / 2;

    let l_value: i32 = array[low_idx as usize];
    let m_value: i32 = array[mid_idx as usize];
    let h_value: i32 = array[high_idx as usize];

    if l_value == value {
        return low_idx;
    } else if m_value == value {
        return mid_idx;
    } else if h_value == value {
        return high_idx;
    } else if low_idx == mid_idx && high_idx - mid_idx <= 1 {
        return -1;
    }

    if m_value < value {
        if h_value > m_value && h_value > value {
            low_idx = mid_idx;
        } else if h_value < m_value {
            low_idx = mid_idx;
        } else if h_value < value {
            high_idx = mid_idx;
        }
    } else if m_value > value {
        if h_value < m_value && l_value > value {
            low_idx = mid_idx;
        } else if h_value > m_value {
            high_idx = mid_idx;
        } else if h_value < value {
            high_idx = mid_idx;
        }
    }
    let idx: i32 = binary_search_recursive(array, value, low_idx, high_idx);

    return idx;
}

pub fn binary_search_value(array: &[i32], value: i32) -> i32 {
    let low_idx: i32 = 0;
    let high_idx: i32 = (array.len() - 1) as i32;
    let idx: i32 = binary_search_recursive(&array, value, low_idx, high_idx);
    idx
}

pub fn get_highest_bit(n: i32) -> i32 {
    let mut bit: i32 = 1;
    while bit * 2 <= n {
        bit *= 2;
    }
    bit
}

pub fn get_bin(mut n: i32, length: usize) -> String {
    let mut binary: String = "".to_string();
    let mut bit: i32 = get_highest_bit(n);
    while bit >= 1 {
        if n - bit >= 0 {
            binary += "1";
            n -= bit;
            bit /= 2;
            continue;
        }
        binary += "0";
        bit /= 2;
    }
    let final_binary: String = "0".repeat(length - binary.len()) + &binary;
    final_binary
}

pub fn binary_digits(n: usize) -> Vec<String> {
    let mut bins: Vec<String> = Vec::new();
    bins.push("0".to_string());
    let mut binary: i32 = 0;
    let max_bin: String = "1".repeat(n);
    let mut binary_string: String = get_bin(binary, n);
    while binary_string != max_bin {
        binary += 1;
        binary_string = get_bin(binary, n);
        bins.push(binary_string.clone());
    }
    return bins;
}

fn binary_recursion(binary: String, n: usize) -> Vec<String> {
    if n == 0 {
        return vec![binary];
    }

    let binary_input1: String = binary.clone() + "0";
    let binary_input2: String = binary + "1";

    let mut rec_binary1: Vec<String> = binary_recursion(binary_input1, n - 1);
    let mut rec_binary2: Vec<String> = binary_recursion(binary_input2, n - 1);

    rec_binary1.append(&mut rec_binary2);
    rec_binary1
}

pub fn generate_binary_combinations(n: usize) -> Vec<String> {
    if n == 0 {
        return vec![];
    }

    binary_recursion("".to_string(), n)
}
