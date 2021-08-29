/// Move a field from one struct to another
/// ```rs
/// struct S1 {
///     field: T,
/// }
///
/// struct S2 {
///     field: T,
/// }
///
/// fn s1_to_s2(s1: S1) -> S2 {
///     S2 { field: move_field(&mut s1.field) }
///     // s1.field is now invalid
/// }
/// ```
pub(crate) unsafe fn move_field<T>(field: &mut T) -> T {
    let replace_with = std::mem::MaybeUninit::uninit().assume_init();
    let moved_field = std::mem::replace(field, replace_with);
    std::mem::forget(field);
    moved_field
}
