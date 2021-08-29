/// Casts a unit-variant enum to another. For this function to be used safely, you must
/// ensure that `[repr(inttype)] TIn == [repr(inttype)] TOut`
pub(crate) unsafe fn cast_enum<TIn, TOut>(enum_in: TIn) -> TOut {
    let mut out: TOut = std::mem::MaybeUninit::uninit().assume_init();
    let dst = &mut out as *mut TOut;
    std::ptr::copy(&enum_in as *const TIn as *const TOut, dst, 1);
    out
}
