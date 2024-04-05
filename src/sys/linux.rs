use std::{marker::PhantomData, process::Command};

use crate::Action;

pub(crate) struct Uri<'a, 'b> {
    inner: &'a str,
    phantom: PhantomData<&'b ()>,
}

impl<'a> Uri<'a> {
    pub(crate) fn new(inner: &'a str) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }

    pub fn action(self, _: Action) -> Self {
        self
    }

    pub fn open(self) -> bool {
        // TODO: Test.
        if let Ok(status) = Command::new("xdg-open").arg(self.inner).status() {
            status.success()
        } else {
            false
        }
    }
}
