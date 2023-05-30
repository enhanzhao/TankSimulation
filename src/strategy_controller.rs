use crate::{player::{Player, PlayerState}, action_manager::{playerOutput, action_manager, manage_player_action}};

pub enum strategiesType{
    find_corner,
    exlporer,
    traverse,
    none
}

pub struct startegies{
    find_corner_status: bool,
    explorer_status: bool,
    traverse_status: bool
}

pub trait strategy_controller{
    fn initialize_initial_startegy() -> Self;
    fn set_active_strategy(&mut self, strategy :strategiesType);
    fn get_active_strategy(&mut self) -> strategiesType;
    fn avaliate_startegy(&mut self, player :&mut Player, action_manager :&mut action_manager);
}

impl strategy_controller for startegies{

    /**
     * Set initial strategy to find corner
     */
    fn initialize_initial_startegy() -> Self{
        return Self{
            find_corner_status: true,
            explorer_status: false,
            traverse_status: false
        };
    }

    /**
     * Set active strategy. By setting a new one, the old strategy should be turned off.
     */
    fn set_active_strategy(&mut self, strategy :strategiesType){
        match strategy {
            strategiesType::find_corner =>{
                self.find_corner_status = true;
                self.explorer_status = false;
                self.traverse_status = false;
            },
            strategiesType::exlporer => {
                self.explorer_status = true;
                self.find_corner_status = false;
                self.traverse_status = false;
            }
            strategiesType::traverse => {
                self.traverse_status = true;
                self.explorer_status = false;
                self.find_corner_status = false;
            }
            _ => ()
        }
    }

    /**
     * Return the current active strategy
     */
    fn get_active_strategy(&mut self) -> strategiesType{

        if self.traverse_status{
            return strategiesType::traverse;
        } else if self.explorer_status{
            return strategiesType::exlporer;
        }else if self.find_corner_status {
            return strategiesType::find_corner;
        }else{
            return strategiesType::none;
        }
    }

    /**
     * Check if the robot should change to another strategy.
     */
    fn avaliate_startegy(&mut self, player :&mut Player, action_manager :&mut action_manager){
        let round_num = player.get_rounds();
        let max_exploration_round = player.get_exploration_rounds();

        if !player.get_corner_status(){
            self.set_active_strategy(strategiesType::find_corner);

        }else if round_num < max_exploration_round{
            self.set_active_strategy(strategiesType::exlporer);

        }else {
            self.set_active_strategy(strategiesType::traverse);
        }
    }

}
