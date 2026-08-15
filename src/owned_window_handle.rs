use raw_window_handle::{HandleError, HasDisplayHandle, HasWindowHandle};
use std::sync::Arc;
#[cfg(target_os = "android")]
use std::{ffi::c_void, ptr::NonNull};

/// On Android it's not safe to rely on Winit's Window implementing
/// HasWindowHandle because it doesn't (and can't) own the `ANativeWindow` that
/// is used as a raw window handle. (If it acquired a reference then it would
/// have to effectively leak that reference every time the app gets a new native
/// window)
///
/// When an Android application suspends, the `ANativeWindow`s associated with
/// surfaces may be released and so a raw window handle obtained via
/// `winit::Window::window_handle()` may become an invalid pointer if nothing
/// `_acquire()`d a reference to it.
///
/// To work around this, we create our own wrapper around the `ANativeWindow` so
/// we can `_acquire()` an owning reference that we `_release()` when Dropped.
pub(crate) struct OwnedWindowHandle {
    #[cfg(not(target_os = "android"))]
    window: Arc<winit::window::Window>,
    #[cfg(target_os = "android")]
    native_window: NonNull<c_void>, // ANativeWindow*
}

unsafe impl Send for OwnedWindowHandle {}
unsafe impl Sync for OwnedWindowHandle {}
impl OwnedWindowHandle {
    pub(crate) fn new(window: Arc<winit::window::Window>) -> Result<Self, HandleError> {
        #[cfg(not(target_os = "android"))]
        {
            Ok(Self { window })
        }

        #[cfg(target_os = "android")]
        {
            let raw_handle = window.window_handle()?.as_raw();
            if let raw_window_handle::RawWindowHandle::AndroidNdk(handle) = raw_handle {
                let native_window = handle.a_native_window;
                unsafe extern "C" {
                    fn ANativeWindow_acquire(window: *mut c_void);
                }
                // SAFETY: We assume the caller has ensured that `native_window` is a valid
                // pointer to an `ANativeWindow` and that we own a reference to it.
                unsafe {
                    ANativeWindow_acquire(native_window.as_ptr());
                }
                Ok(Self { native_window })
            } else {
                panic!("Expected AndroidNdk window handle");
            }
        }
    }
}
impl Drop for OwnedWindowHandle {
    fn drop(&mut self) {
        #[cfg(target_os = "android")]
        {
            unsafe extern "C" {
                fn ANativeWindow_release(window: *mut c_void);
            }
            // SAFETY: We assume that `native_window` is a valid pointer to an
            // `ANativeWindow` that we own a reference to.
            unsafe {
                ANativeWindow_release(self.native_window.as_ptr());
            }
        }
    }
}
impl HasWindowHandle for OwnedWindowHandle {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        #[cfg(not(target_os = "android"))]
        {
            self.window.window_handle()
        }

        #[cfg(target_os = "android")]
        unsafe {
            Ok(raw_window_handle::WindowHandle::borrow_raw(
                raw_window_handle::AndroidNdkWindowHandle::new(self.native_window).into(),
            ))
        }
    }
}
impl HasDisplayHandle for OwnedWindowHandle {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        #[cfg(not(target_os = "android"))]
        {
            self.window.display_handle()
        }

        #[cfg(target_os = "android")]
        unsafe {
            Ok(raw_window_handle::DisplayHandle::borrow_raw(
                raw_window_handle::AndroidDisplayHandle::new().into(),
            ))
        }
    }
}
