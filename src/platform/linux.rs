use std::io::{stderr, stdout, Write};
use snafu::prelude::*;

use nix::unistd::Uid;
use std::process::{Command, Output};
use regex::Regex;
use crate::entities::BootEntry;

// public functions (contract)

pub fn ensure_permission() {
    if !Uid::effective().is_root() {
        eprintln!("You must run this executable with root permissions. Try appending sudo.");
        std::process::exit(1);
    }
}

pub fn get_boot_entries() -> Vec<BootEntry> {
    let output = Command::new("efibootmgr")
        .output()
        .expect("Failed to execute efibootmgr");

    let output_str = String::from_utf8_lossy(&output.stdout);

    parse_boot_entries(&output_str)
}

pub fn set_boot_next(entry: &BootEntry) -> Output {
    Command::new("efibootmgr")
        .args(["-n", entry.platform_id.as_str()])
        .output()
        .expect("Cannot run 'efibootmgr -n' command")
}

#[derive(Debug, Snafu)]
enum SetBootNextError {
    #[snafu(display("Unable to read configuration from {:?}", boot_entry))]
    InvalidBootEntry { boot_entry: BootEntry },
}

// private functions

fn parse_boot_entries(raw_output: &str) -> Vec<BootEntry> {
    // parse the output into entry entities
    const BOOT_RECORD_EXPR: &str = r"^Boot(?<bootnum>\d\d\d\d)(?<is_active>\*?)(?<description>.*)\t(?<boot_path>.*)$";
    // Regex documentation:
    // Group1 (bootnum): BootNum (a four-digit hex number)
    // Group2 (is_active): Inactive asterisk. If empty, entry is inactive, otherwise active.
    // Group3 (description): Description of the entry. May (or will) contain leading/trailing spaces.
    // A tab character as the separator between group 3 and 4
    // Group4 (boot_path): Boot entry's boot path. Probably not needed, but eh.
    let match_boot_record: Regex = Regex::new(BOOT_RECORD_EXPR).unwrap();

    let mut result: Vec<BootEntry> = vec![];
    let groups = match_boot_record.captures_iter(raw_output);
    for group in groups {
        let bootnum = &group["bootnum"];
        let is_active = &group["is_active"];
        let description = &group["description"];
        let boot_path = &group["boot_path"];

        result.push(BootEntry {
            id: format!("Boot{bootnum}"),
            platform_id: bootnum.to_string(),
            description: description.trim().to_string(),
            is_active: !is_active.contains("*"),
            boot_path: boot_path.to_string(),
        })
    }

    result
}