// Copyright 2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

mod openuri;

pub use openuri::*;

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

#[cfg(test)]
mod test {
  use crate::{new_blocking, OpenURI, OpenURIOptions};
  use dbus::blocking::Connection;
  use std::time::Duration;

  #[test]
  fn hello_test() {
    let conn = Connection::new_session().unwrap();
    let timeout = Duration::from_secs(2);
    let portals = new_blocking(timeout, &conn);
    let opts = OpenURIOptions::new().ask(true);
    portals
      .open_uri("", "https://foo.bar#hello-dbus", opts)
      .unwrap();
  }
}
