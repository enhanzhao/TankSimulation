use regex::Regex;

// Split command on whitespace and return a vector of arguments
pub fn get_args(line: String) -> Vec<String> {
    let mut arguments: Vec<String> = Vec::new();

    for arg in Regex::new("[/\\s-]+").unwrap().split(line.as_str()){
            arguments.push(arg.to_string());
    }
    return arguments;
}

#[cfg(test)]
mod unit_test{
    use super::get_args;

    #[test]
    fn get_first_arg(){
        let args: Vec<String> = get_args("START R 0".to_string());
        assert_eq!("START", args.get(0).unwrap(), "The first agrument is START, but something else was caught.");
    }

    #[test]
    fn get_second_arg(){
        let args: Vec<String> = get_args("START       R 0".to_string());
        assert_eq!("R", args.get(1).unwrap(), "The second agrument is R, but something else was caught.");
    }

    #[test]
    fn get_third_arg(){
        let args: Vec<String> = get_args("START R        0".to_string());
        assert_eq!("0", args.get(2).unwrap(), "The third agrument is 0, but something else was caught.");
    }

    
}