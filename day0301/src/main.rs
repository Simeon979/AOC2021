fn main() {
    let mut input = include_str!("../input.txt").lines().peekable();

    let first_line = input.peek().unwrap();

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for position in 0..first_line.len() {
        let mut postion_ones = 0;
        let mut position_zeros = 0;
        for line in input.clone(){
            if line.as_bytes()[position] as char == '0' {
                position_zeros += 1;
            } else {
                postion_ones += 1;
            }
        }

        gamma.push(if postion_ones > position_zeros { '1' } else { '0' });
        epsilon.push(if postion_ones < position_zeros  { '1' } else { '0' });
    }

    let gamma_int = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_int = usize::from_str_radix(&epsilon, 2).unwrap();
    println!("{}", gamma_int * epsilon_int);
}
