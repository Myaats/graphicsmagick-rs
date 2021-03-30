use graphicsmagick_sys::{InitializeMagickEx, MAGICK_OPT_NO_SIGNAL_HANDER};
use std::{ptr::{null, null_mut}, sync::Once};

static HAS_INITIALIZED: Once = Once::new();

/// Wrapper of `graphicsmagick_sys::InitializeMagick`, call it before any `graphicsmagick` action.
pub fn initialize() {
    HAS_INITIALIZED.call_once(|| unsafe {
        InitializeMagickEx(null(), MAGICK_OPT_NO_SIGNAL_HANDER, null_mut());
    });
}

/// Check if [`initialize`] has called.
#[inline]
pub fn has_initialized() -> bool {
    HAS_INITIALIZED.is_completed()
}

#[inline]
pub(crate) fn assert_initialized() {
    assert!(
        has_initialized(),
        "You have to call `graphicsmagick::initialize` first of all"
    )
}
