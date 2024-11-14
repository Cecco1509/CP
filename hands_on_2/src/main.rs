use hands_on_2::MinMax;

fn main() {
    let vec = vec![1, 2, 3, 4, 5];

    let mut min_max = MinMax::new(vec);

    min_max.print_tree();

    println!("max -> {:?}", min_max.query(1, 2, 3, 0));
    println!("update -> {:?}", min_max.query(0, 2, 3, 1));
    println!("max -> {:?}", min_max.query(1, 2, 3, 0));
}
