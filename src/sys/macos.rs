use std::marker::PhantomData;

use icrate::{
    AppKit::NSWorkspace,
    Foundation::{NSString, NSURL},
};

pub(crate) struct Uri<'a, 'b> {
    inner: &'a str,
    phantom: PhantomData<&'b ()>,
}

impl<'a, 'b> Uri<'a, 'b> {
    pub(crate) fn new(inner: &'a str) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }

    pub fn action(self, _: &'b str) -> Self {
        self
    }

    pub fn open(self) {
        let string = NSString::from_str(self.inner);
        let url = unsafe { NSURL::URLWithString(&string) }.unwrap();
        let workspace = unsafe { NSWorkspace::sharedWorkspace() };
        unsafe { workspace.openURL(&url) };
    }
}
