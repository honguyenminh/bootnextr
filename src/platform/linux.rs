use nix::unistd::Uid;
use std::process::{Command, Output};
use regex::Regex;
use crate::entities::BootEntry;

// public functions (contract)

/// Ensure permission requirements to run are met
/// # Arguments
/// * `is_output_only`: whether we only need read permissions (no writing to NVRAM)
/// # returns: `bool`
/// whether permissions are OK
pub fn ensure_permission(is_output_only: bool) -> bool {
    // efibootmgr can run as output only without root permissions
    if is_output_only { return true; }
    if !Uid::effective().is_root() {
        return false;
    }
    true
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

pub fn restart() {
    Command::new("shutdown")
        .args(["-r", "now"])
        .output()
        .expect("Cannot run 'shutdown -r now' command to restart");
}

// private functions

fn parse_boot_entries(raw_output: &str) -> Vec<BootEntry> {
    // parse the output into entry entities
    const BOOT_RECORD_EXPR: &str = r"(?m)^Boot(?<bootnum>\d\d\d\d)(?<is_active>\*?)(?<description>.*)\t(?<boot_path>.*)$";
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