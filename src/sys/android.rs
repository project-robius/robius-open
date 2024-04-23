use jni::objects::JValueGen;

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
        let res = robius_android_env::with_activity(|env, current_activity| {
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

            #[cfg(feature = "android-result")]
            let is_err = {
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

                component_name.as_raw().is_null()
            };
            #[cfg(not(feature = "android-result"))]
            let is_err = false;

            if is_err {
                // NOTE: If the correct permissions aren't added to the app manifest,
                // resolveActivity will return null regardless.
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
        });

        match res {
            Some(Ok(())) => Ok(()),
            Some(Err(_)) => {
                #[cfg(feature = "log")]
                log::error!(
                    "resolveActivity method failed. Is your app manifest missing permissions?"
                );
                // TODO: add error enum: the resolveActivity method failed,
                //       which implies the app manifest is missing permissions.
                Err(())
            }
            None => {
                #[cfg(feature = "log")]
                log::error!(
                    "couldn't get current activity or JVM/JNI. Did you call \
                     `robius_android_env::set_vm()` and \
                     `robius_android_env::set_activity_getter()`?"
                );
                // TODO: add error enum: couldn't get current activity or JVM/JNI
                Err(())
            }
        }
    }
}
