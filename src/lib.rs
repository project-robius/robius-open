mod sys;

pub struct Uri<'a, 'b> {
    inner: sys::Uri<'a, 'b>,
}

impl<'a, 'b> Uri<'a, 'b> {
    pub fn new(s: &'a str) -> Self {
        Self {
            inner: sys::Uri::new(s),
        }
    }

    pub fn action(self, action: &'b str) -> Self {
        Self {
            inner: self.inner.action(action),
        }
    }

    pub fn open(self) {
        self.inner.open();
    }

    // TODO: Callback.
}
