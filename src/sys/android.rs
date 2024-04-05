use jni::{objects::JValueGen, JNIEnv};

pub(crate) struct Uri<'a, 'b, 'c> {
    inner: &'a str,
    action: &'b str,
    env: JNIEnv<'c>,
}

impl<'a, 'b, 'c> Uri<'a, 'b, 'c> {
    pub(crate) fn new(inner: &'a str, temp: JNIEnv<'c>) -> Self {
        Self {
            inner,
            action: "ACTION_VIEW",
            env: temp,
        }
    }

    pub(crate) fn action(self, action: &'b str) -> Self {
        Self { action, ..self }
    }

    pub(crate) fn open(self) {
        let mut env = self.env;

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

        env.call_static_method(
            "android/app/Activity",
            "startActivity",
            "(Landroid/content/Intent;)V",
            &[JValueGen::Object(&intent)],
        )
        .unwrap();

        println!("ok");
    }
}
