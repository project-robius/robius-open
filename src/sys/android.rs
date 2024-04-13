use jni::objects::JValueGen;
use robius_android_env::{current_activity, vm};

pub(crate) struct Uri<'a, 'b> {
    inner: &'a str,
    action: &'b str,
}

impl<'a, 'b> Uri<'a, 'b> {
    pub(crate) fn new(inner: &'a str) -> Self {
        Self {
            inner,
            action: "ACTION_VIEW",
        }
    }

    pub(crate) fn action(self, action: &'b str) -> Self {
        Self { action, ..self }
    }

    pub(crate) fn open(self) {
        let mut env = vm().unwrap().get_env().unwrap();
        let current_activity = current_activity().unwrap();

        let action = env
            .get_static_field("android/content/Intent", self.action, "Ljava/lang/String;")
            .unwrap()
            .l()
            .unwrap();

        let string = env.new_string(self.inner).unwrap();
        let uri = env
            .call_static_method(
                "android/net/Uri",
                "parse",
                "(Ljava/lang/String;)Landroid/net/Uri;",
                &[JValueGen::Object(&string)],
            )
            .unwrap()
            .l()
            .unwrap();

        let intent = env
            .new_object(
                "android/content/Intent",
                "(Ljava/lang/String;Landroid/net/Uri;)V",
                &[JValueGen::Object(&action), JValueGen::Object(&uri)],
            )
            .unwrap();

        env.call_method(
            current_activity,
            "startActivity",
            "(Landroid/content/Intent;)V",
            &[JValueGen::Object(&intent)],
        )
        .unwrap();
    }
}
