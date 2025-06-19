use std::marker::PhantomData;

use block2::RcBlock;
use objc2_foundation::{NSDictionary, NSString, NSURL};
use objc2::{MainThreadMarker, runtime::Bool};
use objc2_ui_kit::UIApplication;

use crate::{Error, Result};

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

    pub(crate) fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn action(self, _: &'b str) -> Self {
        self
    }

    pub fn open<F>(self, on_completion: F) -> Result<()>
    where
        F: Fn(bool) + 'static,
    {
        let block = RcBlock::new(move |success: Bool| {
            #[cfg(feature = "log")]
            log::warn!("Calling on_completion closure with success: {}", success.as_raw());

            on_completion(success.into());
        });

        let string = NSString::from_str(self.inner);
        let url = unsafe { NSURL::URLWithString(&string) }
            .ok_or(Error::MalformedUri)?;
        let mtm = MainThreadMarker::new().ok_or(Error::NotMainThread)?;
        let application = UIApplication::sharedApplication(mtm);

        #[cfg(feature = "log")]
        log::warn!("Calling application.openURL()");

        unsafe {
            application.openURL_options_completionHandler(
                &url,
                &NSDictionary::new(), // no options used currently.
                Some(&block),
            );
        }
        #[cfg(feature = "log")]
        log::warn!("After application.openURL(). Returning Ok(())");
        Ok(())
    }
}
