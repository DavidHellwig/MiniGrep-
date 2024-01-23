use std::env;
use colored::Colorize;

use std::io::BufRead;
use regex::Regex;


#[derive(Debug)]
struct MiniGrep{
    query: String,
    file_path:String
}

impl MiniGrep{
    fn from(s:String,f:String) -> MiniGrep{
        MiniGrep { 
        query: s,
        file_path: f }
    }
    fn open_and_scan_file(&self){
        let pattern = Regex::new(&self.query).unwrap();

        let file = std::fs::File::open(&self.file_path).expect("Error opening file");

        let buffered_reader = std::io::BufReader::new(file);

        for (line_number,line) in buffered_reader.lines().enumerate(){
            
            let new_line = &line.unwrap();
            
            for mat in pattern.find_iter(&new_line){

                let colored_match = mat.as_str().replace(mat.as_str(), &mat.as_str().red().to_string());

                let match_line = new_line.replace(mat.as_str(), &colored_match);

                println!("{} : {}",line_number+1,match_line);
            }


        }
        
    }
}



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3{
        panic!("Error: Invalid number of arguments")

    }

    let query = &args[1];
    let file_path = &args[2];

    

    let grep = MiniGrep::from(query.to_string(),file_path.to_string());
    grep.open_and_scan_file()
    
}