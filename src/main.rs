// declare modules into compilation tree
mod cli;
mod platform;
mod entities;

use std::io::{stderr, stdout, Write};
use clap::Parser;
use nameof::name_of;
use tabled::settings::object::{Columns};
use tabled::settings::{Remove, Style, Width};
use tabled::settings::location::ByColumnName;
use tabled::Table;
use crate::cli::args::Args;
use crate::entities::BootEntry;
use crate::platform::{get_boot_entries, set_boot_next, ensure_permission};

fn main() {
    let args = Args::parse();

    let is_output_only = args.search_keyword.is_none();
    let valid_permission = ensure_permission(is_output_only);
    if !valid_permission {
        eprintln!("You must run this executable with root permissions. Try appending sudo.");
        std::process::exit(1);
    }

    if is_output_only {
        println!("No search_example given, outputting all found entries.");
        println!("Add --help for more info.");
    }

    let entries = get_boot_entries();

    // output found entries
    let mut entry_table = Table::new(&entries);
    entry_table
        // remove platform_id column
        .with(Remove::column(ByColumnName::new(name_of!(platform_id in BootEntry))))
        // set max-width for boot_path cuz no one cares
        .modify(Columns::last(), Width::truncate(20))
        .with(Style::modern());

    println!("{}", &entry_table);

    if is_output_only { return }

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
