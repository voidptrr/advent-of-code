use crate::file;

fn sub_sequence_max_by_k(values: &[u32], k: usize) -> u64 {
    let mut stack: Vec<u32> = Vec::with_capacity(k);

    for (index, &value) in values.iter().enumerate() {
        while let Some(&last) = stack.last() {
            if value > last && stack.len() - 1 + (values.len() - index) >= k {
                stack.pop();
            } else {
                break;
            }
        }

        if stack.len() < k {
            stack.push(value);
        }
    }

    let mut num: u64 = 0;
    for &digit in &stack {
        num = num * 10 + digit as u64;
    }

    num
}

pub fn solve_part_two(lines: &[String]) {
    let sum: u64 = lines
        .iter()
        .map(|l| {
            let values: Vec<u32> = l.chars().map(|c| c.to_digit(10).unwrap()).collect();
            sub_sequence_max_by_k(&values, 12)
        })
        .sum();
    println!("day_three [2] => {}", sum);
}

pub fn solve_part_one(lines: &[String]) {
    let sum: u64 = lines
        .iter()
        .map(|l| {
            let values: Vec<u32> = l.chars().map(|c| c.to_digit(10).unwrap()).collect();
            sub_sequence_max_by_k(&values, 2)
        })
        .sum();
    println!("day_three [1] => {}", sum);
}

pub fn solve() {
    let lines = file::read_lines("days/3.txt");
    match lines {
        Ok(lines) => {
            let to_vec_lines: Vec<String> = lines.map_while(Result::ok).collect();
            solve_part_one(&to_vec_lines);
            solve_part_two(&to_vec_lines);
        }
        _ => panic!("Could not solve problem"),
    }
}
