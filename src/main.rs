use lib::handle_server_output::get_input;
use lib::{*};
use std::env;
use std::io::{self, BufRead, stdout, Write};
use std::process::exit;
mod robot_strategies;
use lib::player::{Player, PlayerState, tank_type};
use parser::get_args;
use std::io::{Error, ErrorKind};

mod mock_server;

const ROBOT_COLOR_INDEX :usize = 2;
const EXPLORATION_TURNS_INDEX :usize = 3;
const SIDE_LEN_INDEX: usize = 1;



fn main() {
    
    let mut tank_check = false;
    let mut scout_check = false;
    let mut heavy_check = false;

    // Get list of arguments supplied to program
    let args: Vec<String> = env::args().collect();

    // Get the arguments, the argument we are interested in is on the inderx one of the list.
    let tank_type_initialization = match args.get(1).unwrap().as_str() {
        "T"|"t" => {
                tank_check = true;
                tank_type::tank
            },
        "H"|"h" =>{ 
                heavy_check = true;
                tank_type::heavy
            },
        "S"|"s" => {
                scout_check = true;
                tank_type::scout
            },
        _ => tank_type::error
    };

    // Check if tank initialization has errors
    let check_error = match tank_type_initialization {
        tank_type::error => true,
        _ => false
    };

    // Throw error if thank option is not acceptable
    if check_error{
        Error::new(ErrorKind::InvalidData, "The executable should be called with one of the 3 possible arguments: (T or t): normal tank, (H or h): Heavy tank, (S or s): Scout tank.");
    }


    let mut player :Player = Player::initialize_player("R".to_string(), 100 as u32, 5, tank_type_initialization);

    let first_input = get_args(handle_server_output::get_input());

    match first_input[0].as_str() {
        "START" => {
            player.set_initial_information(
                first_input[ROBOT_COLOR_INDEX].clone(), 
                first_input[EXPLORATION_TURNS_INDEX].parse().unwrap(), 
                first_input[SIDE_LEN_INDEX].parse().unwrap());

                println!("IAM {}", args.get(1).unwrap().as_str());

                let mut is_valid :bool = false;
                
                while !is_valid{
                    let scond_input = get_args(handle_server_output::get_input());

                    match scond_input[0].as_str(){
                        "OK" => is_valid = true,
                        _ => {
                            if !tank_check{
                                println!("IAM T");
                            }else if !scout_check{
                                println!("IAM S");
                                scout_check = true;
                            }else if !heavy_check {
                                println!("IAM H");
                                heavy_check = true;
                            }else{
                                is_valid = true;
                                println!("The tank options where not valid due time out or duplicated tank definition.");
                                exit(1);
                            }
                        }
                    };
                }

                ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                //                                              :TODO
                // comms::start_listener_thread(); ** we need to fire a thread for listening to other player responses.**
                // This listener should provide information to the current player so that he makes informed decisions.
                //
                // If needed pass the listener as argument to the function 'handle_server_output' or initialize it on that function
                ////////////////////////////////////////////////////////////////////////////////////////////////////////////////

                handle_server_output::handle_server_output(&mut player);
        }
        _ => {
            // We should do nothing here because we handle server output inside (execute_robot_strategies)
            println!("Something went wrong, server probably didn't send anything valid, or reading from the CLI is incorrect.");
        }
    }
}
