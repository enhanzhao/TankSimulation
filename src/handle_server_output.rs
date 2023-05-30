use std::io::{self, stdin, stdout, BufRead, Write};
use std::process::exit;

use crate::CoordSystem::{self, board_operations, board};
use crate::action_manager::{action_manager, playerOutput, manage_player_action};
use crate::parser::{self, get_args};
use crate::player::{self, Player, PlayerState};
use crate::robot_strategies;
use crate::scan_parser::{self, GetScanReturn, ScanResponse};
use crate::strategy_controller::{strategy_controller, startegies, strategiesType};

const POINTS_EXPENDED_INDEX: usize = 1;
const ROUND_NUMBER_INDEX: usize = 1;

enum ServerResponseType {
    // TODO: cleanup
    // Start,
    Damage,
    Dead,
    Move,
    Ok,
    Huh,
    Action,
    Timeout,
    Finish,
}

/**
 * Actions_history helps to keep track of action performed last and wha was the server response.
 */
struct Actions_history {
    server_resp: String,
    player_action: String,
    constructed_server_resp: Vec<String>,
}

/**
 * Scan response struct is reponsible to hold information about a scan action:
 *  - scanned_positions: A vector of String that contains positions from 'a' to 'k' in alphabetical order.
 *  - enemies_pos: A vector of usize that holds the position of enemies returned by the scan. for example:
 *      -> If the returned scan is abc Refgh ijk. This means that there is an enemy R and the index of the enemy on scanned_position vector
 *      will be 3.
 *  - walls_pos: A vector of usize that holds the position of walls returned by the scan. for example:
 *      -> If the returned scan is abW defWW ijW. This means that there are walls 'W' and their index on scanned_position vector are [2, 6, 7, 10].
 * - enemy_detected: True if the scan detected an enemy, enemies has the following possible letters [R, O, Y, G, B, V].
 * - wall_detected: true if there are walls detected by the scan.
 *   
 */

impl Actions_history {
    /**
     * Initialize actions_history
     */
    fn initialize() -> Self {
        Self {
            server_resp: " ".to_string(),
            player_action: " ".to_string(),
            constructed_server_resp: Vec::new(),
        }
    }

    /**
     * Record the last action the player has performed.
     */
    fn set_player_action(&mut self, action: String) {
        self.player_action = action;
    }

    fn get_arg_vector(self) -> Vec<String>{
        return self.constructed_server_resp.clone();
    }

    /**
     * Record the server response to the last action the player has performed, or the last server response.
     */
    fn set_server_resp(&mut self, response: String) {
        self.server_resp = response;
        self.constructed_server_resp = parser::get_args(self.server_resp.clone());
        // This a vec of string i.e ['DAMAGE', 'N', 'NE']
    }

    fn get_server_response_type(&mut self) -> ServerResponseType {
        match self.constructed_server_resp.get(0).unwrap().as_str() {
            // "START" => ServerResponseType::Start,
            "MOVE" => ServerResponseType::Move,
            "FINISH" => ServerResponseType::Finish,
            "DEAD" => ServerResponseType::Dead,
            "ACTION!" => ServerResponseType::Action,
            "TIMEOUT!" => ServerResponseType::Timeout,
            "OK" => ServerResponseType::Ok,
            "HUH?" => ServerResponseType::Huh,
            "DAMAGE" => ServerResponseType::Damage,
            _ => ServerResponseType::Action,
        }
    }
}

pub fn get_input() -> String {
    let mut line = String::new();
    io::stdin()
        .lock()
        .read_line(&mut line)
        .expect("Could not read line");
    return line;
}

/**
 * Handles server output, it is the main controller of the program and should be the one responsible to update other modules
 *  such as player and board. 
 * 
 * It is also responsible to contact the startege controller and pass on the active strategy to the robot strategy. 
 * 
 * The flow is as follows:
 * 
 *  1. Wait to receive a message from the recerver on its STDIN
 *  2. Match the resopnse type.
 *  3. Update player model.
 *  4. Stratehy manager will validate which strategy should be used.
 *  5. Run the defined strategy.
 * 
 * Each strategy has a simple AI to veryfy the players state and make a decision, it will take in consideration:
 *     - Round number.
 *     - Player step [how many actions has the player taken in the current round].
 *     - Others ??
 */
pub fn handle_server_output(player :&mut Player) {


    // INITIALIZE DATA STRUCTURES
    let mut startegy_manager :startegies = strategy_controller::initialize_initial_startegy();      // --> Responsible to decide active strategy.
    let mut action_manager :action_manager = action_manager::initialize();                          // --> Controller that active player will use to communicate with server.
    let mut board :board = board::initialize(player.get_side_len() as i32);                         // --> Board data structure.
    let mut scan :ScanResponse = GetScanReturn::initialize_scan_response();                         // --> scan object, returns informatino about the scanned area ussed on Robot_strategies.
        

    while player.get_health() > 0 {

        // Initialize and set a data structure for the server response. Facilitates data parsing.
        let mut action = Actions_history::initialize();
        let line:String = get_input();
        action.set_server_resp(line);
        let response_type:ServerResponseType = action.get_server_response_type();


        match response_type {
            
            ServerResponseType::Move => {

                /////////////////////////////////////////////////////////////////////////////////////
                ///  TODO: Might want to updated the board based on the other players comms channel
                /// 
                /// ************************ If the board is calibraded **************************
                /////////////////////////////////////////////////////////////////////////////////////

                //Update rond number
                let new_round_num = action.get_arg_vector().get(ROUND_NUMBER_INDEX).unwrap().clone();
                player.start_round(new_round_num.parse().unwrap());                           // => Reset player counters

                action_manager.reset_counter();                                                             // => reset auxiliary function to cound actions taken that doesn't consume points but are important for the strategy
                startegy_manager.avaliate_startegy(player, &mut action_manager);                            // => Check if player should changhe strategy
                go_to_strategy(player, &mut startegy_manager, &mut scan, &mut action_manager);              // => call the right strategy


            },
            ServerResponseType::Finish => exit(0),
            ServerResponseType::Dead => exit(0),
            ServerResponseType::Ok => {                                                                                             // ==> If the response is Ok, than previous action was accepted.
                let max_steps_allowed = 3;
                // If players has taken all the 
                if player.get_step_count() < max_steps_allowed{

                    // Check player's last action
                    match action_manager.get_last_action() {
                        playerOutput::DRIVE =>{
                            let points_expended = action.get_arg_vector().get(POINTS_EXPENDED_INDEX).unwrap().clone();
                            player.update_points_count(points_expended.parse().unwrap());                                               // ==> Player needs to update the points, the OK after a drive will return the total points
                        }
                        _ =>()
                    }
                    startegy_manager.avaliate_startegy(player, &mut action_manager);                                                    // => Check if player should changhe strategy
                    go_to_strategy(player, &mut startegy_manager, &mut scan, &mut action_manager);                                      // => call the right strategy
                }
            },
            ServerResponseType::Huh => {
                // TODO: Something was wrong in the last response to the server, fix it.
                action_manager.end();
            },
            ServerResponseType::Damage => {
                player.take_damage();
                if player.get_health() == 0 {
                    exit(0);
                }
            },
            _ => {

                match action_manager.get_last_action() {
                    playerOutput::SCAN => {
                        scan.scan_entry(action.get_arg_vector());
                        //board.update_board(&mut scan);                                                            // => uncomment when board is updated and functional.
                        startegy_manager.avaliate_startegy(player, &mut action_manager);                            // => Check if player should changhe strategy
                        go_to_strategy(player, &mut startegy_manager, &mut scan, &mut action_manager);              // => call the right strategy
                    },
                    playerOutput::TURN => (),
                    playerOutput::SKIP => (),
                    _ =>{
                        println!("END");
                        stdout().flush();
                    }
                }
            },
        }
    }
}

fn go_to_strategy(player :&mut Player, startegy_manager :&mut startegies, scan :&mut ScanResponse, action_manager :&mut action_manager){

    match startegy_manager.get_active_strategy() {
        strategiesType::find_corner => robot_strategies::orient_to_near_corner(player, scan, action_manager),
        strategiesType::exlporer => robot_strategies::explorer(player, scan, action_manager),
        strategiesType::traverse => robot_strategies::traverse(player, scan, action_manager),
        _=> (),
    }
}