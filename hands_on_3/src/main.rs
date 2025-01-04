use hands_on_3::find_maximum_attractions_opt;

fn main() {
    /*
    input2.txt

    5 4
    4 1 1 2
    1 1 0 5
    5 0 1 1
    2 1 0 4
    3 1 0 3

    output2.txt -> 14
    */

    let n: usize = 5;
    let d: usize = 4;
    let attractions: Vec<Vec<u32>> = vec![
        vec![4, 1, 1, 2],
        vec![1, 1, 0, 5],
        vec![5, 0, 1, 1],
        vec![2, 1, 0, 4],
        vec![3, 1, 0, 3],
    ];

    let result: u32 = find_maximum_attractions_opt(n, d, attractions);

    println!("{}", result);
}
