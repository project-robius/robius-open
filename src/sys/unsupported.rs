use std::marker::PhantomData;

use crate::Action;

pub(crate) struct Uri<'a, 'b> {
    phantom: PhantomData<(&'a (), &'b ())>,
}

impl<'a> Uri<'a> {
    pub(crate) fn new(_: &'a str) -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    pub fn action(self, _: Action) -> Self {
        self
    }

    pub fn open(self) {}
}
