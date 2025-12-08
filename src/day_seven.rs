use crate::file;

fn solve_part_two<T: AsRef<str>>(input: &[T]) -> u64 {
    let width = input[0].as_ref().len();

    let grid: Vec<_> = input.iter().flat_map(|l| l.as_ref().as_bytes()).collect();

    let mut bits: Vec<u64> = vec![0; width];
    let s_pos = input[0].as_ref().chars().position(|c| c == 'S').unwrap();
    bits[s_pos] += 1;

    let rest_grid = &grid[width + 1..];

    let mut index = 0;
    for (i, data) in rest_grid.iter().enumerate() {
        if i.is_multiple_of(width) {
            index = 0;
        }
        match data {
            b'.' => index += 1,
            b'^' => {
                index += 1;
                if bits[index] > 0 {
                    if index < width {
                        bits[index + 1] += bits[index];
                    }

                    if index >= 1 {
                        bits[index - 1] += bits[index];
                    }
                }
                bits[index] = 0;
            }
            _ => panic!("not handled"),
        }
    }

    bits.iter().sum()
}

fn solve_part_one<T: AsRef<str>>(input: &[T]) -> u64 {
    let height = input.len();
    let width = input.iter().map(|line| line.as_ref().len()).max().unwrap();

    let mut bits = vec![false; width + 1];
    let mut buffer = vec![b' '; width * height];

    for (chunk, line) in buffer.chunks_exact_mut(width).zip(input.iter()) {
        let bytes_line = line.as_ref().as_bytes();
        let len = bytes_line.len().min(width);
        chunk[..len].copy_from_slice(&bytes_line[..len]);
    }

    let pos_s = input[0].as_ref().chars().position(|x| x == 'S').unwrap();
    bits[pos_s] = true;

    let mut count = 0;
    let mut index: usize = 0;

    let s_buffer = &buffer[width + 1..];
    for data in s_buffer.iter() {
        if index.is_multiple_of(width) {
            index = 0;
        }

        match data {
            b'.' => index += 1,
            b'^' => {
                index += 1;
                if bits[index] {
                    bits[index] = false;
                    if index > 0 {
                        bits[index - 1] = true;
                    }

                    if index < width {
                        bits[index + 1] = true;
                    }
                    count += 1;
                }
            }
            _ => panic!("unhandled"),
        }
    }
    count
}

pub fn solve() {
    let lines = file::read_lines("days/7.txt");
    match lines {
        Ok(lines) => {
            let to_vec_lines: Vec<String> = lines.map_while(Result::ok).collect();
            println!("day_seven [1] => {}", solve_part_one(&to_vec_lines));
            println!("day_seven [2] => {}", solve_part_two(&to_vec_lines));
        }
        _ => panic!("Could not solve problem"),
    }
}
