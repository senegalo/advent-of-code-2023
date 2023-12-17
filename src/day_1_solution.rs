extern crate regex;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn _readfile(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut out: Vec<String> = Vec::new();

    for (_ , result) in reader.lines().enumerate() {
       out.insert(0, result.unwrap()) 
    }
    return out;
}

pub(crate) fn _solution1 () {
    __solution1(_readfile("./src/day-1-input-1"));
}

fn __solution1(input: Vec<String>) {
    let mut acc = 0;

    for line in input {
        let line_chars:Vec<char> = line.chars().collect();
        let mut start_done = false;
        let mut end_done = false;
        let mut start = 0;
        let mut end = line_chars.len()-1;
        while !start_done || !end_done {
            if !start_done && line_chars[start].is_numeric()  {
                acc += line_chars[start].to_digit(10).unwrap() * 10;    
                start_done = true;
            } else {
                start += 1;
            }

            if !end_done && line_chars[end].is_numeric() {
                acc += line_chars[end].to_digit(10).unwrap();
                end_done = true;
            } else {
                end -= 1;
            }
        }
    }

    println!("{acc}")

}

fn test_solution_2(){
    let numbers = "zero|one|two|three|four|five|six|seven|eight|nine";
    for (i, n) in numbers.split("|").enumerate() {
        let c = i*10+i;
        assert_eq!(__solution2(vec![String::from(n)]), c);
        assert_eq!(__solution2(vec![String::from(n), String::from(n)]), c*2);
        assert_eq!(__solution2(vec![i.to_string()]), c);
        assert_eq!(__solution2(vec![i.to_string(), i.to_string()]), c*2);
        assert_eq!(__solution2(vec![String::from(n), i.to_string()]), c*2);
        assert_eq!(__solution2(vec![format!("{}lkjsdfklj{}", n, i.to_string())]), c);
        assert_eq!(__solution2(vec![format!("{}lkjsdfklj{}", i.to_string(), n)]), c);
        assert_eq!(__solution2(vec![format!("{}lkj1123456789sdfklj{}", i.to_string(), n)]), c);
        assert_eq!(__solution2(vec![format!("{}lkjonetwothreefourfklj{}", i.to_string(), n)]), c);
        assert_eq!(__solution2(vec![format!("{}lkjonetwo98fourfklj{}", i.to_string(), n)]), c);
        assert_eq!(__solution2(vec![format!("lkj{}onetwo98four{}fklj", i.to_string(), n)]), c);
        assert_eq!(__solution2(vec![format!("lkjsa{}dfklj", n)]), c);
        assert_eq!(__solution2(vec![format!("lkjsa{}dfklj", i.to_string())]), c);
        assert_eq!(__solution2(vec![format!("lkjsa{}dfklj", i.to_string())]), c);
    }

    assert_eq!(__solution2(vec![String::from("twoone")]), 21);
    assert_eq!(__solution2(vec![String::from("eighttwo")]), 82);
    assert_eq!(__solution2(vec![String::from("nineight")]), 98);
    assert_eq!(__solution2(vec![String::from("eighthree")]), 83);
    assert_eq!(__solution2(vec![String::from("nineeight")]), 98);
    assert_eq!(__solution2(vec![String::from("eeeight")]), 88);
    assert_eq!(__solution2(vec![String::from("oooeeone")]), 11);
}

pub(crate) fn _solution2(){
    test_solution_2();
    println!("Results Are {}", __solution2(_readfile("./src/day-1-input-1-test2")));
    println!("Results Are {}", __solution2(_readfile("./src/day-1-input-1")));
}

fn __solution2(input: Vec<String>) -> usize{
    let mut set : HashMap<String, usize>= HashMap::new();
    let numbers = "zero|one|two|three|four|five|six|seven|eight|nine";
    for (index, n ) in numbers.split('|').enumerate() {
        set.insert(String::from(n), index);
        set.insert((index).to_string(), index);
    }

    let re_start = Regex::new(&format!(r"({}|[0-9])", numbers)).unwrap();
    let re_end= Regex::new(&format!(r".*({}|[0-9])", numbers)).unwrap();

    let mut acc:usize= 0;

    for (index, line) in input.iter().enumerate() {
        print!("#{} line: {}, ", index, line);
        let lower_case_line = line.to_lowercase();
        let start_captures: Vec<_>= re_start.captures_iter(&lower_case_line).map(|c| c.extract::<1>().1).collect();
        let end_captures: Vec<_>= re_end.captures_iter(&lower_case_line).map(|c| c.extract::<1>().1).collect();
        let start = start_captures[0][0];
        let end = end_captures[0][0];
        print!("start: {}, end: {}, ", start, end);
        let digit = set.get(start).unwrap() * 10 + set.get(end).unwrap();
        acc += digit;
        println!("digit: {} acc: {}", digit, acc);
    }
    return acc;
}