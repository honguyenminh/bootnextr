// rust does not fucking have a reliable way to ensure consistent platform-specific APIs.
// So macros it is.

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_os = "windows")] {
        #[path = "windows.rs"]
        mod os;
    } else if #[cfg(target_os = "linux")] {
        #[path = "linux.rs"]
        mod os;
    } else {
        #[cfg(not(any(target_os = "windows", target_os = "linux")))]
        #[path = "unsupported.rs"]
        mod os;
    }
}

// contract to force the existence of these symbols.
// still not perfect since there's no signature guarantee.
// just use kotlin I guess.
pub use os::ensure_permission;
pub use os::get_boot_entries;
pub use os::set_boot_next;
