use jacdac_rs::{brain::brain::Brain, transport::frame::Frame};

fn main() {
    let str_buffer = [
        "2AA10C00BEE3269A73CAE807080000001F03000063A2731400", // Services report from button id '07E8CA739A26E3BE' named RB71
        "C4620400BEE3269A73CAE807000101D500",                 // Event report "button pressed"
    ];

    let mut brain = Brain::new(|| 0u64);

    println!("Brain: {:#?}", brain);

    for string in str_buffer {
        let buffer: Vec<u8> = match str_to_buff(string) {
            Some(v) => v,
            None => {
                eprintln!("Failed to parse string...");
                return;
            }
        };

        let frame = match Frame::from_buffer(&buffer) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Frame error: {:?}", e);
                return;
            }
        };

        match brain.handle_frame(frame) {
            Ok(_) => (),
            Err(e) => eprintln!("Failed to handle frame: {:?}", e),
        }
    }

    println!("Brain: {:#?}", brain);
}

fn str_to_buff(s: &str) -> Option<Vec<u8>> {
    let mut result = Vec::with_capacity(s.len() / 2);

    for i in (0..(s.len())).step_by(2) {
        if i + 1 >= s.len() {
            break;
        }

        let hex = u8::from_str_radix(&s[i..(i + 2)], 16).ok()?;
        result.push(hex);
    }

    Some(result)
}
