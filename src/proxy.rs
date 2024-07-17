use midir::{Ignore, MidiInput};
use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::net::{ToSocketAddrs, UdpSocket};
use std::time::Duration;
use std::{io, str};
const PORT_NAME: &str = "midi-over-udp";
pub fn start(port: usize, to: &str) -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut midi_in = MidiInput::new("midi2udp")?;
    midi_in.ignore(Ignore::None);
    let in_ports = midi_in.ports();
    let in_port = in_ports.get(port).ok_or("invalid input port selected")?;
    let in_port_name = midi_in.port_name(in_port)?;
    let socket = UdpSocket::bind("0.0.0.0:0").expect("failed to bind socket");
    println!("MIDI Data Forwarding to: {}", to);
    let to = to.to_string();
    let _conn_in = midi_in.connect(
        in_port,
        "midi2udp",
        move |stamp, message, _| {
            println!("{}: {:?} (len = {})", stamp, message, message.len());
            socket
                .send_to(message, to.as_str())
                .expect("failed to send to socket");
        },
        (),
    )?;

    println!(
        "Connection open,  listening on {}' (press enter to exit) ...",
        in_port_name
    );

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Exit MIDI Proxy");
    return Ok(());
}
