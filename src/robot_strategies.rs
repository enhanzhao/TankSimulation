use crate::move_manager::{
    construct_drive_moves, construct_shoot_move, NORTH, NORTH_EAST, NORTH_WEST, SOUTH, SOUTH_EAST,
    SOUTH_WEST,
};
use crate::parser::get_args;
use crate::player::{Player, PlayerState, tank_type};
use crate::scan_parser::{self, GetScanReturn, ScanResponse};
use crate::strategy_controller::{strategiesType};
use crate::action_manager::{playerOutput, action_manager, manage_player_action};

// Chooses decisions on now the control the tank, based on 'phases'.
// Starts with phase 1, continues to phase 2 etc.
// Phase n does not necessarily have to be followed by phase n+1
// but all phases must flow into a different phase.

/// Turn type. Either Left or Right.
/// Left -> counter-clockwise
/// Right -> clockwise
/// 

enum Turn {
    Left,
    Right,
}

static DIRECTIONS: &'static [&str] =
    &[NORTH, NORTH_EAST, SOUTH_EAST, SOUTH, SOUTH_WEST, NORTH_WEST];




/// Helper function.
/// Turns the robot left or right from current position
fn turn_LR(player: &mut Player, turn_direction: Turn, action_manager :&mut action_manager) {
    let facing = player.get_facing_direction();
    let index = DIRECTIONS.iter().position(|&dir| dir == facing).unwrap();
    // Gets the corresponding index for facing direction

    let direction: &str;

    if matches!(turn_direction, Turn::Right) {
        direction = DIRECTIONS[(index + 1) % 6];
    } else {
        direction = DIRECTIONS[(index + 6 - 1) % 6];
    }
    action_manager.turn(direction, player);
}

/// Helper function.
/// Turns the robot left or right from current position N times but in a single turn move
fn turn_LR_n(player: &mut Player, turn_direction: Turn, n: usize) {
    let facing = player.get_facing_direction();
    let index = DIRECTIONS.iter().position(|&dir| dir == facing).unwrap();
    // Gets the corresponding index for facing direction

    let direction: &str;

    if matches!(turn_direction, Turn::Right) {
        direction = DIRECTIONS[(index + n) % 6];
    } else {
        direction = DIRECTIONS[(index + n - 1) % 6];
    }
}

/// Helper function.
/// Turns the tank to a random direction
fn turn_rand(player: &mut Player, action_manager :&mut action_manager) {
    action_manager.turn(DIRECTIONS[(rand::random::<u32>() % 5) as usize], player);
}

/// Helper function.
/// Turns the tank opposite current direction i.e. player.facing_direction
fn turn_opposite(player: &mut Player, action_manager :&mut action_manager) {
    let direction = match player.get_facing_direction().as_str() // Translate.
    {
        NORTH => SOUTH,
        SOUTH => NORTH,
        NORTH_EAST => SOUTH_WEST,
        SOUTH_WEST => NORTH_EAST,
        NORTH_WEST => SOUTH_EAST,
        SOUTH_EAST => NORTH_WEST,
        _ => ""
    };
    action_manager.turn(direction, player);
}

/// phase 1. Starts by finding a nearby corner. Then turns inward
/// to face the center of the board. Flows into phase 2.
/// Expects external call to phase 2.
pub fn orient_to_near_corner(player: &mut Player, scan :&mut ScanResponse, action_manager :&mut action_manager) {
 //   V    R
    //     /\
    //  B |  | O
    //     \/
    //   G    Y
    // let raw_scan = String::new();

    ////////////////////////////////////////////////////////////////////////
    /// If on round one and in the first actions of player, turn tank

    if player.get_rounds() == 1 && action_manager.get_action_counter() == 0{
        match player.get_colour().as_str() {
            "R" => action_manager.turn(SOUTH_EAST, player),
            "O" => action_manager.turn(SOUTH, player),
            "Y" => action_manager.turn(SOUTH_WEST, player),
            "G" => action_manager.turn(NORTH_WEST, player),
            "B" => action_manager.turn(NORTH, player),
            "V" => action_manager.turn(NORTH_EAST, player),
            _ => (),
        }
    } else {

        ////////////////////////////////////////////////////////////////////////
        /// If the forst action of the round scan
        if player.get_scan_action_count() == 0{
            action_manager.scan(player);
        } else{

            ////////////////////////////////////////////////////////////////////////
            /// If last action was a scan, check if ther is a wall right in front of the tank, if not then drive.
            match action_manager.get_last_action() {
                playerOutput::SCAN => {
                    if scan.get_walls().contains(&1) {
                        player.corner_found();                      

                    } else {
                        if player.get_drive_actions_check(){
                            // ------ BEFORE MOVING, DO WE HAVE ENOUGH POINTS TO MOVE? ---//
                            action_manager.drive(player);
                        }else{
                            action_manager.end();
                        }
                    }
                }
                ,
                playerOutput::DRIVE =>{
                    ////////////////////////////////////////////////////////////////////////
                    /// If last action was a drive, tank is on position 1, so we check if there is a wall in fornt of it, if not the 
                    if !scan.get_walls().contains(&5){
                        if player.get_drive_actions_check(){
                            // ------ BEFORE MOVING, DO WE HAVE ENOUGH POINTS TO MOVE? ---//
                            action_manager.drive(player);
                        }else{
                            action_manager.end();
                        }
                    }else {
                        player.corner_found();
                    }
                },
                _=> ()
            }
        
        }

    }

    if player.get_corner_status(){

        //////////////////////////////////////////////////////////////////////////////
        // CORNER WAS FOUND THE BOARD SHOULD BE CALIBRADED
        // 
        //  -- Board or player should have a flag taht when the board is calibraded the strategy controller will move to next strategy.
        //
        //  the q_coord, r_coord, and s_coord are the real coord
        //////////////////////////////////////////////////////////////////////////////
        match player.get_colour().as_str() {
            "R" => {
                    let q_coord :i32 = (player.get_side_len() -1) as i32;
                    let r_coord :i32 = -((player.get_side_len() -1) as i32);
                    let s_coord :i32 = 0;
                },
            "O" => {
                    let q_coord :i32 = (player.get_side_len() -1) as i32;
                    let r_coord :i32 = 0;
                    let s_coord :i32 = -((player.get_side_len() -1) as i32);
                },
            "Y" => {
                    let q_coord :i32 = 0;
                    let r_coord :i32 = (player.get_side_len() -1) as i32;
                    let s_coord :i32 = -((player.get_side_len() -1) as i32);
                },
            "G" => {
                    let q_coord :i32 = -((player.get_side_len() -1) as i32);
                    let r_coord :i32 = (player.get_side_len() -1) as i32;
                    let s_coord :i32 = 0;
                },
            "B" => {
                    let q_coord :i32 = -((player.get_side_len() -1) as i32);
                    let r_coord :i32 = 0;
                    let s_coord :i32 = (player.get_side_len() -1) as i32;
                },
            "V" => {
                    let q_coord :i32 = 0;
                    let r_coord :i32 = -((player.get_side_len() -1) as i32);
                    let s_coord :i32 = (player.get_side_len() -1) as i32;
                },
            _ => (),
        }

        let max_steps = 3;
        if player.get_step_count() < max_steps {
            action_manager.skip(player);
        }
    }
    

}

/**
 * Function that should be used by normal tank or scout, if its heavy then just skip untill the end
 */
pub fn explorer(player: &mut Player, scan :&mut ScanResponse, action_manager :&mut action_manager) {

    match player.get_tank_type() {
        tank_type::heavy => action_manager.end(),
        _ => {
            // OBS: There are actions that don't count as player actions [don't consume a step], like 'turn'  so actions_member does keep track of that as it keeps track of player last action.
            match player.get_step_count(){
                1 => {
                    // TODO: What to do on the first action the player performs.
                },
                2 => {
                    // TODO: What to do on the second action the player performs.
                },
                3 => {
                    // TODO: What to do on the third action the player performs.
                },
                _ => ()
            };
        }
    };
}

/**
 * Function to traverse the mmap after the initial exploration round is done.
 */
pub fn traverse (player: &mut Player, scan :&mut ScanResponse, action_manager :&mut action_manager) {


    match player.get_tank_type() {
        tank_type::heavy => {
            // OBS: There are actions that don't count as player actions [don't consume a step], like 'turn'  so actions_member does keep track of that as it keeps track of player last action.
            match player.get_step_count(){
                1 => {
                    // TODO: What to do on the first action the player performs.
                },
                2 => {
                    // TODO: What to do on the second action the player performs.
                },
                3 => {
                    // TODO: What to do on the third action the player performs.
                },
                _ => ()
            };
        },
        tank_type::scout => {
            // OBS: There are actions that don't count as player actions [don't consume a step], like 'turn'  so actions_member does keep track of that as it keeps track of player last action.
            match player.get_step_count(){
                1 => {
                    // TODO: What to do on the first action the player performs.
                },
                2 => {
                    // TODO: What to do on the second action the player performs.
                },
                3 => {
                    // TODO: What to do on the third action the player performs.
                },
                _ => ()
            };

        },
        tank_type::tank => {
            // OBS: There are actions that don't count as player actions [don't consume a step], like 'turn'  so actions_member does keep track of that as it keeps track of player last action.
            match player.get_step_count(){
                1 => {
                    // TODO: What to do on the first action the player performs.
                },
                2 => {
                    // TODO: What to do on the second action the player performs.
                },
                3 => {
                    // TODO: What to do on the third action the player performs.
                },
                _ => ()
            };
        },
        _ =>()
    };
}

// Exploration
/*
   Find corner
    |
    v
   looker -> stalker -> tank_movement

*/

// / phase 2. Goes toward the center twice and then drives around in a
// / smaller hexagon while looking for enemies. Flows into phase 3 (internal call)
// / Also flows into phase 4 if function ends (expected to be called externally).
// fn looker(player: &mut Player) {
//     // Drive inside loop
//     // Scan left and right
//     // drive forward
//     // drive twice since can scan 3 tiles away
//     player.drive();
//     player.end();
//     player.drive();
//     player.end();
//     // turn counter clockwise
//     turn_LR(player, Turn::Left);
//     let mut response: scan_parser::ScanResponse;
//     let mut drive_count = 0;
//     loop {
//         // 2 inside the hex so 2+1
//         if drive_count >= player.get_side_len() - 3 {
//             drive_count = 0;
//             turn_LR(player, Turn::Right);
//         }
//         turn_LR(player, Turn::Left);

//         response = player.scan();
//         player.end();

//         if response.get_enemy_detected() {
//             stalk(player);
//         };

//         turn_LR_n(player, Turn::Right, 2);
//         response = player.scan();
//         player.end();

//         if response.get_enemy_detected() {
//             stalk(player);
//         };

//         turn_LR(player, Turn::Left);
//         player.drive();
//         player.end();
//         drive_count += 1;

//         // if player rounds played >= exploration rounds, break

//         if player.get_rounds() >= player.get_exploration_rounds() {
//             break;
//         }
//     }
// }

// /// Helper function. Assumes enemy is directly in front of robot.
// /// Goes left and around the enemy, expects no obstacles adjacent to enemy
// fn flank(player: &mut Player) -> bool {
//     turn_LR(player, Turn::Left);
//     player.drive();
//     player.end();
//     turn_LR(player, Turn::Right);
//     player.drive();
//     player.end();
//     turn_LR_n(player, Turn::Right, 2);
//     return true;
// }

// /// phase 3. Stalks an enemy player. Closes the distance and attempts to circle around.
// /// ONLY FOR EXPLORATIOIN ROUNDS!
// /// Does not attack, only observes.
// /// returns false if target is lost, returns true if target is in vision and exploration rounds are over
// fn stalk(player: &mut Player) -> bool {
//     let mut player_lost_for_rounds = 0;
//     let mut response: ScanResponse;

//     loop {
//         // if exploration rounds are over, go to the tank phase
//         if player.get_rounds() >= player.get_exploration_rounds() {
//             return true;
//         }

//         response = player.scan();
//         player.end();

//         if response.get_enemy_detected() {
//             if response.get_enemies().contains(&(1 as usize)) {
//                 // upper: Driving twice
//                 if response.get_walls().contains(&0) && // check if we can get around player
//                     response.get_walls().contains(&4)
//                 {
//                     player.skip();
//                 } else {
//                     flank(player); // drives twice
//                 }
//             }
//             // Face towards enemy, then drive.
//             else if !response.get_enemies().contains(&8)
//                 && !response.get_enemies().contains(&9)
//                 && !response.get_enemies().contains(&10)
//             {
//                 let mut move_dir = construct_shoot_move(
//                     player.get_facing_direction(),
//                     response.get_enemies()[0] as usize,
//                 );
//                 move_dir = get_args(move_dir).get(1).unwrap().to_string();
//                 player.turn(&*move_dir);
//                 player.drive();
//                 player.end();
//             } else {
//                 // just drive forward hopefully in enemy direction
//                 player.drive();
//                 player.end();
//             }
//         } else {
//             player_lost_for_rounds += 1;
//             player.drive();
//             player.end();
//         }

//         if player_lost_for_rounds > 2 {
//             return false;
//         }
//     }
// }

// /// phase 4. Chosen for its exceptional survivability and adaptability.
// /// A very general set of moves that work after exploration rounds are over.
// /// Directly influenced by the professor's tank algorithm.
// /// Main difference is that it scans first instead of randomly turns first.
// /// This is to take advantage of previous functions that set up robot for the kill
// fn tank_movement(player: &mut Player) {
//     // eprintln!("Reached tank!");
//     let mut response: ScanResponse;

//     loop {
//         // start with scan
//         // We scan in a random direction and shoot at the closest thing to us,
//         response = player.scan();
//         player.end();

//         // We saw an enemy and they're in range, so we shoot!
//         if response.get_enemy_detected() && response.get_enemies()[0] <= 7 {
//             let shoot_command =
//                 construct_shoot_move(player.get_facing_direction(), response.get_enemies()[0]);
//             player.shoot(&*shoot_command);
//         }

//         let mut valid_moves = construct_drive_moves(player.get_facing_direction(), &mut response);

//         if valid_moves.len() > 0 {
//             let num_moves = valid_moves.len() as usize;
//             player.turn(&valid_moves[rand::random::<usize>() % num_moves][6..]);
//             player.drive();
//             player.end();
//         } else {
//             turn_opposite(player);
//             response = player.scan();
//             player.end();

//             valid_moves = construct_drive_moves(player.get_facing_direction(), &mut response);

//             let num_moves = valid_moves.len() as usize;
//             player.turn(&valid_moves[rand::random::<usize>() % num_moves][6..]);
//             player.drive();
//             player.end();
//         }

//         // in the end turn to a random direction
//         turn_rand(player);
//     }
// }

// /// The public facing function that starts executing the robot strategies
// pub fn execute_robot_strategies(player: &mut Player) {
//     orient_to_near_corner(player);
//     looker(player);
//     tank_movement(player);
// }
