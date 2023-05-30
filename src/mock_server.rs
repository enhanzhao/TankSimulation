use std::fs::File;
use std::io::{BufReader};
use std::io::prelude::*;
use std::path::Path;


/**
 * Function responsable to mock a server.
 * 
 * The function will read each line from a file (serverInput.txt), and add it to a Vector.
 * 
 * consider the file having lines
 *  --------------------------------------------------------------------------------------
 *  START START 5 R 100
 *  HUH?
 *  OK
 *  --------------------------------------------------------------------------------------
 * The vector returned will contain:
 * 
 *  Index
 *  [0]: START START 5 R 100
 *  [1]: HUH?
 *  [2]: OK
 * 
 * @Return Vec<String>: It will return a vector where each member represents a line from the file
 */
pub fn read_file_to_vector() -> Vec<String>{

    let mut file_vector: Vec<String> = Vec::new();

    //Checks if the file actually exists. Panics with error message if not found.
    //Create a bufd reader to parse the input file. Read file into a str, and close the buff reader.  
    if Path::new("./mock_server_files/serverInput.txt").exists(){
        
        let file = File::open("./mock_server_files/serverInput.txt").expect("Something went wrong, coudn't open the serverInput.txt File");
        let buf_file = BufReader::new(file);

        for line in buf_file.lines().filter_map(|result| result.ok()){
            file_vector.push(line);
        }

    }else{
        panic!("There is no serverInput file. readFileToVector() can't read file to vector.");
    }

    return file_vector;

}