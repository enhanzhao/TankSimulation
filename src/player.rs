use std::ptr::null;

use crate::CoordSystem::Point;
use crate::move_manager::{construct_drive_moves, construct_shoot_move};
use crate::scan_parser::{GetScanReturn, ScanResponse};

#[derive(Debug, Clone, Copy)]
pub enum tank_type{
    heavy, 
    scout,
    tank,
    error
}

/**
 * player struct
 */
pub struct Player {
    round_number: u32,
    tank_type: tank_type,
    max_scans: u32,
    max_shoots: u32,
    max_move:u32,
    drive_actions_taken: u32,
    scan_actions_taken: u32,
    skip_actions_taken: u32,
    shot_actions_taken: u32,
    total_possible_points: u32,
    points_count: u32,
    health: u8,
    facing_directon: String,
    colour: String,
    max_exploration_rounds: u32,
    side_length: u32,
    steps: u32,
    found_corner: bool
}

pub trait PlayerState {
    fn initialize_player(colour: String, exploration_rounds: u32, side_length: u32, tank_type: tank_type) -> Self;
    fn set_initial_information(&mut self, colour: String, max_round: u32, side_length: u32);
    fn construct_possible_moves(&mut self, scanned_area: &mut ScanResponse) -> Vec<String>; // ==> Should be the controller decision making not the player model.
    //-----------SETTERS/UPDATE------------//
    fn take_damage(&mut self);
    fn start_round(&mut self, new_round_num: u32);
    fn update_facing_direction(&mut self, direction: &str);
    fn update_points_count(&mut self, points_count :u32);
    fn add_shoot_action(&mut self);
    fn add_drive_action(&mut self);
    fn add_scan_action(&mut self);
    fn add_step(&mut self);
    fn add_skip_action(&mut self);
    fn corner_found(&mut self);
    //-----------GETTERS-------------------//
    fn get_colour(&self) -> String;
    fn get_side_len(&self) -> u32;                     //======> SAME INFO WILL BE RECORDED ON THE BOARD DATA STRUCTURE, COULD BE **DELETED**
    fn get_rounds(&self) -> u32;
    fn get_health(&mut self) -> u8;
    fn get_exploration_rounds(&self) -> u32;
    fn get_facing_direction(&mut self) -> String;
    fn get_drive_actions_check(&self) -> bool;
    fn get_scan_actions_check(&self) -> bool;
    fn get_skip_actions_check(&self) -> bool;
    fn get_shoot_action_check(&self) -> bool;
    fn get_scan_action_count(&self) -> u32;
    fn get_points_left(&self) -> i32;
    fn get_step_count(&self) ->u32;
    fn get_corner_status(&self)-> bool;
    fn get_tank_type(&self) ->tank_type;
}

impl PlayerState for Player {
    /**
     * function responsible to initialize the player.
     *
     * - round_nummber: The number of round that has passed in the game.
     * - total_points: number of action taken by the player in a round.
     * - drive_actions_taken: The number of move action a player performed in a round.
     * - scan_actions_taken: The number of scan action a player performed in a round.
     * - shot_this_round: Check if the player has shot in the current round. just one shot is allowed per round.
     * - health: The current health of the player.
     */
    fn initialize_player(colour: String, exploration_rounds: u32, side_length: u32, tank_type: tank_type) -> Self {
        let mut max_scans_point = 0;
        let mut max_shoots_point = 0;
        let mut max_move_point = 0;
        let mut total_possible_points = 0;

        match tank_type{
            tank_type::heavy => {
                max_move_point = 2;
                max_shoots_point = 2;
                max_scans_point = 1;
                total_possible_points = 5;
            },
            tank_type::scout => {
                max_move_point = 4;
                max_shoots_point = 1;
                max_scans_point = 3;
                total_possible_points = 5;
            },
            _ =>{
                max_move_point = 2;
                max_shoots_point = 1;
                max_scans_point = 2;
                total_possible_points = 4;
            }
        }

        Self {
            round_number: 1,
            tank_type: tank_type,
            max_scans: max_scans_point,
            max_shoots: max_shoots_point,
            max_move:max_move_point,
            drive_actions_taken: 0,
            scan_actions_taken: 0,
            shot_actions_taken: 0,
            skip_actions_taken: 0,
            total_possible_points: total_possible_points,
            points_count: 0,
            health: 2,
            facing_directon: "N".to_string(),
            colour: colour,
            max_exploration_rounds: exploration_rounds,
            side_length: side_length,
            steps: 0,
            found_corner: false
        }
    }

    /**
     * Set the initial information for the player after the server offers more information about the player such as:
     *  - colour: The player colour, which is the map section's colour where the player was spawned.
     *  - max_round: The number of max exploration rounds.
     *  - side_length: the number of cells in one side of the map.
     */
    fn set_initial_information(&mut self, colour: String, max_round: u32, side_length: u32) {
        self.colour = colour;
        self.max_exploration_rounds = max_round;
        self.side_length = side_length;
    }

    /**
     * Function to restart a round and increase values:
     * - total_points: reset to 3, every round has 3 steps.
     * - round_number: increase by 1.
     * - drive_actions_taken: reset to 0.
     * - scan_actions_taken: reset to 0.
     * - shot_this_round: reset to false.
     */
    fn start_round(&mut self, new_round_num: u32) {
        self.drive_actions_taken = 0;
        self.scan_actions_taken = 0;
        self.shot_actions_taken = 0;
        self.skip_actions_taken = 0;
        self.steps = 0;
        self.points_count = 0;
        self.round_number = new_round_num
    }

    /**
     * Return the player health.
     */
    fn get_health(&mut self) -> u8 {
        return self.health;
    }

    /**
     * Function to decrease player's health if they take damage.
     */
    fn take_damage(&mut self) {
        self.health -= 1;
    }

    fn update_points_count(&mut self, points_count :u32) {
        self.points_count += points_count;
    }

    /**
     * Add one to the number
     */
    fn add_step(&mut self) {
        self.steps += 1;
    }

    /**
     * Add to shoot action count
     */
    fn add_shoot_action(&mut self) {
        self.shot_actions_taken += 1;
    }

    /**
     * Add to drive action count.
     */
    fn add_drive_action(&mut self) {
        self.drive_actions_taken += 1;
    }

    fn add_skip_action(&mut self) {
        self.skip_actions_taken += 1;
    }

    fn corner_found(&mut self) {
        self.found_corner = true;
    }

    /**
     * Update player facing direction.
     */
    fn update_facing_direction(&mut self, direction: &str) {
        self.facing_directon = direction.to_string();
    }

    /**
     * Add to scan action.
     */
    fn add_scan_action(&mut self) {
        self.scan_actions_taken += 1;
    }

    /**
     * Returns player's facing direction.
     */
    fn get_facing_direction(&mut self) -> String {
        return self.facing_directon.to_string();
    }

    /**
     * Function responsible to further parse information returned by scan: abc defgh ijk
     * ''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''
     *  If there is an enemy represented by [R, O, Y, G, B, V] the string will be:
     *     - abR defgh ijY -> (Two enemies)
     *  If there is a wall it will have a 'W' char:
     *     - abW defWW ijW
     *  This functions will stract informations from the scan and return a scan_response struct
     *
     * In here we have to look at the direciton we're facing and convert position indices to moves.
     *
     *
     */
    fn construct_possible_moves(&mut self, scanned_area: &mut ScanResponse) -> Vec<String> {
        let mut possible_moves: Vec<String> = Vec::new();
        // we start with if an enemy has been detected
        if scanned_area.get_enemy_detected() {
            if self.round_number > self.max_exploration_rounds {
                if self.shot_actions_taken > 0{
                    for enemy_pos in scanned_area.get_enemies() {
                        possible_moves.push(construct_shoot_move(
                            self.facing_directon.clone(),
                            enemy_pos,
                        ));
                    }
                }
            }
        } else {
            if self.scan_actions_taken < self.scan_actions_taken {
                possible_moves.push(String::from("SCAN"));
            }
            possible_moves.extend(construct_drive_moves(
                self.facing_directon.clone(),
                scanned_area,
            ));
        }
        return possible_moves;
    }

    /**
     * Get player's color
     */
    fn get_colour(&self) -> String {
        self.colour.clone()
    }

    /**
     * Get side len of board
     */
    fn get_side_len(&self) -> u32 {
        self.side_length
    }

    /**
     * Get the round number.
     */
    fn get_rounds(&self) -> u32 {
        return self.round_number;
    }

    /**
     * Get the number of exploration round.
     */
    fn get_exploration_rounds(&self) -> u32 {
        return self.max_exploration_rounds;
    }

    /**
     * Check if player can drive in current round.
     */
    fn get_drive_actions_check(&self) -> bool {
        let mut drive_check = true;

        if (self.drive_actions_taken >= self.max_move) || ( self.get_points_left() <= 0){
            drive_check = false;
        }

        return drive_check;
    }

    /**
     * Check if player can scan in current round.
     */
    fn get_scan_actions_check(&self) -> bool {
        let mut scan_check = true;

        if (self.scan_actions_taken >= self.max_scans) || ( self.get_points_left() <= 0){
            scan_check = false
        }
        
        return scan_check;
    }

    fn get_scan_action_count(&self) -> u32 {
        return self.scan_actions_taken;
    }

    fn get_tank_type(&self) ->tank_type {
        return self.tank_type;
    }

    /**
     * Check if player can skip in current round.
     */
    fn get_skip_actions_check(&self) -> bool {
        
        return match self.drive_actions_taken {
            1 => {
                false
            },
            _ =>{
                true
            }
        };
    }

    /**
     * Check if player can shoot in current round.
     */
    fn get_shoot_action_check(&self) -> bool {

        let mut shoot_check = true;

        if (self.shot_actions_taken >= self.max_shoots) || ( self.get_points_left() <= 0){
            shoot_check = false;
        }
        
        return shoot_check;
    }

    /**
     * check player's point left, each player has a limit of points to be used in a round.
     *   - Ex.: The elevation of a terrain will add to the number of points an move action take.
     */
    fn get_points_left(&self) -> i32 {
        return  (self.total_possible_points - self.points_count) as i32;
    }

    /**
     * Each player has 3 steps in a round. That is, each player has the possibility to take 3 actions (steps) in a round.
     */
    fn get_step_count(&self) ->u32 {
        return self.steps;
    }

    fn get_corner_status(&self)-> bool {
        return self.found_corner;
    }
}

#[cfg(test)]
mod unit_test {
    use crate::{player::*, scan_parser};

    #[test]
    fn player_actions_checker(){
        let mut player: Player = Player::initialize_player("R".to_string(), 100 as u32, 9, tank_type::tank);
        player.add_drive_action();
        assert_eq!(
            true, player.get_drive_actions_check(),
            "Player moved only once, tank should be able to move 2 more times."
        );
        player.add_drive_action();
        player.add_drive_action();
        assert_eq!(
            false, player.get_drive_actions_check(),
            "Player tank moved 3 times, it shoundn't be allowed to move again."
        );
    }

    #[test]
    fn player_points_count(){
        let mut player: Player = Player::initialize_player("R".to_string(), 100 as u32, 9, tank_type::tank);
        player.update_points_count(2);

        assert_eq!(
            2, player.get_points_left(),
            "Player tank has 4 points in total, it should have 2 points left but it actually have {} left.", player.get_points_left()
        );

        player.update_points_count(2);

        assert_eq!(
            0, player.get_points_left(),
            "Player tank has 4 points in total, it should have 0 points left but it actually have {} left.", player.get_points_left()
        );
    }

    // Test initialization round number.
    #[test]
    fn initialization_round_number() {
        let player: Player = Player::initialize_player("R".to_string(), 100 as u32, 9, tank_type::tank);
        assert_eq!(
            1, player.round_number,
            "Player round number on initialization should be 1."
        );
    }

    // Test initialization points.
    #[test]
    fn test_set_player_initial_information() {
        let mut player: Player = Player::initialize_player("R".to_string(), 100 as u32, 10, tank_type::tank);
        player.set_initial_information("Y".to_string(), 200 as u32, 400);
        assert_eq!(
            "Y".to_string(),
            player.colour,
            "Player color information was set to 'Y' but got {}instead",
            player.colour
        );
    }

    #[test]
    fn initialization_total_points() {
        let player: Player = Player::initialize_player("R".to_string(), 100 as u32, 8, tank_type::tank);
        assert_eq!(4, player.total_possible_points, "Player as tank type should be initialized with 4 total poiunts but it was initialized with {}.", player.total_possible_points);
    }

    // Test drive actions taken at initialization.
    #[test]
    fn initialization_drive_actions() {
        let player: Player = Player::initialize_player("R".to_string(), 100 as u32, 7, tank_type::tank);
        assert_eq!(
            0, player.drive_actions_taken,
            "Player should start with 0 drive actions taken."
        );
    }

    // Test scan actions at initialization.
    #[test]
    fn initialization_scan_actions() {
        let player: Player = Player::initialize_player("R".to_string(), 100 as u32, 5, tank_type::tank);
        assert_eq!(
            0, player.scan_actions_taken,
            "Player should start with 0 scan actions taken."
        );
    }

    // Test initialization shot.
    #[test]
    fn initialization_shot() {
        let player: Player = Player::initialize_player("R".to_string(), 100 as u32, 10, tank_type::tank);
        assert_eq!(
            0, player.shot_actions_taken,
            "Player hasn't shoot in the first round, shot_this_round should be false."
        );
    }

    // Test initialization health.
    #[test]
    fn initialization_health() {
        let player: Player = Player::initialize_player("R".to_string(), 100 as u32, 8, tank_type::tank);
        assert_eq!(
            2, player.health,
            "Player initial health should be 2 at initialization."
        );
    }

    // Test initialization facing direction.
    #[test]
    fn initialization_facing_direction() {
        let player: Player = Player::initialize_player("R".to_string(), 100 as u32, 78, tank_type::tank);
        assert_eq!(
            "N".to_string(),
            player.facing_directon,
            "Player facing direction on start should be North."
        );
    }

    // Test taking damage.
    #[test]
    fn get_health_test() {
        let mut player: Player = Player::initialize_player("R".to_string(), 100 as u32, 76, tank_type::tank);
        assert_eq!(
            2,
            player.get_health(),
            "The initial health of player should be 2 but got {} uinstead",
            player.get_health()
        );
    }

    #[test]
    fn take_damage() {
        let mut player: Player = Player::initialize_player("R".to_string(), 100 as u32, 65, tank_type::tank);
        player.take_damage();
        assert_eq!(
            1, player.health,
            "Player starts with health = 2, taking damager should remove 1."
        );
    }

    // Test taking a step for the round, should increase the number of step taken by +1.
    #[test]
    fn restart_round() {
        let mut player: Player = Player::initialize_player("R".to_string(), 100 as u32, 55, tank_type::tank);
        player.add_step();
        player.start_round(1);
        assert_eq!(
            0, player.get_step_count(),
            "Player step should start as 3 and decreasing by one on take_step."
        );
    }

    // Test shooting, should flip the shooting switch to true.
    #[test]
    fn shoot() {
        let mut player: Player = Player::initialize_player("R".to_string(), 100 as u32, 45, tank_type::tank);
        player.add_shoot_action();
        assert_eq!(
            false, player.get_shoot_action_check(),
            "After a shot action, the player.shot_this_round should be updated to true."
        );
    }

    // Test performing a scan action, should update the number of action taken to +1.
    #[test]
    fn add_scan_action() {
        let mut player: Player = Player::initialize_player("R".to_string(), 100 as u32, 12, tank_type::tank);
        player.add_scan_action();
        assert_eq!(
            1, player.scan_actions_taken,
            "After scaning, pleyer.scan_actions_taken should be updated to 1."
        );
    }

    // Test drive action, should increase the drive action count by +1.
    #[test]
    fn test_add_drive_action() {
        let mut player: Player = Player::initialize_player("R".to_string(), 100 as u32, 65, tank_type::tank);
        player.add_drive_action();
        assert_eq!(
            1, player.drive_actions_taken,
            "After a drive/move action, the player.drive_actions_taken should be updated to 1."
        );
    }

    // Test changing the facing direction.
    #[test]
    fn test_update_facing_direction_test() {
        let mut player: Player = Player::initialize_player("R".to_string(), 100 as u32, 79, tank_type::tank);
        player.update_facing_direction("S");
        assert_eq!(
            "S".to_string(),
            player.facing_directon,
            "Updating facing direction to S has failed."
        );
    }

    // Test shooting an enemy on the 'a' position of the scan struct.
    #[test]
    fn test_construct_shoot_one_enemy() {
        let mut player: Player = Player::initialize_player("G".to_string(), 20 as u32, 56, tank_type::tank);
        //test all viable
        //11 walls                                          1               2             3              4              5              6              7              8              9              10             11
        let scan_string: Vec<String> = vec![
            "R".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
            "f".to_string(),
            "g".to_string(),
            "h".to_string(),
            "i".to_string(),
            "j".to_string(),
            "k".to_string(),
        ];
        let mut response: scan_parser::ScanResponse = ScanResponse::initialize_scan_response();
        response.scan_entry(scan_string);

        let enemy_pos: usize = *response.get_enemies().get(0).unwrap();

        let shot_output: String = construct_shoot_move(player.facing_directon.clone(), enemy_pos);

        assert_eq!("SHOOT NW", shot_output, "With player facing north and an enemy on position 'a' of the scan, the shoot response should be NW.");
    }

    // Test shooting an enemy on the g position.
    #[test]
    fn test_shoot_enemy_outerside_range() {
        let mut player: Player = Player::initialize_player("V".to_string(), 20 as u32, 9, tank_type::tank);
        //test all viable
        //11 walls                                          1               2             3              4              5              6              7              8              9              10             11
        let scan_string: Vec<String> = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
            "f".to_string(),
            "R".to_string(),
            "h".to_string(),
            "i".to_string(),
            "j".to_string(),
            "k".to_string(),
        ];
        let mut response: scan_parser::ScanResponse = ScanResponse::initialize_scan_response();

        response.scan_entry(scan_string);

        let enemy_pos: usize = *response.get_enemies().get(0).unwrap();

        let shot_output: String = construct_shoot_move(player.facing_directon.clone(), enemy_pos);

        assert_eq!("SHOOT N-NE", shot_output, "With player facing north and an enemy on position 'g' of the scan, the shoot response should be N-NE, received {}.", shot_output);
    }
}
