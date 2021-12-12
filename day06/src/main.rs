use std::collections::HashMap;
#[derive(Debug)]
struct Lantern {
    lifecycle: usize,
    reprod: usize,
}

impl Lantern {
    fn new(lifecycle: usize, reprod: usize) -> Self {
        Self { lifecycle, reprod }
    }
}
fn main() {
    let mut store: HashMap<(usize, usize), usize> = HashMap::new();

    let result = include_str!("../input.txt")
        .split(',')
        .map(|n| Lantern::new(256, n.parse().unwrap()))
        .fold(0, |acc, l| acc + get_children(&l, &mut store));
    println!("{:?}", result);
}

fn get_children(lantern: &Lantern, store: &mut HashMap<(usize, usize), usize>) -> usize {
    let result;
    if let Some(c) = store.get(&(lantern.lifecycle, lantern.reprod)) {
        *c
    } else {
        if lantern.reprod >= lantern.lifecycle {
            result = 1;
        } else {
            let mut generations = get_generations(lantern.lifecycle - lantern.reprod - 1);
            generations.push(Lantern::new(lantern.lifecycle - lantern.reprod - 1, 8));
            result = generations
                .iter()
                .fold(1, |acc, x| acc + get_children(x, store));
        }
        store.insert((lantern.lifecycle, lantern.reprod), result);
        result
    }
}
fn get_generations(days: usize) -> Vec<Lantern> {
    let mut children = Vec::new();
    for i in 1..=days / 7 {
        children.push(Lantern::new(days - (7 * i), 8));
    }
    children
}
