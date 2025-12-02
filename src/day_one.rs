use crate::file;

pub fn solve() {
    let mut sum = 50;
    let mut combination = 0;
    if let Ok(lines) = file::read_lines("days/1.txt") {
        for line in lines.map_while(Result::ok) {
            let ref_line = line.as_str();
            let (rotation, value) = ref_line.split_at(1);

            match rotation {
                "L" => sum -= value.parse::<i32>().unwrap(),
                "R" => sum += value.parse::<i32>().unwrap(),
                _ => panic!("Invalid rotation symbol"),
            }

            sum = ((sum % 100) + 100) % 100;

            if sum == 0 {
                combination += 1;
            }
        }
    }
    println!("day_one => {}", combination);
}
