use jacdac_rs::{service::packet::Packet, transport::frame::Frame};

fn main() {
    let str_buffer = "b55c08000c1b11ff66a11bed0401028ac8000000"; // Event report
                                                                 // let str_buffer = "7f9c0c000c1b11ff66a11bed080000002f01010063a27314"; // Services report from button id '0c1b11ff66a11bed' named RB71

    let buffer: Vec<u8> = match str_to_buff(str_buffer) {
        Some(v) => v,
        None => {
            eprintln!("Failed to parse str_buffer...");
            return;
        }
    };

    let frame = Frame::from_buffer(&buffer);

    println!("Result: {:#?}", frame);

    if let Ok(mut frame) = frame {
        let packet = frame.data.pop().unwrap();
        println!("Packet: {:#?}", Packet::from_transport(packet, frame.flag))
    }
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
