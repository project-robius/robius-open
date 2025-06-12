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

    pub fn action(self, _: &'b str) -> Self {
        self
    }

    pub fn open(self) -> Result<()> {
        let string = NSString::from_str(self.inner);
        let url = unsafe { NSURL::URLWithString(&string) }.ok_or(Error::MalformedUri)?;
        let mtm = MainThreadMarker::new().ok_or(Error::NotMainThread)?;
        let application = UIApplication::sharedApplication(mtm);

        let (tx, rx) = std::sync::mpsc::channel();
        let block = RcBlock::new(move |success: Bool| {
            // TODO: this closure block never fires.
            #[cfg(feature = "log")]
            log::error!("open completionHandler called with success: {}", success.as_raw());

            // NOTE: We want to panic here as the main thread will hang waiting for a
            // message on the channel.
            tx.send(success).expect("failed to send open result");

            #[cfg(feature = "log")]
            log::error!("open completionHandler is done, sent tx.send(success) successfully");
        });

        #[cfg(feature = "log")]
        log::error!("Calling application.open()");

        unsafe {
            application.openURL_options_completionHandler(
                &url,
                &NSDictionary::new(), // no options used currently.
                Some(&block),
            );
        }
        #[cfg(feature = "log")]
        log::error!("After application.open(), waiting on rx.recv()");

        // NOTE: try a timeout here on rx.recv() to avoid locking the thread entirely
        //       if the completionHandler block never fires.

        let res = match rx.recv() {
            Ok(success) if success.is_true() => Ok(()),
            _ => Err(Error::Unknown),
        };
        #[cfg(feature = "log")]
        log::error!("After rx.recv(), got {res:?}");
        res
    }
}
