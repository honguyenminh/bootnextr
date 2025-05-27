/// Represents a boot entry in UEFI NVRAM
#[derive(Debug)]
pub struct BootEntry {
    /// Full form id of the boot entry. Usually contains more detail than platform_id
    pub id: String,
    /// ID used by internal platform boot manager to set next boot record.
    /// GUID for Windows (BcdEdit), BootNum (the #### in Boot####) for Linux
    pub platform_id: String,
    /// Entry description, usually a human-readable name. What you see in the boot menu.
    pub description: String,
    /// Whether the entry is marked as inactive in NVRAM.
    /// Apparently (?) Windows does not show these, so will always be true there.
    pub is_active: bool,
    /// Path to bootable pointed at by this entry.
    pub boot_path: String,
}
