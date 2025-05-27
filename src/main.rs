// declare modules into compilation tree
mod cli;
mod platform;
mod entities;

use std::io::{stderr, stdout, Write};
use clap::Parser;
use cli::args::Args;
use crate::entities::BootEntry;
use crate::platform::{get_boot_entries, set_boot_next, ensure_permission};

fn main() {
    ensure_permission();
    let args = Args::parse();

    let entries = get_boot_entries();
    
    let target_entry: &BootEntry = &entries[0];
    
    
    let output = set_boot_next(&entries[0]);

    if !output.status.success() {
        stdout().write(output.stdout.as_slice()).unwrap();
        stderr().write(output.stderr.as_slice()).unwrap();
        eprintln!("ERROR: Cannot set BootNext to {} ({})!", target_entry.id, target_entry.description);
        return
    }
    
    println!("Set BootNext to {} ({}) successfully", target_entry.id, target_entry.description)
}
