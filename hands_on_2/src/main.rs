use hands_on_2::MinMax;
use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;
fn main() {
    let tests: Vec<String> = vec![
        "./testset1/input0.txt".to_string(),
        "./testset1/input1.txt".to_string(),
        "./testset1/input2.txt".to_string(),
        "./testset1/input3.txt".to_string(),
        "./testset1/input4.txt".to_string(),
        "./testset1/input5.txt".to_string(),
        "./testset1/input6.txt".to_string(),
        "./testset1/input7.txt".to_string(),
        "./testset1/input8.txt".to_string(),
        "./testset1/input9.txt".to_string(),
        "./testset1/input10.txt".to_string(),
    ];

    let output: Vec<String> = vec![
        "./testset1/output0.txt".to_string(),
        "./testset1/output1.txt".to_string(),
        "./testset1/output2.txt".to_string(),
        "./testset1/output3.txt".to_string(),
        "./testset1/output4.txt".to_string(),
        "./testset1/output5.txt".to_string(),
        "./testset1/output6.txt".to_string(),
        "./testset1/output7.txt".to_string(),
        "./testset1/output8.txt".to_string(),
        "./testset1/output9.txt".to_string(),
        "./testset1/output10.txt".to_string(),
    ];

    for (i, test) in tests.iter().enumerate() {
        println!("\n\n\nTEST {} \n", i);
        match file_reader(test.to_string(), output[i].to_string()) {
            Ok(_) => println!("Test case {} passed", test),
            Err(e) => println!("Test case {} failed: {}", test, e),
        }
    }
}

fn file_reader(input_path: String, output_path: String) -> io::Result<()> {
    if !Path::new(&input_path).exists() && !Path::new(&output_path).exists() {
        println!("File {} not found", input_path);
        return Ok(());
    }

    let input_file = File::open(input_path)?;
    let output_file: File = File::open(output_path)?;
    let mut input_reader = io::BufReader::new(input_file);
    let mut output_reader = io::BufReader::new(output_file);

    let mut first_line = String::new();
    if input_reader.read_line(&mut first_line)? > 0 {
        let numbers: Vec<i32> = first_line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        //println!("First line: {:?}", numbers);
    } else {
        println!("The file is empty or the first line could not be read.");
    }

    let mut second_line = String::new();
    let mut min_max: MinMax;
    if input_reader.read_line(&mut second_line)? > 0 {
        let numbers: Vec<i32> = second_line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        //println!("Array line: {:?}", numbers);
        // for i in 0..numbers.len() {
        //     println!("[{}] -> {}", (i), numbers[i]);
        // }
        min_max = MinMax::new(numbers);
        //min_max.print_tree();
        // match min_max.validate_tree() {
        //     Ok(_) => println!("Tree is valid!"),
        //     Err(err) => println!("Tree validation failed: {}", err),
        // }
    } else {
        println!("The file is empty or the first line could not be read.");
        return Err(Error::last_os_error());
    }

    let mut right : i32 = 0;
    let mut wrong : i32 = 0;

    for line in input_reader.lines() {
        let line = line?;

        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let result: Option<i32> = if numbers.len() == 3 {

            min_max.query(numbers[0] as usize, numbers[1] as usize, numbers[2] as usize, 0)

        } else {
            min_max.query(numbers[0] as usize, numbers[1] as usize, numbers[2] as usize, numbers[3])
        };

        if numbers.len() == 4 {
            // println!(
            //     "update: % ## range: {} - {} ## T: {}",
            //     numbers[1], numbers[2], numbers[3]
            // );

        } else {

            let mut output = String::new();
            let _ = output_reader.read_line(&mut output);
            let success: i32 = output.trim().parse().ok().unwrap();

            if success == result.unwrap() {
                right += 1;
            }else {
                wrong += 1;
                println!(
                "max   : {:?} ## range: {} - {}     ######### ( {} ) ######## -> {}",
                    result, numbers[1], numbers[2], (success == result.unwrap()), success
                )
            }

            // println!(
            //     "max   : {:?} ## range: {} - {}     ######### ( {} ) ######## -> {}",
            //     result, numbers[1], numbers[2], (success == result.unwrap()), success
            // )
        }
    }

    println!("TOT: {} right: {}, wrong: {}\n", right+wrong, right, wrong);

    if wrong > 0 {
        return Err(Error::new(io::ErrorKind::Other, "Test case failed"));
    }
    Ok(())
}
