use clap::Parser;
use midir::{Ignore, MidiInput};
mod proxy;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// MIDI Input port
    #[arg(short,long,default_value_t=0)]
    port: usize,
    /// udp send target
    #[arg(long,default_value_t=String::from("localhost:8083"))]
    to: String,
    /// Show MIDI Ports
    #[arg(short, long)]
    list: bool,
}
fn main() {
    let args = Args::parse();
    
    if args.list {
        let mut midi_in = MidiInput::new("midir reading input").unwrap();
        midi_in.ignore(Ignore::None);
        let ports = midi_in.ports();
        if ports.is_empty() {
            println!("no input port found");
        } else {
            ports.iter().enumerate().for_each(|(i, port)| {
                println!("{}: {}", i, midi_in.port_name(port).unwrap());
            });
        }
    } else {
      match proxy::start(args.port,&args.to) {
        Ok(_) => {},
        Err(e) => println!("error: {}",e),
      }
    }
}
