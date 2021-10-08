extern crate protobuf;
use std::fs::File;
use std::io::{self,BufRead};
use std::io::prelude::*;
use std::path::Path;
use std::env;
pub mod Person_msg;

// To read each line from given input file 
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// To print a empty line
fn pr(){
    println!();
}

// To create a output file by converting text into byte streams 
fn convert()
{
    let mut file=File::create("OUTPUT_FILE").expect("Create Failed");
    let mut count=0;
    if let Ok(lines) = read_lines("./Input_file.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let text=format!("{}\n",ip);
                let v:Vec<&str>=ip.split(',').collect();
                let mut person=Person_msg::Person::new();
                count+=1;
                println!("Person {} details: ",count);
                person.set_lname(v[0].to_string());
                person.set_fname(v[1].to_string());
                person.set_dob(v[2].to_string());
                println!("{:?}",person);
                
                let bytes=text.into_bytes();
                let ab=protobuf::Message::write_length_delimited_to_bytes(&person);
                match file.write_all(&bytes){
                    Err(why) => panic!("couldn't write: {}",why),
                    Ok(_) => bytes,
                };
                println!("{:?}",ab);
            }
        }
        
        println!("\nSuccessfully written to the file");
    }
}

fn help() {
    println!("Too Many arguments");
}

fn getting_arguments() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // no arguments passed
        1 => {
            println!("No arguments Received");
        },
        // one argument passed
        2 => {
            println!("Received only Input file\n");
            //let str = &args[1];
            //display_content_of_file(str.to_string());
        }
        // all the other cases
        3 =>{
            println!("Received both the files\n");
            /*let str = &args[1];
            println!("INPUT FILE CONTENTS: ");
            display_content_of_file(str.to_string());
            pr();*/
            convert();
        }
        _ => {
            // show a help message
            help();
        }
    }
}


fn main()
{
    pr();
    getting_arguments();
}