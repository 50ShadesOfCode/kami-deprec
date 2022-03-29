extern crate clap;
use clap::{crate_version, App};

fn main() {
    let matches = App::new("kami")
        .version(crate_version!())
        .about("This is about coding")
        .author("50ShadesOfCode <kaktysh20040130@gmail.com>")
        .get_matches();
    println!("..")
}
