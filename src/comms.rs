use std::{fs::File, os::unix::prelude::FromRawFd, io::{Read, Write}, sync::mpsc::{Receiver, Sender, self}};
use crate::{parser, player::Player};

// /// comm scan struct
// /// Used to send and receive scan information between robots
// /// to sychronize internal maps
// #[derive(serde::Serialize, serde::Deserialize)]
// pub struct CommScan
// {
//     scan: Vec<String>
// }

/// internal function, called by the listener thread
/// Consumes scanned information from other robots
fn put_scan(s:String, player: &mut Player)
{
    let scan: CommScan = serde_json::from_str(&s).unwrap();
    // sca.scan is the vector of a scan from another player
    eprintln!("{:?}", scan.scan);
    // TODO: use received scan: put the information into player
}

pub fn parse_thread_comms(in_string: String, player: &mut Player)
{
    let vec_in = parser::get_args(in_string);
    match vec_in[0].as_str()
    {
        "SCAN" => put_scan(vec_in[1].clone(), player),
        _ => () // bad input
    }
}

/// Starts a listener thread. Returns the thread so that it can
/// be joined later before robot stops executing.
/// How to use: Detailed instructions in the test called: start_listener_thread_test
pub fn start_listener_thread(tx: Sender<String>, end_rx: Receiver<String>) -> std::thread::JoinHandle<()>
{
    // team listens to channel 3
    let input_fd = 3;
    let comm_thread = std::thread::spawn(move || {
        let mut fd_in = unsafe { File::from_raw_fd(input_fd) };
        loop
        {
            let mut input = String::new();
            fd_in.read_to_string(&mut input).unwrap();
            let in_vec: Vec<&str> = input.split('\n').collect();

            for str in in_vec 
            {
                if str != ""
                {
                    tx.send(str.to_string()).unwrap();
                }
            }

            match end_rx.try_recv()
            {
                Ok(_) | Err(mpsc::TryRecvError::Disconnected) => {
                    break;
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
    });

//     comm_thread
// }

// /// Internal function, used to write some string to some file descriptor
// fn write_to_fd(output_string: String, output_fd: i32)
// {
//     let mut out_fd = unsafe { File::from_raw_fd(output_fd) };
//     write!(&mut out_fd, "{}", output_string).unwrap();
// }

// /// Used to send scan information to other robots.
// pub fn comm_send_scan(scan_vector: Vec<String>)
// {
//     // team outputs to channel 4
//     let out_fd_num = 4;

//     let out_struct = CommScan{ scan:scan_vector };
//     let serialized = serde_json::to_string(&out_struct).unwrap();

    let out = "SCAN ".to_string() + &serialized + "\n";
    write_to_fd(out, out_fd_num);
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn comm_send_scan_tests()
//     {
//         // TEST FAILS UNLESS YOU USE COMMANDLINE: cargo test 4>{anything you want}
//         let scan_vector = vec!["a".to_string(), "b".to_string(), "c".to_string()];
//         comm_send_scan(scan_vector);
//     }

    #[test]
    fn listener_decisions_test()
    {
        let example_str = "SCAN {\"scan\":[\"a\",\"b\",\"c\"]}".to_string();
        let mut player = <Player as crate::player::PlayerState>::initialize_player("V".to_string(), 20 as u32, 9, crate::player::tank_type::tank);
        parse_thread_comms(example_str, &mut player);
    }

    #[test]
    fn start_listener_thread_test()
    {
        // TEST FAILS UNLESS YOU USE COMMANDLINE: cargo test 3<{something that has expected input}

        // start by making 2 sets of transmitter/receiver pairs. One communicates the information from other tanks
        // other one is for turning off the thread (end_tx/end_rx).
        // do this when initializing the tank in the very beginning one time.
        let (tx, rx) = mpsc::channel();
        let (end_tx, end_rx) = mpsc::channel();
        // create the thread with the transmitter/receiver pairs passed in
        let x = start_listener_thread(tx, end_rx);
        // create a try_iter(has to be try_iter). It receives info from thread
        let mut iter = rx.try_iter();
        // done with tank initialization stuff

        // this line is to simulate server lag. Ignore for implementations
        std::thread::sleep(std::time::Duration::from_millis(100));

        // creating a dummy player (needs to be passed into function: parse_thread_comms)
        let mut player = <Player as crate::player::PlayerState>::initialize_player("V".to_string(), 20 as u32, 9, crate::player::tank_type::tank);        

        // this loop goes through all the messages in transmission line.
        // this loop (or a similar loop) should be called beginning of every round.
        // do this beginning of each turn
        loop {
            match iter.next() {
                Some(s) => {
                    parse_thread_comms(s, &mut player);
                },
                None => {
                    break;
                }
            }   
        }
        // stop here and do the rest of the turn then go back to start of loop on line 131 beginning of next turn

        // do this at the very end when tank is dead or game is over
        let _ = end_tx.send("Done!".to_string());
        x.join().unwrap();
    }

}