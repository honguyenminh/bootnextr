// declare modules into compilation tree
mod cli;
mod platform;
mod entities;

use std::io::{stderr, stdout, Write};
use std::process::ExitCode;
use clap::Parser;
use nameof::name_of;
use tabled::settings::object::{Columns};
use tabled::settings::{Remove, Style, Width};
use tabled::settings::location::ByColumnName;
use tabled::Table;
use crate::cli::args::Args;
use crate::cli::{flush_stdout, get_stdin_confirm, get_stdin_number};
use crate::entities::BootEntry;
use crate::platform::{get_boot_entries, set_boot_next, ensure_permission, restart};

// TODO: standardize exit error codes
// TODO: split this shit out.
fn main() -> ExitCode {
    let args = Args::parse();

    // cant chain this bc rust is dumb
    let search_keyword = &args.search_keyword;
    let is_output_only = search_keyword.is_none();

    let valid_permission = ensure_permission(is_output_only);
    if !valid_permission {
        eprintln!("You must run this executable with root permissions. Try appending sudo.");
        return ExitCode::from(1);
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

    if is_output_only { return ExitCode::SUCCESS }

    let mut entry_matches: Vec<&BootEntry> = vec![];
    // match search_keyword with entries, fulltext cuz im lazy
    // unwrap into another var, cuz again, rust
    let keyword = args.search_keyword.unwrap().to_lowercase();
    for entry in &entries {
        if !args.allow_inactive && !entry.is_active {
            continue;
        }
        if !entry.description.to_lowercase().contains(keyword.as_str()) {
            continue;
        }
        entry_matches.push(&entry);
        if args.force_first {
            break;
        }
    }

    if entry_matches.is_empty() {
        eprintln!("No matches boot entry found for keyword '{}'", keyword);
        return ExitCode::from(3);
    }

    let target_entry: &BootEntry =
        if args.force_first || entry_matches.len() == 1 {
            entry_matches[0]
        } else {
            println!("Found multiple matching boot entries to keyword '{}'", keyword);
            for (i, entry) in entry_matches.iter().enumerate() {
                let inactive_slug = if entry.is_active {""} else {"(inactive) "};
                println!("[{}] {}{} - {}", i, inactive_slug, entry.id, entry.description);
            }
            // get user input
            print!("Pick an entry to boot (with the index x in [x]): ");
            stdout().flush().unwrap();

            let i = get_stdin_number();
            if i.is_err() {
                eprintln!("Invalid input, not a number.");
                return ExitCode::from(2);
            }
            let i = i.unwrap();
            if i >= entry_matches.len() {
                eprintln!("Index out of range, input the number in the square brackets like this -> [2]");
                return ExitCode::from(2);
            }
            entry_matches[i]
        };

    let output = set_boot_next(&entries[0]);

    if !output.status.success() {
        stdout().write(output.stdout.as_slice()).unwrap();
        stderr().write(output.stderr.as_slice()).unwrap();
        eprintln!("ERROR: Cannot set BootNext to {} ({})!", target_entry.id, target_entry.description);
        return ExitCode::from(3);
    }

    println!("Set BootNext to {} ({}) successfully", target_entry.id, target_entry.description);

    // restart
    if args.restart {
       restart();
    }
    print!("Restart now? (Tips: use -r to always restart) [Y/n]: ");
    flush_stdout();
    let confirm = get_stdin_confirm(Some(true));
    if confirm {
        restart()
    } else {
        println!("No restart.")
    }

    ExitCode::SUCCESS
}
