// PART 1
pub fn find_maximum_attractions(n: usize, k: usize, attractions: Vec<Vec<u32>>) -> u32 {
    // I need only two rows to compute the result, one for the current city and one for the previous ones
    // I swap the rows to avoid copying the values
    let rows: usize = 2;
    let columns: usize = k + 1;

    let mut matrix: Vec<Vec<u32>> = vec![vec![0; columns]; rows];

    // Inizialize the first row to the first city itinerary
    for i in 0..(columns - 1) {
        matrix[0][i + 1] = matrix[0][i] + attractions[0][i];
    }

    // Loop through the cities except the first one
    for (i, itinerary) in attractions.iter().enumerate().skip(1) {
        let mut current_value: u32 = itinerary[0];

        let (current_row, compare_row): (usize, usize) = if i % 2 != 0 { (1, 0) } else { (0, 1) };

        // compare the first attraction of the current city with the previous cities
        for l in 1..(k + 1) {
            matrix[current_row][l] = u32::max(
                current_value + matrix[compare_row][l - 1],
                matrix[compare_row][l],
            );
        }

        // compare the other attractions of the current city with the values in the current row
        // but still using the previous cities values
        for (j, dayly_attractions) in itinerary.iter().enumerate().skip(1) {
            current_value += dayly_attractions;

            for l in 1..(k + 1) {
                if j < l {
                    matrix[current_row][l] = u32::max(
                        current_value + matrix[compare_row][l - (j + 1)],
                        matrix[current_row][l],
                    );
                }
            }
        }
    }

    matrix[(n - 1) % 2][k]
}

// PART 2
pub fn find_maximum_topics(mut topics: Vec<(u32, u32)>) -> u32 {
    // Sort by beauty
    topics.sort_by(|a, b| a.0.cmp(&b.0));

    let mut bst = vec![(topics[0].1, 0_usize)];

    // Find the LIS in the difficulty column
    for (i, topic) in topics.iter().enumerate().skip(1) {
        let last = bst.last().unwrap();

        if topic.1 > last.0 && topic.0 != topics[last.1].0 {
            bst.push((topic.1, i));
        } else {
            let mut left = 0;
            let mut right = bst.len() - 1;

            while left < right {
                let mid = left + (right - left) / 2;

                if bst[mid].0 < topic.1 {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }

            bst[left].0 = topic.1;
            bst[left].1 = i;
        }
    }

    bst.len() as u32
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

        let result: u32 = find_maximum_attractions(n, d, attractions);

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

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn test0() {
        /*
        input0.txt

        5
        0 3
        99 1
        11 20
        1 2
        10 5

        output0.txt -> 3
        */

        let topics: Vec<(u32, u32)> = vec![(0, 3), (99, 1), (11, 20), (1, 2), (10, 5)];

        let result: u32 = find_maximum_topics(topics);

        assert_eq!(result, 3);
    }

    #[test]
    fn test1() {
        /*
        input1.txt

        3
        6 4
        13 11
        10 8


        output1.txt -> 3
        */

        let topics: Vec<(u32, u32)> = vec![(6, 4), (13, 11), (10, 8)];

        let result: u32 = find_maximum_topics(topics);

        assert_eq!(result, 3);
    }

    #[test]
    fn test2() {
        /*
        input2.txt

        2
        3 3
        3 3

        output2.txt -> 4
        */

        let topics: Vec<(u32, u32)> = vec![(3, 3), (3, 3)];

        let result: u32 = find_maximum_topics(topics);

        assert_eq!(result, 1);
    }

    #[test]
    fn test3() {
        let topics: Vec<(u32, u32)> = vec![
            (44, 49),
            (15, 35),
            (38, 21),
            (55, 93),
            (14, 29),
            (50, 52),
            (94, 76),
            (89, 84),
            (30, 96),
            (41, 14),
            (17, 38),
            (30, 14),
            (21, 100),
            (12, 78),
            (86, 87),
        ];

        let result: u32 = find_maximum_topics(topics);

        assert_eq!(result, 6);
    }

    #[test]
    fn test4() {
        let topics: Vec<(u32, u32)> = vec![
            (54, 56),
            (66, 50),
            (74, 97),
            (52, 23),
            (62, 74),
            (27, 56),
            (73, 24),
            (11, 47),
            (32, 83),
            (51, 29),
            (12, 74),
            (4, 48),
            (51, 22),
            (82, 82),
            (1, 24),
        ];

        let result: u32 = find_maximum_topics(topics);

        assert_eq!(result, 5);
    }

    #[test]
    fn test5() {
        let topics: Vec<(u32, u32)> = vec![
            (56, 90),
            (61, 30),
            (82, 62),
            (60, 44),
            (72, 58),
            (20, 80),
            (46, 79),
            (39, 15),
            (67, 46),
            (64, 63),
            (72, 9),
        ];

        let result: u32 = find_maximum_topics(topics);

        assert_eq!(result, 5);
    }

    #[test]
    fn test6() {
        let topics: Vec<(u32, u32)> = vec![
            (64, 56),
            (51, 51),
            (61, 74),
            (88, 53),
            (1, 15),
            (50, 81),
            (43, 24),
            (53, 78),
            (6, 34),
            (33, 46),
            (27, 1),
            (9, 37),
            (18, 47),
            (38, 21),
            (69, 95),
        ];

        let result: u32 = find_maximum_topics(topics);

        assert_eq!(result, 7);
    }

    #[test]
    fn test7() {
        let topics: Vec<(u32, u32)> = vec![
            (64, 56),
            (51, 51),
            (61, 74),
            (88, 53),
            (1, 15),
            (50, 81),
            (43, 24),
            (53, 78),
            (6, 34),
            (33, 46),
            (27, 1),
            (9, 37),
            (18, 47),
            (38, 21),
            (69, 95),
        ];

        let result: u32 = find_maximum_topics(topics);

        assert_eq!(result, 7);
    }

    #[test]
    fn test8() {
        let topics: Vec<(u32, u32)> = vec![
            (33, 5),
            (52, 5),
            (33, 54),
            (80, 11),
            (12, 78),
            (62, 2),
            (17, 1),
            (66, 79),
            (94, 30),
            (54, 14),
            (28, 17),
            (100, 70),
        ];

        let result: u32 = find_maximum_topics(topics);

        assert_eq!(result, 5);
    }

    #[test]
    fn test9() {
        let topics: Vec<(u32, u32)> = vec![
            (80, 88),
            (7, 62),
            (60, 14),
            (27, 60),
            (95, 66),
            (68, 71),
            (10, 76),
            (14, 87),
            (6, 92),
            (81, 81),
            (80, 90),
        ];

        let result: u32 = find_maximum_topics(topics);

        assert_eq!(result, 4);
    }

    #[test]
    fn test10() {
        let topics: Vec<(u32, u32)> = vec![
            (30, 73),
            (4, 89),
            (66, 60),
            (61, 22),
            (30, 16),
            (94, 60),
            (27, 87),
            (75, 8),
            (91, 33),
            (69, 78),
            (41, 69),
            (70, 12),
            (88, 76),
            (91, 92),
        ];

        let result: u32 = find_maximum_topics(topics);

        assert_eq!(result, 5);
    }
}
