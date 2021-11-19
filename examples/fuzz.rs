use std::io::Read as _;

#[path = "../tests/helpers/mod.rs"]
mod helpers;

fn check_full(vt_base: &vt100::Screen, empty: &vt100::Screen, idx: usize) {
    let mut vt_full = vt100::Parser::default();
    vt_full.process(&vt_base.state_formatted());
    vt_full.process(&vt_base.bells_diff(empty));
    assert!(
        helpers::compare_screens(vt_full.screen(), vt_base),
        "{}: full",
        idx,
    );
}

fn check_diff_empty(
    vt_base: &vt100::Screen,
    empty: &vt100::Screen,
    idx: usize,
) {
    let mut vt_diff_empty = vt100::Parser::default();
    vt_diff_empty.process(&vt_base.state_diff(empty));
    vt_diff_empty.process(&vt_base.bells_diff(empty));
    assert!(
        helpers::compare_screens(vt_diff_empty.screen(), vt_base),
        "{}: diff-empty",
        idx,
    );
}

fn check_diff(
    vt_base: &vt100::Screen,
    vt_diff: &mut vt100::Parser,
    prev: &vt100::Screen,
    empty: &vt100::Screen,
    idx: usize,
) {
    vt_diff.process(&vt_base.state_diff(prev));
    vt_diff.process(&vt_base.bells_diff(empty));
    assert!(
        helpers::compare_screens(vt_diff.screen(), vt_base),
        "{}: diff",
        idx,
    );
}

fn check_rows(vt_base: &vt100::Screen, empty: &vt100::Screen, idx: usize) {
    let mut vt_rows = vt100::Parser::default();
    let mut wrapped = false;
    for (idx, row) in vt_base.rows_formatted(0, 80).enumerate() {
        vt_rows.process(b"\x1b[m");
        if !wrapped {
            vt_rows.process(format!("\x1b[{}H", idx + 1).as_bytes());
        }
        vt_rows.process(&row);
        wrapped = vt_base.row_wrapped(idx.try_into().unwrap());
    }
    vt_rows.process(&vt_base.cursor_state_formatted());
    vt_rows.process(&vt_base.attributes_formatted());
    vt_rows.process(&vt_base.input_mode_formatted());
    vt_rows.process(&vt_base.title_formatted());
    vt_rows.process(&vt_base.bells_diff(empty));
    assert!(
        helpers::compare_screens(vt_rows.screen(), vt_base),
        "{}: rows",
        idx,
    );
}

fn read_byte() -> Option<u8> {
    let mut byte = [0];
    match std::io::stdin().read(&mut byte) {
        Ok(bytes) => {
            if bytes != 1 {
                return None;
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            return None;
        }
    }
    Some(byte[0])
}

fn main() {
    let mut vt_base = vt100::Parser::default();
    let mut vt_diff = vt100::Parser::default();
    let mut prev_screen = vt_base.screen().clone();
    let empty_screen = vt100::Parser::default().screen().clone();
    let mut idx = 0;
    while let Some(byte) = read_byte() {
        vt_base.process(&[byte]);

        check_full(vt_base.screen(), &empty_screen, idx);
        check_diff_empty(vt_base.screen(), &empty_screen, idx);
        check_diff(
            vt_base.screen(),
            &mut vt_diff,
            &prev_screen,
            &empty_screen,
            idx,
        );
        check_rows(vt_base.screen(), &empty_screen, idx);

        prev_screen = vt_base.screen().clone();
        idx += 1;
    }
}