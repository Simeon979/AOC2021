fn main() {
    let input = include_str!("../input.txt");
    let (final_h_pos, final_d_pos)= input
        .split("\n")
        .fold((0, 0), |(h_pos, d_pos), movement| {
            match movement.split(" ").collect::<Vec<_>>()[..] {
                ["forward", x] => (h_pos + x.parse::<u32>().unwrap(), d_pos),
                ["down", x] =>  (h_pos, d_pos + x.parse::<u32>().unwrap()),
                ["up", x] => (h_pos, d_pos - x.parse::<u32>().unwrap()),
                _ => panic!("Unrecognized movement detected in input file")
            }
        });
    println!("{}", final_d_pos * final_h_pos);
}
