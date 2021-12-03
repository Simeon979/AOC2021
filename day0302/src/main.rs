fn main() {
    let mut input = include_str!("../input.txt").lines().peekable();

    let first_line_length = input.peek().unwrap().len();

    let mut oxygen_lines: Vec<&str> = input.collect();
    let mut co2_lines: Vec<&str> = oxygen_lines.clone();

    for position in 0..first_line_length{
        let mut position_ones = Vec::new();
        let mut position_zeros = Vec::new();
        for line in oxygen_lines{
            if line.as_bytes()[position] as char == '0' {
                position_zeros.push(line.clone());
            } else {
                position_ones.push(line.clone());
            }
        }

        oxygen_lines = if position_ones.len() >= position_zeros.len() { position_ones } else { position_zeros };
        if oxygen_lines.len() == 1 { break };
    }

    for position in 0..first_line_length{
        let mut position_ones = Vec::new();
        let mut position_zeros = Vec::new();
        for line in co2_lines{
            if line.as_bytes()[position] as char == '0' {
                position_zeros.push(line.clone());
            } else {
                position_ones.push(line.clone());
            }
        }

        co2_lines = if position_ones.len() >= position_zeros.len() { position_zeros } else { position_ones };
        if co2_lines.len() == 1 { break };
    }

    let oxygen_rating = usize::from_str_radix(oxygen_lines[0], 2).unwrap();
    let co2_rating = usize::from_str_radix(co2_lines[0], 2).unwrap();
    println!("{}", oxygen_rating * co2_rating);
}
