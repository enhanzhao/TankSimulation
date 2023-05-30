use lib::{self, player::{Player, PlayerState, tank_type}, scan_parser::{ScanResponse, GetScanReturn}, move_manager, parser};

#[test]
fn player_basic_commands(){
    let mut player :Player = Player::initialize_player("R".to_string(), 100 as u32, 7, tank_type::tank);

    let mut scan :ScanResponse = ScanResponse::initialize_scan_response();
    let mut scan_string: Vec<String> = parser::get_args("abcdefghijk".to_string());
    

    player.update_facing_direction("NE");
    scan.scan_entry(scan_string);
    let mut possible_moves :Vec<String> = move_manager::construct_drive_moves(player.get_facing_direction(), &mut scan);

    assert_eq!(possible_moves.get(0).unwrap(), "DRIVE N", "With player facing NE the first first possible index on position 'a' should be N.");

    player.update_facing_direction("S");
    scan_string = parser::get_args("abWdefWWijW".to_string());

    scan.scan_entry(scan_string);

    possible_moves = move_manager::construct_drive_moves(player.get_facing_direction(), &mut scan);

    assert_eq!(possible_moves.len(), 2, "The scanned entry had only two possible moves but {} moves was returned.", possible_moves.len());
    assert_eq!(possible_moves.get(1).unwrap(), "DRIVE S", "The second possible move on position 'b' should be DRIVE S but found {}.", possible_moves.get(1).unwrap());
}

#[test]
fn player_scanning_walls(){
    let mut player :Player = Player::initialize_player("R".to_string(), 100 as u32, 7, tank_type::tank);

    let mut scan :ScanResponse = ScanResponse::initialize_scan_response();
    let scan_string: Vec<String> = parser::get_args("WWWWWWWWWWW".to_string());

    scan.scan_entry(scan_string);
    let possible_moves :Vec<String> = move_manager::construct_drive_moves(player.get_facing_direction(), &mut scan);
    assert_eq!(possible_moves.len(), 0, "All the scan entries are walls and therefore the possible moves should be empty but it has length of {}",possible_moves.len());
}