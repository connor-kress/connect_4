use clearscreen::clear;

/// Clears the terminal screen.
///
/// Equivelent to `cls` on Windows or `clear` on Mac/Linux.
pub fn clear_screen() {
    clear().unwrap();
}
