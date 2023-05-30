enum TeamColor {
    R,
    O,
    Y,
    G,
    B,
    V,
}
struct EnemiesPos {
    // Position := (x_pos,y_pos,z_pos)
    lastseen_x_pos: i32,
    lastseen_y_pos: i32,
    lastseen_z_pos: i32,
    team: TeamColor,
    seen_at_round: u32,
}

impl EnemiesPos {
    pub fn new(
        relative_x: i32,
        relative_y: i32,
        relative_z: i32,
        team: TeamColor,
        seen_at_round: u32,
    ) -> Self {
        Self {
            lastseen_x_pos: relative_x,
            lastseen_y_pos: relative_y,
            lastseen_z_pos: relative_z,
            team: team,
            seen_at_round: seen_at_round,
        }
    }
}
