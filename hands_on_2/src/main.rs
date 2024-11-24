use hands_on_2::{IsThere, MinMax};
use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;
fn main() {
    let tests1: Vec<String> = vec![
        "./Testset_handson2_p1/input0.txt".to_string(),
        "./Testset_handson2_p1/input1.txt".to_string(),
        "./Testset_handson2_p1/input2.txt".to_string(),
        "./Testset_handson2_p1/input3.txt".to_string(),
        "./Testset_handson2_p1/input4.txt".to_string(),
        "./Testset_handson2_p1/input5.txt".to_string(),
        "./Testset_handson2_p1/input6.txt".to_string(),
        "./Testset_handson2_p1/input7.txt".to_string(),
        "./Testset_handson2_p1/input8.txt".to_string(),
        "./Testset_handson2_p1/input9.txt".to_string(),
        "./Testset_handson2_p1/input10.txt".to_string(),
    ];

    let output1: Vec<String> = vec![
        "./Testset_handson2_p1/output0.txt".to_string(),
        "./Testset_handson2_p1/output1.txt".to_string(),
        "./Testset_handson2_p1/output2.txt".to_string(),
        "./Testset_handson2_p1/output3.txt".to_string(),
        "./Testset_handson2_p1/output4.txt".to_string(),
        "./Testset_handson2_p1/output5.txt".to_string(),
        "./Testset_handson2_p1/output6.txt".to_string(),
        "./Testset_handson2_p1/output7.txt".to_string(),
        "./Testset_handson2_p1/output8.txt".to_string(),
        "./Testset_handson2_p1/output9.txt".to_string(),
        "./Testset_handson2_p1/output10.txt".to_string(),
    ];

    let tests2: Vec<String> = vec![
        "./Testset_handson2_p2/input0.txt".to_string(),
        "./Testset_handson2_p2/input1.txt".to_string(),
        "./Testset_handson2_p2/input2.txt".to_string(),
        "./Testset_handson2_p2/input3.txt".to_string(),
        "./Testset_handson2_p2/input4.txt".to_string(),
        "./Testset_handson2_p2/input5.txt".to_string(),
        "./Testset_handson2_p2/input6.txt".to_string(),
        "./Testset_handson2_p2/input7.txt".to_string(),
    ];

    let output2: Vec<String> = vec![
        "./Testset_handson2_p2/output0.txt".to_string(),
        "./Testset_handson2_p2/output1.txt".to_string(),
        "./Testset_handson2_p2/output2.txt".to_string(),
        "./Testset_handson2_p2/output3.txt".to_string(),
        "./Testset_handson2_p2/output4.txt".to_string(),
        "./Testset_handson2_p2/output5.txt".to_string(),
        "./Testset_handson2_p2/output6.txt".to_string(),
        "./Testset_handson2_p2/output7.txt".to_string(),
    ];

    println!("TESTSET 1");
    for (i, test) in tests1.iter().enumerate() {
        match file_reader_p1(test.to_string(), output1[i].to_string()) {
            Ok(_) => println!("Test case {} passed", test),
            Err(e) => println!("Test case {} failed: {}", test, e),
        }
    }

    println!("\nTESTSET 2");
    for (i, test) in tests2.iter().enumerate() {
        match file_reader_p2(test.to_string(), output2[i].to_string()) {
            Ok(_) => println!("Test case {} passed", test),
            Err(e) => println!("Test case {} failed: {}", test, e),
        }
    }
}

fn file_reader_p1(input_path: String, output_path: String) -> io::Result<()> {
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
        let _numbers: Vec<i32> = first_line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
    } else {
        println!("The file is empty or the first line could not be read.");
        return Err(Error::last_os_error());
    }

    let mut second_line = String::new();
    let mut min_max: MinMax;
    if input_reader.read_line(&mut second_line)? > 0 {
        let numbers: Vec<i32> = second_line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        min_max = MinMax::new(numbers);
    } else {
        println!("The file is empty or the first line could not be read.");
        return Err(Error::last_os_error());
    }

    let mut wrong: i32 = 0;

    for line in input_reader.lines() {
        let line = line?;

        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let result: Option<i32> = if numbers.len() == 3 {
            min_max.query(
                numbers[0] as usize,
                numbers[1] as usize,
                numbers[2] as usize,
                0,
            )
        } else {
            min_max.query(
                numbers[0] as usize,
                numbers[1] as usize,
                numbers[2] as usize,
                numbers[3],
            )
        };

        if numbers.len() == 3 {
            let mut output = String::new();
            let _ = output_reader.read_line(&mut output);
            let success: i32 = output.trim().parse().ok().unwrap();

            if success != result.unwrap() {
                wrong += 1;
                println!(
                    "max   : {:?} ## range: {} - {}     ######### ( {} ) ######## -> {}",
                    result,
                    numbers[1],
                    numbers[2],
                    (success == result.unwrap()),
                    success
                )
            }
        }
    }

    if wrong > 0 {
        return Err(Error::new(io::ErrorKind::Other, "Test case failed"));
    }
    Ok(())
}

fn file_reader_p2(input_path: String, output_path: String) -> io::Result<()> {
    if !Path::new(&input_path).exists() && !Path::new(&output_path).exists() {
        println!("File {} not found", input_path);
        return Ok(());
    }

    let input_file = File::open(input_path)?;
    let output_file: File = File::open(output_path)?;
    let mut input_reader = io::BufReader::new(input_file);
    let mut output_reader = io::BufReader::new(output_file);

    let mut first_line = String::new();
    let mut is_there: IsThere;
    if input_reader.read_line(&mut first_line)? > 0 {
        let numbers: Vec<i32> = first_line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        is_there = IsThere::new(numbers[0] as u128);
    } else {
        println!("The file is empty or the first line could not be read.");
        return Err(Error::last_os_error());
    }

    let mut wrong: i32 = 0;

    for line in input_reader.lines() {
        let line = line?;

        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let result: i8 = if numbers.len() == 2 {
            is_there.query(0, numbers[0] as usize, numbers[1] as usize, 0)
        } else {
            is_there.query(
                1,
                numbers[0] as usize,
                numbers[1] as usize,
                numbers[2] as u128,
            )
        };

        if numbers.len() == 3 {
            let mut output = String::new();
            let _ = output_reader.read_line(&mut output);
            let success: i8 = output.trim().parse().ok().unwrap();

            if success != result {
                wrong += 1;
                println!(
                    "max   : {:?} ## range: {} - {}     ######### ( FALSE ) ######## -> {}",
                    result, numbers[0], numbers[1], success
                )
            }
        }
    }

    if wrong > 0 {
        return Err(Error::new(io::ErrorKind::Other, "Test case failed"));
    }

    Ok(())
}
