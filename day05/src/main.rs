fn main() {
    let mut points = std::collections::HashSet::new();
    let mut overlaps = std::collections::HashSet::new();
    include_str!("../input.txt").lines().map(|line| {
        line.split(" -> ").map(|coord| {
            if let [x, y] = coord.split(",").map(|axis| axis.parse().unwrap()).collect::<Vec<isize>>()[..]{
                return (x, y)
            } else {
                panic!("Error parsing input")
            };
        }).collect::<Vec<_>>()
    }).for_each(|line| {
        let (x1, y1) = line[0];
        let (x2, y2) = line[1];
        if x1 == x2 {
            for point in std::cmp::min(y1, y2)..=std::cmp::max(y1, y2) {
                if points.contains(&(x1, point)) {
                    overlaps.insert((x1, point));
                } else {
                    points.insert((x1, point));
                }
            }
        }

        else if y1 == y2 {
            for point in std::cmp::min(x1, x2)..=std::cmp::max(x1, x2) {
                if points.contains(&(point, y1)) {
                    overlaps.insert((point, y1));
                } else {
                    points.insert((point, y1));
                }
            }
        }

        else {
            for increase in 0..=(x1 - x2).abs() {
                let x = if x1 > x2 { x1 - increase } else { x1 + increase };
                let y = if y1 > y2 { y1 - increase } else { y1 + increase };
                // dbg!(x, y, x1, y1, x2, y2);
                if points.contains(&(x, y)) {
                    overlaps.insert((x, y));
                } else {
                    points.insert((x, y));
                }
            }
            // dbg!()
        }
    });
    println!("{}", overlaps.len());
}
