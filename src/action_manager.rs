use crate::player::{ Player, PlayerState};

#[derive(Debug, Clone, Copy)]
pub enum playerOutput{
    SCAN,
    TURN,
    END,
    SHOOT,
    SKIP,
    DRIVE,
    NONE
}

pub struct action_manager {
    last_action: playerOutput,
    round_action_counter: u32
}

pub trait manage_player_action{
    fn initialize() -> Self;
    fn get_last_action(&mut self) -> playerOutput;
    fn get_action_counter(&mut self) -> u32;
    fn reset_counter(&mut self);
    fn shoot(&mut self, direction: &str, player: &mut Player); // ======> should't be done at player's model, state machine should do the action and the update the model accordingly.
    fn turn(&mut self, direction: &str, player: &mut Player);  // ======> same
    fn drive(&mut self, player: &mut Player);                  // ======> same
    fn scan(&mut self, player: &mut Player);   // ======> same
    fn skip(&mut self, player: &mut Player);                   // ======> same
    fn end(&mut self);  
}

impl manage_player_action for action_manager{

    fn initialize() -> Self {
        Self { 
            last_action: playerOutput::NONE,
            round_action_counter: 0
        }
    }
    
    fn get_last_action(&mut self) -> playerOutput {
        return self.last_action;
    }

    fn reset_counter(&mut self) {
        self.round_action_counter = 0;
    }

    fn get_action_counter(&mut self) -> u32 {
        return self.round_action_counter;
    }

     /**
     * Send a shoot message to server.
     * direction: for example SHOOT N or SHOOT N-E
     */
    fn shoot(&mut self, direction: &str,  player: &mut Player) {
        player.add_shoot_action();
        player.add_step();
        println!("{}", direction);
        self.last_action = playerOutput::SHOOT;
        self.round_action_counter += 1;
    }

    /**
     * Send a turn message to the server
     * direction: for example N or NE
     */
    fn turn(&mut self, direction: &str,  player: &mut Player) {
        println!("TURN {} ", direction);
        player.update_facing_direction(direction);
        self.last_action = playerOutput::TURN;
        self.round_action_counter += 1;
    }

    /**
     * Send a drive action to the server
     */
    fn drive(&mut self,  player: &mut Player) {
        player.add_drive_action();
        player.add_step();
        println!("DRIVE");
        self.last_action = playerOutput::DRIVE;
        self.round_action_counter += 1;
    }

    /**
     * Send and receive a scan message to the server
     */
    fn scan(&mut self,  player: &mut Player) {
        player.add_scan_action();
        player.add_step();
        println!("SCAN");
        self.last_action = playerOutput::SCAN;
        self.round_action_counter += 1;
    }

    /**
     * Send a scan message to the server
     */
    fn skip(&mut self,  player: &mut Player) {
       player.add_skip_action();
       player.add_step();
        println!("SKIP");
        self.last_action = playerOutput::SKIP;
        self.round_action_counter += 1;
    }

    fn end(&mut self) {
        println!("END");
        self.last_action = playerOutput::END;
        self.round_action_counter += 1;
    }
}
