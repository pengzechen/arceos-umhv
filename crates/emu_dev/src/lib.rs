
#![no_std]
#![feature(const_ptr_as_ref)]
#![feature(const_option)]
#![feature(const_nonnull_new)]

#[derive(Debug)]
pub struct EmuContext {
    /// fault address
    pub address: usize,
    /// instruction width
    pub width: usize,
    /// write or read
    pub write: bool,
    /// Data item whether should be sign-extended.
    pub sign_ext: bool,
    /// target or source register idx
    pub reg: usize,
    /// target or source register width
    pub reg_width: usize,
}
