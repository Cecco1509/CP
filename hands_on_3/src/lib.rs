pub fn find_maximum_attractions(n: usize, k: usize, attractions: Vec<Vec<u32>>) -> u32 {
    let rows: usize = ((n - 1) * k) + 1; // this could be optimized to k + 1
    let columns: usize = k + 1;

    let mut matrix: Vec<Vec<u32>> = vec![vec![0; columns]; rows];

    println!("rows: {}, columns: {}", rows, columns);
    for j in 0..(columns - 1) {
        matrix[0][j + 1] = attractions[0][j];
    }

    let mut current_city: usize = 1;
    let mut current_weight: usize = 1;
    let mut current_value: u32 = 0;

    for i in 1..(rows) {
        current_value += attractions[current_city][current_weight - 1];

        for j in 1..(columns) {
            if j < current_weight {
                matrix[i][j] = matrix[i - 1][j];
            } else {
                matrix[i][j] = u32::max(
                    current_value + matrix[i - current_weight][j - current_weight],
                    matrix[i - 1][j],
                );
            }
        }

        current_weight += 1;
        if current_weight > k {
            current_weight = 1;
            current_city += 1;
            current_value = 0;
        }
    }

    matrix[rows - 1][columns - 1]
}

pub fn find_maximum_attractions_opt(n: usize, k: usize, attractions: Vec<Vec<u32>>) -> u32 {
    let rows: usize = 2;
    let columns: usize = k + 1;

    let mut matrix: Vec<Vec<u32>> = vec![vec![0; columns]; rows];

    println!("rows: {}, columns: {}", rows, columns);
    for i in 0..(columns - 1) {
        matrix[0][i+1] = matrix[0][i] + attractions[0][i];
    }

    println!("{:?}", matrix);

    // City
    for i in 1..n {

        let mut current_value: u32 = 0;

        let mut delta: i8 = - 1;

        // Attractions weight
        for j in 0..k {
            //println!("{} ", attractions[i][j]);

            // Attraction current value
            current_value += attractions[i][j];

            println!("current_value: {} c: {} w: {}", current_value, i, j);


            /* create an example matrix

                j
              l 0 3 5
            
            */

            // Weight
            let current_row = if delta < 0 { 1 } else { 0 };

            for l in 1..(k+1) {
                //println!("j: {} l: {}", j, l);
                
                if j+1 <= l {
                    println!("comparison: {} - {} pos: {} curr: {} ", current_value + matrix[(current_row + delta) as usize][l-(j+1)], matrix[(current_row + delta) as usize][l], l-(j+1), current_value);
                    println!("delta: {} current_row: {}", delta, current_row);
                    println!("matrix: {:?}", matrix);
                    matrix[(current_row + delta) as usize][l] = u32::max(
                        current_value + matrix[(current_row + delta) as usize][l-(j+1)],
                        matrix[(current_row + delta) as usize][l],
                    );
                }else {
                    matrix[(current_row) as usize][l] = matrix[(current_row + delta) as usize][l];
                }
            }

            delta -= 2*delta;

        }

        println!("{:?}", matrix);

    }

    matrix[1][k]
}


#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn test0() {
        /*
        input0.txt

        6 8
        3 2 1 4 2 4 3 4
        3 3 1 2 3 5 5 3
        3 4 1 5 3 3 4 1
        3 1 5 4 3 4 2 5
        2 5 4 4 4 5 3 4
        5 1 4 4 3 2 4 5

        output0.txt -> 32
        */

        let n: usize = 6;
        let d: usize = 8;
        let attractions: Vec<Vec<u32>> = vec![
            vec![3, 2, 1, 4, 2, 4, 3, 4],
            vec![3, 3, 1, 2, 3, 5, 5, 3],
            vec![3, 4, 1, 5, 3, 3, 4, 1],
            vec![3, 1, 5, 4, 3, 4, 2, 5],
            vec![2, 5, 4, 4, 4, 5, 3, 4],
            vec![5, 1, 4, 4, 3, 2, 4, 5],
        ];

        let result: u32 = find_maximum_attractions(n, d, attractions);

        assert_eq!(result, 32);
    }

    #[test]
    fn test1() {
        /*
        input1.txt

        3 5
        1 1 1 4 2
        3 3 5 3 5
        2 1 4 5 1

        output1.txt -> 19
        */

        let n: usize = 3;
        let d: usize = 5;
        let attractions: Vec<Vec<u32>> = vec![
            vec![1, 1, 1, 4, 2],
            vec![3, 3, 5, 3, 5],
            vec![2, 1, 4, 5, 1],
        ];

        let result: u32 = find_maximum_attractions(n, d, attractions);

        assert_eq!(result, 19);
    }

    #[test]
    fn test2() {
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

        assert_eq!(result, 14);
    }

    #[test]
    fn test3() {
        /*
        input3.txt

        2 15
        1 0 2 2 0 0 2 2 0 0 0 2 0 2 2
        0 1 1 0 0 1 2 2 1 1 1 1 2 2 1

        output3.txt -> 16
        */

        let n: usize = 2;
        let d: usize = 15;
        let attractions: Vec<Vec<u32>> = vec![
            vec![1, 0, 2, 2, 0, 0, 2, 2, 0, 0, 0, 2, 0, 2, 2],
            vec![0, 1, 1, 0, 0, 1, 2, 2, 1, 1, 1, 1, 2, 2, 1],
        ];

        let result: u32 = find_maximum_attractions(n, d, attractions);

        assert_eq!(result, 16);
    }

    #[test]
    fn test4() {
        /*
        input4.txt

        10 10
        3 0 0 0 0 0 0 0 0 3
        0 3 0 0 0 0 0 0 4 0
        0 0 2 0 0 0 0 5 0 0
        0 0 0 5 0 0 1 0 0 0
        0 0 0 0 5 4 0 0 0 0
        0 0 0 0 4 1 0 0 0 0
        0 0 0 1 0 0 5 0 0 0
        0 0 5 0 0 0 0 5 0 0
        0 5 0 0 0 0 0 0 5 0
        4 0 0 0 0 0 0 0 0 3

        output4.txt -> 21
        */

        let n: usize = 10;
        let d: usize = 10;
        let attractions: Vec<Vec<u32>> = vec![
            vec![3, 0, 0, 0, 0, 0, 0, 0, 0, 3],
            vec![0, 3, 0, 0, 0, 0, 0, 0, 4, 0],
            vec![0, 0, 2, 0, 0, 0, 0, 5, 0, 0],
            vec![0, 0, 0, 5, 0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 5, 4, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 4, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 5, 0, 0, 0],
            vec![0, 0, 5, 0, 0, 0, 0, 5, 0, 0],
            vec![0, 5, 0, 0, 0, 0, 0, 0, 5, 0],
            vec![4, 0, 0, 0, 0, 0, 0, 0, 0, 3],
        ];

        let result: u32 = find_maximum_attractions(n, d, attractions);

        assert_eq!(result, 21);
    }
}
