use jni::{
    objects::{JClass, JValueGen},
    JNIEnv,
};

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
        let mut env: JNIEnv = todo!();
        let context: JClass = todo!();

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

        let activity = env.new_object("android/app/Activity", "()V", &[]).unwrap();

        env.call_method(
            // activity,
            context,
            "startActivity",
            "(Landroid/content/Intent;)V",
            &[JValueGen::Object(&intent)],
        )
        .unwrap();
    }
}
