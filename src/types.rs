use libc;

pub enum ScreenImpl {}
pub enum CellImpl {}
pub struct ColorImpl(pub libc::uint32_t);

#[repr(C)]
pub struct Loc {
    pub row: libc::c_int,
    pub col: libc::c_int,
}