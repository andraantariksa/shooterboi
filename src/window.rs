use std::ops::{Deref, DerefMut};
use winit::window::Window as WinitWindow;

pub struct Window {
    window_internal: WinitWindow,
    is_cursor_grabbed: bool,
}

impl Window {
    pub(crate) fn set_is_cursor_grabbed(&mut self, grabbed: bool) {
        self.is_cursor_grabbed = grabbed;
        self.window_internal.set_cursor_grab(grabbed);
    }

    pub(crate) fn is_cursor_grabbed(&self) -> bool {
        self.is_cursor_grabbed
    }
}

impl From<WinitWindow> for Window {
    fn from(window_internal: WinitWindow) -> Self {
        Self {
            is_cursor_grabbed: false,
            window_internal,
        }
    }
}

impl Deref for Window {
    type Target = WinitWindow;

    fn deref(&self) -> &Self::Target {
        &self.window_internal
    }
}

impl DerefMut for Window {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window_internal
    }
}
