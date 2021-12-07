fn subsquence_after_n(sequence: &Vec<usize>, subsequence: &Vec<usize>) -> Option<usize> {
    let mut after_n = 0;
    for n in subsequence {
        if let Some(pos) = sequence.iter().position(|m| m == n) {
            after_n = std::cmp::max(pos, after_n);
        } else {
            return None;
        }
    }
    Some(after_n)
}

fn main() {
    let mut input = include_str!("../input.txt").lines();

    let sequence: Vec<usize> = input
        .nth(0)
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let mut boards: Vec<Vec<Vec<usize>>> = vec![];
    while let Some(board_separator) = input.by_ref().next() {
        assert_eq!("", board_separator); // make sure we don't mess up the parsing and discard a board line as a board separator
        let board: Vec<Vec<usize>> = input
            .by_ref()
            .take(5)
            .map(|line| {
                line.trim()
                    .split(" ")
                    .filter(|n| *n != "")
                    .map(|n| n.trim().parse().unwrap())
                    .collect()
            })
            .collect();
        boards.push(board);
    }

    let mut min_moves = sequence.len() + 1;
    let mut max_moves = 0;
    let mut min_result = 0;
    let mut max_result = 0;

    for board  in boards {
        let mut board_min = sequence.len() + 1;
        for row in &board {
            if let Some(after_n) = subsquence_after_n(&sequence, row) {
                board_min = std::cmp::min(board_min, after_n);
            }
        }

        for col in 0..5 {
            let mut col_seq = vec![];
            for row in &board {
                col_seq.push(row[col]);
            }
            if let Some(after_n) = subsquence_after_n(&sequence, &col_seq) {
                board_min = std::cmp::min(board_min, after_n);
            }
        }

        if board_min < min_moves {
            min_moves = board_min;
            let winning_n = sequence[board_min];
            let unmarked_sum: usize = board.concat().iter().filter(|x| !sequence[0..board_min + 1].contains(*x)).sum();
            min_result = winning_n * unmarked_sum;
        }
        if board_min > max_moves {
            max_moves = board_min;
            let winning_n = sequence[board_min];
            let unmarked_sum: usize = board.concat().iter().filter(|x| !sequence[0..board_min + 1].contains(*x)).sum();
            max_result = winning_n * unmarked_sum;
        }
    }
    println!("min: {}, max: {}", min_result, max_result)
}
