/// Aborts current process (thread). Also crashes system.
///
/// Useful for failures by memmgr reasons.
///
/// See [`core::intrinsics::abort()`].
#[inline(always)]
pub fn abort() -> ! { core::intrinsics::abort() }
