use jni::{
    objects::{JClass, JObject},
    JNIEnv,
};

mod sys;

pub struct Uri<'a, 'b> {
    inner: sys::Uri<'a, 'b, 'static>,
}

impl<'a, 'b> Uri<'a, 'b> {
    pub fn new(s: &'a str) -> Self {
        // Self {
        // inner: sys::Uri::new(s),
        // }
        todo!();
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

#[no_mangle]
extern "C" fn rust_auth() {
    let uri = Uri::new("https://google.com");
    uri.open();
    let uri = Uri::new("etalrouhoaeu:huaeorckjqk");
    uri.open();
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_myapplication2_Test_greeting<'a>(
    env: JNIEnv<'a>,
    _: JClass<'a>,
) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Error),
    );
    sys::Uri::new("https://google.com", env).open();
}
