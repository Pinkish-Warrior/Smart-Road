#![windows_subsystem = "windows"] // From the druid docs: "By default, Windows will open a console with your application’s window. If you don’t want the console to be shown, use #![windows_subsystem = "windows"] at the beginning of your crate."

mod cars;
mod lanes;
mod sim;
mod stats;
mod textures;
mod trees;
mod types;

use crate::cars::Traffic;

fn main() {
    let mut traffic = Traffic::new();
    sim::simulate(&mut traffic);
    let s = traffic.format();
    stats::show(&s);
}
