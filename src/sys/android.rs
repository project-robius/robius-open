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

    pub(crate) fn open(self) -> Result<(), ()> {
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

        let package_manager = env
            .call_method(
                current_activity,
                "getPackageManager",
                "()Landroid/content/pm/PackageManager;",
                &[],
            )
            .unwrap()
            .l()
            .unwrap();

        let component_name = env
            .call_method(
                &intent,
                "resolveActivity",
                "(Landroid/content/pm/PackageManager;)Landroid/content/ComponentName;",
                &[JValueGen::Object(&package_manager)],
            )
            .unwrap()
            .l()
            .unwrap();

        // TODO: This doesn't work for some reason
        // if component_name.as_raw().is_null() {
        if false {
            Err(())
        } else {
            env.call_method(
                current_activity,
                "startActivity",
                "(Landroid/content/Intent;)V",
                &[JValueGen::Object(&intent)],
            )
            .unwrap();
            Ok(())
        }
    }
}
