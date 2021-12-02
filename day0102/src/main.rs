fn main() {
    let input = include_str!("../input.txt");
    let lines: Vec<usize> = input
        .split("\n")
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    let mut increase = 0;
    for idx in 3..lines.len() {
        if lines[idx] > lines[idx - 3] {
            increase += 1;
        }
    }
    println!("{}", increase);
}
