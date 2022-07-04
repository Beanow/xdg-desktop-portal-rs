// Copyright 2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use dbus::{
  arg::{OwnedFd, PropMap, Variant},
  blocking::{self, stdintf::org_freedesktop_dbus},
  Error as DbusError, Path,
};
use std::ops::Deref;

const INTERFACE: &'static str = "org.freedesktop.portal.OpenURI";

pub trait OpenURI {
  /// Asks to open a uri.
  ///
  /// Note that `file://` uris are explicitly not supported by this method.
  /// To request opening local files, use `OpenURI::open_file()`.
  ///
  /// `parent_window`: Identifier for the application window, see crate comments for common conventions.
  /// `uri`: The uri to open
  fn open_uri(
    &self,
    parent_window: &str,
    uri: &str,
    options: OpenURIOptions,
  ) -> Result<Path<'static>, DbusError>;

  ///  Asks to open a local file.
  ///
  /// `parent_window`: Identifier for the application window, see crate comments for common conventions.
  /// `fd`: File descriptor for the file to open.
  fn open_file(
    &self,
    parent_window: &str,
    fd: OwnedFd,
    options: OpenURIOptions,
  ) -> Result<Path<'static>, DbusError>;

  ///  Asks to open the directory containing a local file in the file browser.
  ///
  /// `parent_window`: Identifier for the application window, see crate comments for common conventions.
  /// `fd`: File descriptor a file.
  fn open_directory(
    &self,
    parent_window: &str,
    fd: OwnedFd,
    options: OpenURIOptions,
  ) -> Result<Path<'static>, DbusError>;
  fn version(&self) -> Result<u32, DbusError>;
}

#[derive(Default)]
pub struct OpenURIOptions {
  handle_token: Option<String>,
  writable: Option<bool>,
  #[cfg(feature = "spec-v3")]
  ask: Option<bool>,
  #[cfg(feature = "spec-v4")]
  activation_token: Option<String>,
}

impl OpenURIOptions {
  pub fn new() -> Self {
    Default::default()
  }

  /// A string that will be used as the last element of the @handle. Must be a valid
  /// object path element. See the #org.freedesktop.portal.Request documentation for
  /// more information about the @handle.
  pub fn handle_token(mut self, handle_token: String) -> Self {
    self.handle_token = Some(handle_token);
    self
  }

  /// Whether to allow the chosen application to write to the file.
  ///
  /// This key only takes effect the uri points to a local file that
  /// is exported in the document portal, and the chosen application
  /// is sandboxed itself.
  pub fn writable(mut self, writable: bool) -> Self {
    self.writable = Some(writable);
    self
  }

  /// Whether to ask the user to choose an app. If this is not passed, or false,
  /// the portal may use a default or pick the last choice.
  ///
  /// The ask option was introduced in version 3 of the interface.
  #[cfg(feature = "spec-v3")]
  pub fn ask(mut self, ask: bool) -> Self {
    self.ask = Some(ask);
    self
  }

  /// A token that can be used to activate the chosen application.
  ///
  /// The activation_token option was introduced in version 4 of the interface.
  #[cfg(feature = "spec-v4")]
  pub fn activation_token(mut self, activation_token: String) -> Self {
    self.activation_token = Some(activation_token);
    self
  }
}

impl From<OpenURIOptions> for PropMap {
  fn from(options: OpenURIOptions) -> Self {
    let mut map = PropMap::new();
    if let Some(handle_token) = options.handle_token {
      map.insert("handle_token".to_string(), Variant(Box::new(handle_token)));
    }
    if let Some(writable) = options.writable {
      map.insert("writable".to_string(), Variant(Box::new(writable)));
    }
    #[cfg(feature = "spec-v3")]
    if let Some(ask) = options.ask {
      map.insert("ask".to_string(), Variant(Box::new(ask)));
    }
    #[cfg(feature = "spec-v4")]
    if let Some(activation_token) = options.activation_token {
      map.insert(
        "activation_token".to_string(),
        Variant(Box::new(activation_token)),
      );
    }
    map
  }
}

impl<'a, T: blocking::BlockingSender, C: Deref<Target = T>> OpenURI for blocking::Proxy<'a, C> {
  fn open_uri(
    &self,
    parent_window: &str,
    uri: &str,
    options: OpenURIOptions,
  ) -> Result<Path<'static>, DbusError> {
    self
      .method_call(
        INTERFACE,
        "OpenURI",
        (parent_window, uri, PropMap::from(options)),
      )
      .and_then(|r: (Path<'static>,)| Ok(r.0))
  }

  fn open_file(
    &self,
    parent_window: &str,
    fd: OwnedFd,
    options: OpenURIOptions,
  ) -> Result<Path<'static>, DbusError> {
    self
      .method_call(
        INTERFACE,
        "OpenFile",
        (parent_window, fd, PropMap::from(options)),
      )
      .and_then(|r: (Path<'static>,)| Ok(r.0))
  }

  fn open_directory(
    &self,
    parent_window: &str,
    fd: OwnedFd,
    options: OpenURIOptions,
  ) -> Result<Path<'static>, DbusError> {
    self
      .method_call(
        INTERFACE,
        "OpenDirectory",
        (parent_window, fd, PropMap::from(options)),
      )
      .and_then(|r: (Path<'static>,)| Ok(r.0))
  }

  fn version(&self) -> Result<u32, DbusError> {
    <Self as org_freedesktop_dbus::Properties>::get(&self, INTERFACE, "version")
  }
}
