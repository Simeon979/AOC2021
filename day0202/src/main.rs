fn main() {
    let input = include_str!("../input.txt");
    let mut aim = 0;
    let (final_h_pos, final_d_pos) = input.split("\n").fold((0, 0), |(h_pos, d_pos), movement| {
        match movement.split(" ").collect::<Vec<_>>()[..] {
            ["forward", x] => {
                let x= x.parse::<u32>().unwrap();
                (h_pos + x , d_pos + (aim * x))
            },
            ["down", x] => {
                aim += x.parse::<u32>().unwrap();
                (h_pos, d_pos)
            }
            ["up", x] => {
                aim -= x.parse::<u32>().unwrap();
                (h_pos, d_pos)
            }
            _ => panic!("Unrecognized movement detected in input file"),
        }
    });
    println!("{}", final_d_pos * final_h_pos);
}
