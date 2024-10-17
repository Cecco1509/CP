
use std::{collections::VecDeque, vec};
use num::FromPrimitive;
use num::Num;
use std::cmp::PartialOrd;

fn main() {
    let v = vec![1, 2, 3, 4, 1, 2];
    let ret = next_largest_elem(&v);

    for elem in ret {
        println!("{} -> {}", elem.0, elem.1);
    }

    println!("{}", sqrt(25))
}

fn next_largest_elem(nums: &Vec<i32>) -> Vec<(i32, i32)> {
    let n = nums.len();
    let mut d: VecDeque<i32> = VecDeque::new();
    let mut maxs: Vec<(i32, i32)> = Vec::with_capacity(n);

    for elem in nums {
        while (!d.is_empty()) && *d.back().unwrap() < *elem {
            maxs.push((d.pop_back().unwrap(), *elem));
        }

        d.push_back(*elem);
    }

    maxs
}


fn binary_search_range<T, F>(low: T, high: T, pred: F) -> Option<T>
where
    T: Num + PartialOrd + FromPrimitive + Copy,
    F: Fn(T) -> bool,
{
    let mut low = low;
    let mut high = high;

    let mut ans = None;

    while low < high {
        let middle = low + (high - low) / FromPrimitive::from_u64(2).unwrap();

        match pred(middle) {
            true => {
                low = middle + T::one();
                ans = Some(middle)
            }
            false => high = middle,
        }
    }

    ans
}

fn sqrt(v : u64) -> u64 {
    binary_search_range(0, v+1, |x| x*x <= v).unwrap()
}
