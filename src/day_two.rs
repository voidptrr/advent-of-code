use crate::file;

fn is_same_sequence_twice(id: &str) -> bool {
    let (left, right) = id.split_at(id.len() / 2);
    return left.eq(right);
}

fn get_range_ids(range: &Vec<&str>) -> (i64, i64) {
    let left_r = range[0].parse::<i64>().unwrap();
    let right_r = range[1].parse::<i64>().unwrap();

    return (left_r, right_r);
}

fn solve_part_two(lines: &Vec<String>) {
    let mut sum = 0;
    for line in lines {
        line.split(',').for_each(|range| {
            let ids: Vec<_> = range.split('-').collect();
            let (left_r, right_r) = get_range_ids(&ids);
            for num in left_r..=right_r {
                let id_string = num.to_string();
                let len = id_string.len();

                for seq_len in 1..=(len / 2) {
                    if len % seq_len != 0 {
                        continue;
                    }

                    let seq = &id_string[0..seq_len];
                    let times = len / seq_len;

                    if seq.repeat(times) == id_string {
                        sum += num;
                        break;
                    }
                }
            }
        });
    }

    println!("day_two [2] => {}", sum);
}

fn solve_part_one(lines: &Vec<String>) {
    let mut sum = 0;
    for line in lines {
        line.split(',').for_each(|range| {
            let ids: Vec<_> = range.split('-').collect();
            let (left_r, right_r) = get_range_ids(&ids);
            for num in left_r..=right_r {
                if is_same_sequence_twice(&num.to_string()) {
                    sum += num;
                };
            }
        });
    }

    println!("day_two [1] => {}", sum);
}

pub fn solve() {
    let lines = file::read_lines("days/2.txt");
    match lines {
        Ok(lines) => {
            let to_vec_lines: Vec<String> = lines.map_while(Result::ok).collect();
            solve_part_one(&to_vec_lines);
            solve_part_two(&to_vec_lines);
        }
        _ => panic!("Could not solve problem"),
    }
}
