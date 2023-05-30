use crate::scan_parser::{ScanResponse, GetScanReturn};


pub const NORTH: &str = "N";
pub const NORTH_EAST: &str = "NE";
pub const NORTH_WEST: &str = "NW";
pub const SOUTH: &str = "S";
pub const SOUTH_EAST: &str = "SE";
pub const SOUTH_WEST: &str = "SW";

enum DIRECTION {
    NORTH, NORTH_EAST, SOUTH_EAST, SOUTH, SOUTH_WEST, NORTH_WEST
}

/**
* An example would be:
*
* we're facing NE
* indices: 0 1 2 3 4 5 6 7 8 9 10
* char:    a b c d e f g h i j k
*
* 0 -> (turn(n), drive())
* 1 -> (drive())
* 2 -> (turn(se), drive())
* 3 -> (turn(n), drive(), drive())
*/
pub fn construct_drive_moves(player_direction :String, scanned_area:&mut ScanResponse) -> Vec<String> { // Needs a rewrite, additional info needed.
    let mut possible_drive_moves:Vec<String> = Vec::new();
    let player_facing_direction: &str = player_direction.as_str();
    if player_facing_direction == NORTH {
        if check_if_robot_can_move_to_index(0, scanned_area) {
            possible_drive_moves.push(String::from("DRIVE NW"));
        }
        if check_if_robot_can_move_to_index(1, scanned_area) {
            possible_drive_moves.push(String::from("DRIVE N"));

        }
        if check_if_robot_can_move_to_index(2, scanned_area) {
            possible_drive_moves.push(String::from("DRIVE NE"));

        }
    } else if player_facing_direction == NORTH_EAST {
        if check_if_robot_can_move_to_index(0, scanned_area) {
            possible_drive_moves.push(String::from("DRIVE N"));
        }
        if check_if_robot_can_move_to_index(1, scanned_area) {
            possible_drive_moves.push(String::from("DRIVE NE"));

        }
        if check_if_robot_can_move_to_index(2, scanned_area) {
            possible_drive_moves.push(String::from("DRIVE SE"));

        }
    } else if player_facing_direction == NORTH_WEST {
        if check_if_robot_can_move_to_index(0, scanned_area) {
            possible_drive_moves.push(String::from("DRIVE SW"));
        }
        if check_if_robot_can_move_to_index(1, scanned_area) {
            possible_drive_moves.push(String::from("DRIVE NW"));

        }
        if check_if_robot_can_move_to_index(2, scanned_area) {
            possible_drive_moves.push(String::from("DRIVE N"));

        }
    } else if player_facing_direction == SOUTH {
        if check_if_robot_can_move_to_index(0, scanned_area) {
            possible_drive_moves.push(String::from("DRIVE SW"));
        }
        if check_if_robot_can_move_to_index(1, scanned_area) {
            possible_drive_moves.push(String::from("DRIVE S"));

        }
        if check_if_robot_can_move_to_index(2, scanned_area) {
            possible_drive_moves.push(String::from("DRIVE SE"));

        }
    } else if player_facing_direction == SOUTH_EAST {
        if check_if_robot_can_move_to_index(0, scanned_area) {
            possible_drive_moves.push(String::from("DRIVE NE"));
        }
        if check_if_robot_can_move_to_index(1, scanned_area) {
            possible_drive_moves.push(String::from("DRIVE SE"));

        }
        if check_if_robot_can_move_to_index(2, scanned_area) {
            possible_drive_moves.push(String::from("DRIVE S"));

        }
    } else if player_facing_direction == SOUTH_WEST {
        if check_if_robot_can_move_to_index(0, scanned_area) {
            possible_drive_moves.push(String::from("DRIVE SE"));
        }
        if check_if_robot_can_move_to_index(1, scanned_area) {
            possible_drive_moves.push(String::from("DRIVE S"));

        }
        if check_if_robot_can_move_to_index(2, scanned_area) {
            possible_drive_moves.push(String::from("DRIVE SW"));

        }
    }
    return possible_drive_moves;
}

/**
 * Checks if a move to this index is possible for our robot.
 * Can't move if there's a wall at the index or an enemy is at the position.
 * 
 */
fn check_if_robot_can_move_to_index(index: usize, scanned_area: &mut ScanResponse) -> bool {
    return !scanned_area.get_enemies().contains(&index) 
    && !scanned_area.get_walls().contains(&index);
}

/**
 * This constructs the possible shoot movements that we can make for the first three - 
 * five hexagons within from our current position based on what we're facing
 * This will result in a string as a result based on the available data we have. 
 * Keep in mind we can shot several ways in some cases but we just chose one case to keep it simple.
 */
pub fn construct_shoot_move(player_direction :String, enemy_pos_index: usize) -> String {
    assert!(enemy_pos_index <= 8); // if this fails we're rekt.

    let player_facing_direction: &str = player_direction.as_str();
    if enemy_pos_index == 0 {
        match player_facing_direction {
            NORTH => "SHOOT NW".to_string(),
            NORTH_EAST => "SHOOT N".to_string(),
            NORTH_WEST => "SHOOT SW".to_string(),
            SOUTH => "SHOOT SE".to_string(),
            SOUTH_EAST => "SHOOT NE".to_string(),
            SOUTH_WEST => "SHOOT S".to_string(),
            _ => "".to_string(),
        }
    } else if enemy_pos_index == 1 {
        match player_facing_direction {
            NORTH => "SHOOT N".to_string(),
            NORTH_EAST => "SHOOT NE".to_string(),
            NORTH_WEST => "SHOOT NW".to_string(),
            SOUTH => "SHOOT S".to_string(),
            SOUTH_EAST => "SHOOT SE".to_string(),
            SOUTH_WEST => "SHOOT SW".to_string(),
            _ => "".to_string(),
        }
    } else if enemy_pos_index == 2 {
        match player_facing_direction {
            NORTH => "SHOOT NE".to_string(),
            NORTH_EAST => "SHOOT SE".to_string(),
            NORTH_WEST => "SHOOT N".to_string(),
            SOUTH => "SHOOT SW".to_string(),
            SOUTH_EAST => "SHOOT S".to_string(),
            SOUTH_WEST => "SHOOT NW".to_string(),
            _ => "".to_string(),
        }
    } else if enemy_pos_index == 3 {
        match player_facing_direction {
            NORTH => "SHOOT NW-NW".to_string(),
            NORTH_EAST => "SHOOT N-N".to_string(),
            NORTH_WEST => "SHOOT SW-SW".to_string(),
            SOUTH => "SHOOT SE-SE".to_string(),
            SOUTH_EAST => "SHOOT NE-NE".to_string(),
            SOUTH_WEST => "SHOOT S-S".to_string(),
            _ => "".to_string(),
        }
    } else if enemy_pos_index == 4 {
        match player_facing_direction {
            NORTH => "SHOOT NW-N".to_string(),
            NORTH_EAST => "SHOOT N-NE".to_string(),
            NORTH_WEST => "SHOOT SW-NW".to_string(),
            SOUTH => "SHOOT S-SE".to_string(),
            SOUTH_EAST => "SHOOT NE-SE".to_string(),
            SOUTH_WEST => "SHOOT S-SW".to_string(),
            _ => "".to_string(),
        }
    } else if enemy_pos_index == 5 {
        match player_facing_direction {
            NORTH => "SHOOT N-N".to_string(),
            NORTH_EAST => "SHOOT NE-NE".to_string(),
            NORTH_WEST => "SHOOT NW-NW".to_string(),
            SOUTH => "SHOOT S-S".to_string(),
            SOUTH_EAST => "SHOOT SE-SE".to_string(),
            SOUTH_WEST => "SHOOT SW-SW".to_string(),
            _ => "".to_string(),
        }
    } else if enemy_pos_index == 6 {
        match player_facing_direction {
            NORTH => "SHOOT N-NE".to_string(),
            NORTH_EAST => "SHOOT NE-SE".to_string(),
            NORTH_WEST => "SHOOT NW-N".to_string(),
            SOUTH => "SHOOT S-SW".to_string(),
            SOUTH_EAST => "SHOOT SE-S".to_string(),
            SOUTH_WEST => "SHOOT SW-NW".to_string(),
            _ => "".to_string(),
        }
    } else if enemy_pos_index == 7 {
        match player_facing_direction {
            NORTH => "SHOOT NE-NE".to_string(),
            NORTH_EAST => "SHOOT SE-SE".to_string(),
            NORTH_WEST => "SHOOT N-N".to_string(),
            SOUTH => "SHOOT SW-SW".to_string(),
            SOUTH_EAST => "SHOOT S-S".to_string(),
            SOUTH_WEST => "SHOOT NW-NW".to_string(),
            _ => "".to_string(),
        }
    } else {
        return String::from("");
    }
}

#[cfg(test)]
mod unit_test{

    use crate::move_manager::*;

    #[test]
    fn test_construct_drive_move_all_walls(){
        //11 walls                                          1               2             3              4              5              6              7              8              9              10             11
        let scan_string: Vec<String> = vec!["W".to_string(), "W".to_string(), "W".to_string(), "W".to_string(), "W".to_string(), "W".to_string(), "W".to_string(), "W".to_string(), "W".to_string(), "W".to_string(), "W".to_string()];
        
        let mut scan :ScanResponse = ScanResponse::initialize_scan_response();
        scan.scan_entry(scan_string);
        
        let move_output:Vec<String> = construct_drive_moves("N".to_string(), &mut scan);


        assert_eq!(0, move_output.len(), "Should have 0 viable moves, the output vector is not empty.");
    }

    #[test]
    fn test_construct_drive_move_all_viable(){

        //test all viable 
        //11 walls                                          1               2             3              4              5              6              7              8              9              10             11
        let scan_string: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string(), "f".to_string(), "g".to_string(), "h".to_string(), "i".to_string(), "j".to_string(), "k".to_string()];
        let mut scan :ScanResponse = ScanResponse::initialize_scan_response();
        scan.scan_entry(scan_string);
        
        let move_output:Vec<String> = construct_drive_moves("N".to_string(), &mut scan);

        assert_eq!(3, move_output.len(), "Should have all 11 viable moves, the output vector has less than 11.");
        assert_eq!("DRIVE NW", move_output.get(0).unwrap(), "Player is facing N, the first location should be NW. Got something else.");
    }
}