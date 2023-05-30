use crate::CoordSystem::Point;

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
pub struct ScanResponse {
    scanned_positions: Vec<Point>,
    enemies_pos: Vec<usize>, // Indicates which indexes the enemies are located at in the scan.
    walls_pos: Vec<usize>,  // Indicates which indexes the enemies are located at in the scan.
    enemy_detected: bool,
    wall_detected: bool,
}


pub trait GetScanReturn {
    fn initialize_scan_response() -> Self;
    fn scan_entry(&mut self, entry :Vec<String>);
    fn get_enemy_detected(&mut self) -> bool;
    fn get_wall_detected(&mut self) -> bool;
    fn get_walls(&mut self) -> Vec<usize>;
    fn get_enemies(&mut self) -> Vec<usize>;
    fn get_scanned_positions(&mut self) -> Vec<Point>;
}

impl GetScanReturn for ScanResponse {

    /**
     * Initialize the scan_response.
     */
    fn initialize_scan_response() -> Self {
        Self {
            scanned_positions: Vec::new(),
            enemies_pos: Vec::new(),
            walls_pos: Vec::new(),
            enemy_detected: false,
            wall_detected: false,
        }
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
     * ''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''
     *
     *  - scan_response: String returned by the server
     *
     *  @ return scan_response: Return a struct of scan_response type which contains information on each of the tiles from a to k.
     */
    fn scan_entry(&mut self, entry :Vec<String>){
        let enemy_letters: Vec<&str> = vec!["R", "O", "Y", "G", "B", "V"];


        let scan_string: String = entry.into_iter().collect();

        // We're looping through the string without spaces and pushing elements to the vector according to their index.
        // This should help us build a way to convert integers to moves.
        for (pos, char) in scan_string.chars().enumerate() {
            // TODO: fix hard-coded boolean check
            self.scanned_positions.push(Point::new(pos, char, false));

            if enemy_letters.iter().any(|x| x == &char.to_string()) {
                self.enemies_pos.push(pos);
                self.enemy_detected = true;
            }

            if char == 'W' {
                self.walls_pos.push(pos);
                self.wall_detected = true;
            }
        }
    }

    fn get_enemy_detected(&mut self) -> bool{
        return self.enemy_detected;
    }

    fn get_wall_detected(&mut self) -> bool{
        return self.wall_detected;
    }

    fn get_enemies(&mut self) -> Vec<usize>{
        return self.enemies_pos.clone();
    }

    fn get_walls(&mut self) -> Vec<usize>{
        return self.walls_pos.clone();
    }
    
    fn get_scanned_positions(&mut self) -> Vec<Point> {
        return self.scanned_positions.clone();
    }
}

#[cfg(test)]
mod unit_test{
    use super::{ScanResponse, GetScanReturn};


    #[test]
    fn test_scanning_no_enemies(){
        let mut scan :ScanResponse = ScanResponse::initialize_scan_response();
        let scan_string: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string(), "f".to_string(), "g".to_string(), "h".to_string(), "i".to_string(), "j".to_string(), "k".to_string()];

        scan.scan_entry(scan_string);

        assert_eq!(scan.get_enemy_detected(), false, "There wasn't supplied any enemies in the entry but the scan is returning {} enemies", scan.get_enemies().len());
    }

    #[test]
    fn test_scanning_no_walls(){
        let mut scan :ScanResponse = ScanResponse::initialize_scan_response();
        let scan_string: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string(), "f".to_string(), "g".to_string(), "h".to_string(), "i".to_string(), "j".to_string(), "k".to_string()];

        scan.scan_entry(scan_string);

        assert_eq!(scan.get_wall_detected(), false, "There wasn't supplied any walls in the entry but the scan is returning {} walls", scan.get_walls().len());
    }

    #[test]
    fn test_scanning_walls(){
        let mut scan :ScanResponse = ScanResponse::initialize_scan_response();
        let scan_string: Vec<String> = vec!["W".to_string(), "b".to_string(), "W".to_string(), "W".to_string(), "e".to_string(), "f".to_string(), "g".to_string(), "h".to_string(), "i".to_string(), "j".to_string(), "k".to_string()];

        scan.scan_entry(scan_string);
        assert_eq!(scan.get_walls().len(), 3, "There were 3 walls in the entry but {} was detected by the scan.", scan.get_walls().len());
    }

    #[test]
    fn test_scanning_enemies(){
        let mut scan :ScanResponse = ScanResponse::initialize_scan_response();
        let scan_string: Vec<String> = vec!["R".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string(), "f".to_string(), "g".to_string(), "h".to_string(), "i".to_string(), "j".to_string(), "k".to_string()];

        scan.scan_entry(scan_string);
        assert_eq!(scan.get_enemies().len(), 1, "There should be one enemy detected but {} was detected.", scan.get_enemies().len());
    }

    #[test]
    fn test_scanning_both(){
        let mut scan :ScanResponse = ScanResponse::initialize_scan_response();
        let scan_string: Vec<String> = vec!["R".to_string(), "W".to_string(), "W".to_string(), "d".to_string(), "W".to_string(), "f".to_string(), "g".to_string(), "h".to_string(), "i".to_string(), "j".to_string(), "k".to_string()];

        scan.scan_entry(scan_string);
        assert_eq!(scan.get_enemies().len(), 1, "There should be one enemy detected but {} was detected.", scan.get_enemies().len());
        assert_eq!(scan.get_walls().len(), 3, "There were 3 walls in the entry but {} was detected by the scan.", scan.get_walls().len());
    }
}