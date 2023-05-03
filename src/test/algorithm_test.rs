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
