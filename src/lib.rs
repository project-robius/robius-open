//! A crate for opening files.
//!
//! ```
//! # use robius_open::Uri;
//! Uri::new("tel:+61 123 456 789").open();
//! ```

mod sys;

/// A uniform resource identifier.
pub struct Uri<'a, 'b> {
    inner: sys::Uri<'a, 'b>,
}

impl<'a, 'b> Uri<'a, 'b> {
    /// Constructs a new URI.
    pub fn new(s: &'a str) -> Self {
        Self {
            inner: sys::Uri::new(s),
        }
    }

    /// Sets the action to perform with this URI.
    ///
    /// This only has an effect on Android, and corresponds to an [action
    /// activity][aa]. By default, it is set to `"ACTION_VIEW"`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use robius_open::Uri;
    /// Uri::new("tel:+61 123 456 789").action("ACTION_DIAL").open();
    /// ```
    ///
    /// [aa]: https://developer.android.com/reference/android/content/Intent#standard-activity-actions
    pub fn action(self, action: &'b str) -> Self {
        Self {
            inner: self.inner.action(action),
        }
    }

    /// Opens the URI.
    pub fn open(self) {
        self.inner.open();
    }

    // TODO: Callback.
}
