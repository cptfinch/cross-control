#[cfg(target_os = "linux")]
pub mod x11;

#[cfg(target_os = "windows")]
pub mod windows;  // Placeholder for future Windows support 