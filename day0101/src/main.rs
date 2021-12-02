fn main() {
    let input = include_str!("../input.txt");
    let lines: Vec<usize> = input
        .split("\n")
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    let (increase, _) = lines.iter()
        .fold(
            (0, lines[0]),
            |(n_increased, previous), current| {
                (
                    n_increased + if current > &previous { 1 } else { 0 },
                    current.to_owned(),
                )
            }
        );
    println!("{}", increase);
}
