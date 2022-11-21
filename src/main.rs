#![allow(unused)]
use std::{fs, path::{self, PathBuf, Path}, num::ParseIntError, borrow::Borrow, process::exit, ops::{Index, Add}, ptr::eq, vec, array};
use core::f32::consts::E;
use core::num::IntErrorKind::InvalidDigit;
fn main() {
    loop {
        let mut parsed_input: usize;
        let mut can_read = false;
        let mut read_path: PathBuf;
        println!("These are the files that were found:");
        let mut increment = 1;
        let mut paths = vec![];
        for entry in fs::read_dir("text_files").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                println!("{:?} is a dir", path);
            } else {
                let printedpath = println!("{0}. {1:?}", increment, &path);
                let read_path = &path;
                let mut content = fs::read_to_string(read_path).unwrap();
                increment += 1;
                paths.push(path.to_owned());
            }
        }
        println!("Which File Would You Like To Open?");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("can not read user input");
        match input.trim().parse::<usize>() {
            Ok(i) => {input = input.trim().parse::<usize>().unwrap().to_string(); 
                let mut vecindex = input.parse::<String>().unwrap();
                let nvecindex = vecindex.parse::<i32>().unwrap() - 1;
                if nvecindex < paths.len().try_into().unwrap() {
                    let mut chosen_file = &paths[nvecindex as usize];
                    let mut filecontent = fs::read_to_string(chosen_file).unwrap();
                    //println!("{}", filecontent);
                    linechecker(filecontent);
                }
                else {
                    println!("That is not a valid choice. Please Try Again");
                };
            }
            Err(..) => { println!("That is not a number, Please try again");}
        }
    }
}

fn linechecker((c):(String)) { 
    let mut all_arrays: Vec<[String; 2]> = vec![];
    for line in c.lines() {
        let layers:String = ((line.len() - line.trim().len())/4).to_string();
        let mut layer_and_text_arr = [layers, line.trim().to_string()];
        all_arrays.push(layer_and_text_arr);
    }
    //println!("{:?}", roots);
    //println!("{:?}", all_arrays)
    search(all_arrays);
}

fn search(incoming_array:Vec<[String; 2]>) {
    let mut all_lines;
    let mut lines_vec = vec![];
    for (index, elem) in incoming_array.iter().enumerate() {
        //println!("{},{:?}", index, elem);
        all_lines = LineInfo {
            line_number: index.to_string().parse::<i32>().unwrap().add(1),
            line_content: elem[1].clone(),
            depth: elem[0].clone(),
        };
        lines_vec.push(all_lines);
    };
    println!("What were you looking for?");
        let mut input2 = String::new();
        std::io::stdin()
            .read_line(&mut input2)
            .expect("can not read user input");
    for n in lines_vec {
        if n.iter().any(|&i| i==input2) {
            println!("yes")
        }
        else {}
        }
}

#[derive(Debug)]
struct LineInfo 
{
    line_number: i32,
    line_content: String,
    depth: String
}