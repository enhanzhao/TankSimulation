use crate::scan_parser::{ScanResponse, GetScanReturn};

pub struct board {
    board: Vec<Vec<Vec<Point>>>,
    teammates: Vec<Point>,
    sidelen: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    q: i32,
    r: i32,
    s: i32,
    elevation: i32,
    occupied_by: char
}

pub trait board_operations {
    fn initialize(side_len: i32) -> Self;
    fn find_path(source: Point,destination: Point) -> String;
    fn dist(p1: Point, p2: Point) -> u32;

    fn update_board(&mut self, scan:&mut ScanResponse);
}

impl board_operations for board {

    /**
     * Initialize a board with a colection of empty points.
     *   - Empty points has q, r, s = -1.
     * 
     * It will add the point that the player has calibrated by adding the points 'q', 'r', 's'
     * 
     *  - q: the q coordinate.
     *  - r: the r coordinate.
     *  - s: the s coordinate.
     *  - side_len: The lenght of one side of the board.
     * 
     *  - Returns: An initialized version of the board.
     * 
     * //////////////////////////////////////////////////////////////////////////////////////
     * // For better comprehension of the board visit this website:
     * // https://www.redblobgames.com/grids/hexagons/#line-drawing
     * /////////////////////////////////////////////////////////////////////////////////////
     */
    fn initialize(side_len: i32) -> Self {

        let empty_point: Point = Point { 
            q: -1, 
            r: -1, 
            s: -1, 
            elevation: -1, 
            occupied_by: ' '
        };

        // let new_point: Point = Point { 
        //     q: q, 
        //     r: r, 
        //     s: s, 
        //     elevation: -1, 
        //     occupied_by: ' '
        // };

        let array_len: usize = ((side_len * 2) - 1) as usize;

        
        let mut board: Vec<Vec<Vec<Point>>> = vec![vec![vec![empty_point; array_len]; array_len]; array_len];
        //board[q as usize][r as usize][s as usize] = new_point;

        
        Self { board: board, 
            teammates: Vec::new(), 
            sidelen:  side_len}
    }

    fn find_path(source: Point,destination: Point) -> String {
        todo!();
    }

    fn dist(p1: Point, p2: Point) -> u32 {
        todo!()
    }

    /**
     * Add the scan response to the board. The board may have negative coordinates but in the array representation it can't. 
     * ////////////////////////////////////////////////////////////////////////////////////////////
     * // Imagene a board with side len = 6.
     * //   - The coordinated will range from -5 to +5 => -(side len -1) to (side len -1)
     * //
     * //  => When q = -5 in the array its position will be on index 0 => [-5 + (side len -1)].
     * //  => When q = 5 in the array its position will be on index 10 => [5 + (side len -1)].
     * // 
     * // This hold true for the oder coordinates 'r' and 's'.
     * //
     * /////////////////////////////////////////////////////////////////////////////////////////////
     * // For better comprehension of the board visit this website:
     * // https://www.redblobgames.com/grids/hexagons/#line-drawing
     * /////////////////////////////////////////////////////////////////////////////////////////////
     * 
     * The board will add the points returned from a scan and the indexing order will be [q][r][s].
     */
    fn update_board(&mut self, scan: &mut ScanResponse) {
        for points in scan.get_scanned_positions().iter(){

            let q: usize = (points.q + (self.sidelen-1)) as usize;
            let r: usize = (points.r + (self.sidelen-1)) as usize;
            let s: usize = (points.s + (self.sidelen-1)) as usize;

            self.board[q][r][s] = points.clone();
            
        }
    }
}
pub trait Point_operations {
    fn new(p: usize, occupied_by: char) -> Point;
}

impl Point {

    /**
     * Initialize a point based on the position of the character in the vector returned by scan response.
     * 
     *  - p: The position of the entry in the scan respoonse.
     *  - occupied_by: Check what that tile contains, it could be:
     *          - terrain type.
     *          - player.
     *  - is_scout: a flag to determine if the player is of scout type, since its scan is bigger than the others.
     */
    pub fn new(p: usize, occupied_by: char, is_scout: bool) -> Point {
        let mut new_tile = Point {
            q: 0,
            r: 0,
            s: 0,
            elevation: -1, // -1 Indicates unset elevation
            occupied_by
        };

        let mut q:i32 = 0;
        let mut r:i32 = 0;
        let pos:i32 = p as i32;

        // Really bad way to do this.  Is there a formula to map index position to a relative position?
        if pos < 3 { // abc
            q = (pos - 1) as i32;
            if pos == 0 {
                r = 0;
            } else {
                r = -1;
            }
        } else if pos >= 3 && pos < 8 { //efg
            q = (pos - 6) as i32;
            if pos == 3 {
                r = 0;
            } else if pos == 4 {
                r = -1;
            } else {
                r = -2;
            }
        }
        // abc efghi jkl : standard scan
        // abc efghi jklmn opq

        // Handle if scout or not
        if is_scout {
            q = (pos - 12);
            if pos > 7 {
                if pos == 8 {
                    r = -1;
                } else if pos == 9 {
                    r = -2;
                } else {
                    r = -3;
                }
            } else if pos > 12 {
                q = pos - 14;
                if pos == 13 {
                    r = -3;
                } else {
                    r = -4;
                }
            }
        } else { // Is a standard scan
            if pos > 8 {
                q = pos - 9;
                if pos == 9 {
                    r = -2;
                } else {
                    r = -3;
                }
            }
        }

        new_tile.q = q;
        new_tile.r = r;
        new_tile.s = -q - r;

        new_tile.occupied_by = occupied_by;

        return new_tile;
    }

    /// Translates a given point by displacement values
    /// dq: Displacement in the q direction
    /// dr: Displacement in the r direction
    /// ds: Displacement in the s direction
    pub fn translate(&mut self, dq: i32, dr: i32, ds: i32)  {
        self.q = self.q + dq;
        self.r = self.r + dr;
        self.s = self.s + ds;
    }

    /// Rotates a point/vector where a rotation is changing the axis.  e.g. one rotation of a north-facing vector CW results in a NE-facing vector
    /// Implemented rotation as defined here:
    /// https://www.redblobgames.com/grids/hexagons/#rotation
    pub fn rotate(&mut self, rotations: i32) {
        if rotations > 0  {
            for _i in 0..rotations {
                let tmp = self.s;
                self.s = self.q;
                self.q = self.r;
                self.r = tmp;
            }
        } else {
            for _i in 0..-rotations {
                let tmp = self.r;
                self.r = self.q;
                self.q = self.s;
                self.s = tmp;
            }
        }

        let mut sign = -1 * rotations.abs() % 2;
        if sign == 0 { sign = 1; }

        self.r = self.r * sign;
        self.q = self.q * sign;
        self.s = self.s * sign;
    }

    fn find_path(&mut self, destination: Point) {
        todo!()
    }
}


// Is there a rule way to deserialize
