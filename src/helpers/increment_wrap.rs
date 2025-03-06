pub(crate) fn decrement_wrap(num: &mut i32, wrap_around: i32) {
    *num = (*num + wrap_around - 1) % wrap_around;
}
pub(crate) fn increment_wrap(num: &mut i32, wrap_around: i32) {
    *num = (*num + 1) % wrap_around;
}
