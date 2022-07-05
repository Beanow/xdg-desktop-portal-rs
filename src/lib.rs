// Copyright 2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Parent window identifiers
//! 
//! Most portals interact with the user by showing dialogs. These dialogs should generally be placed on top of the application window that triggered them. To arrange this, the compositor needs to know about the application window. Many portal requests expect a "parent_window" string argument for this reason.
//! Under X11, the "parent_window" argument should have the form "x11:XID", where XID is the XID of the application window in hexadecimal notation.
//! Under Wayland, it should have the form "wayland:HANDLE", where HANDLE is a surface handle obtained with the xdg_foreign protocol.
//! For other windowing systems, or if you don't have a suitable handle, just pass an empty string for "parent_window". 

mod open_uri;

pub use open_uri::*;

use dbus::blocking::{BlockingSender, Proxy};
use std::{ops::Deref, time::Duration};

pub fn new_blocking<'a, B: BlockingSender, C: Deref<Target = B>>(
  timeout: Duration,
  connection: C,
) -> Proxy<'a, C> {
  Proxy::new(
    "org.freedesktop.portal.Desktop",
    "/org/freedesktop/portal/desktop",
    timeout,
    connection,
  )
}
