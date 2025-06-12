use std::marker::PhantomData;

use block2::{Block, StackBlock};
use objc2_foundation::{NSObjectProtocol, NSDictionary, NSString, NSURL};
use objc2::{
    extern_class, extern_conformance, extern_methods, rc::Retained, runtime::{AnyObject, Bool, NSObject}
};

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

        let application = UIApplication::shared();
        let (tx, rx) = std::sync::mpsc::channel();
        let block = StackBlock::new(move |success: Bool| {
            // TODO: this closure block never fires.
            #[cfg(feature = "log")]
            log::error!("open completionHandler called with success: {}", success.as_raw());

            // NOTE: We want to panic here as the main thread will hang waiting for a
            // message on the channel.
            tx.send(success).expect("failed to send open result");

            #[cfg(feature = "log")]
            log::error!("open completionHandler is done, sent tx.send(success) successfully");
            0usize
        })
        .copy();

        #[cfg(feature = "log")]
        log::error!("Calling application.open()");
        application.open(&url, &NSDictionary::new(), &block);
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

extern_class!(
    #[unsafe(super(NSObject))]
    struct UIResponder;
);

extern_class!(
    #[unsafe(super(UIResponder))]
    struct UIApplication;
);

// not sure if these are needed
extern_conformance!(unsafe impl NSObjectProtocol for UIResponder {});
// extern_conformance!(unsafe impl NSCopying for UIResponder {});
// extern_conformance!(unsafe impl NSCoding for UIResponder {});

extern_conformance!(unsafe impl NSObjectProtocol for UIApplication {});


impl UIApplication {
    extern_methods!(
        #[unsafe(method(sharedApplication))]
        pub fn shared() -> Retained<UIApplication>;

        #[unsafe(method(canOpenURL:))]
        fn can_open(&self, url: &NSURL) -> bool;

        #[unsafe(method(openURL:options:completionHandler:))]
        fn open(
            &self,
            url: &NSURL,
            // TODO?
            options: &NSDictionary<NSString, AnyObject>,
            // TODO?
            completion_handler: &Block<dyn Fn(Bool) -> usize>,
        );
    );
}
