//! Display pokemon sprites in your terminal.

use clap::Parser;
use image::imageops::FilterType;
use pokeget::cli::Args;
use pokeget::list::List;
use pokeget::pokemon::{Attributes, Pokemon};
use pokeget::sprites::combine_sprites;
use std::process::exit;
use std::net::TcpStream;
use std::net::SocketAddr;
use std::time::Duration;

fn main() {
    let list = List::read();
    let args = Args::parse();

    if args.pokemon.is_empty() {
        eprintln!("you must specify the pokemon you want to display");
        exit(1);
    }

    if args.socket_addr.is_empty() {
        eprintln!("you must specify the socket address you want to send data to. Use the form '127.0.0.1:51629'");
        exit(1);
    }

    let server_addr: SocketAddr = args.socket_addr
                                    .parse()
                                    .expect("Unable to parse Socket Address");

    let attributes = Attributes::new(&args);
    let pokemons: Vec<Pokemon> = args
        .pokemon
        .into_iter()
        .map(|x| Pokemon::new(x, &list, &attributes))
        .collect();

    let combined = combine_sprites(&pokemons);

    let combind_resized = combined.resize_exact(256, 192, FilterType::Nearest);

    if let Ok(stream) = TcpStream::connect_timeout(&server_addr, Duration::from_millis(500)) {
        println!("Successfully connected to provided server");
    } else {
        eprintln!("Failed to connect to provided server");
        exit(1);
    }

}
