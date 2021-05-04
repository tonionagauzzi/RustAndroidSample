use jni::objects::JObject;
use jni::sys::jstring;
use jni::JNIEnv;

#[no_mangle]
pub unsafe extern "C" fn Java_com_vitantonio_nagauzzi_rustandroidsample_MainActivity_stringFromJNI(
    env: JNIEnv,
    _this: JObject,
) -> jstring {
    let hello = "Hello from Rust!";

    env.new_string(hello)
        .expect("Couldn't create Java string!")
        .into_inner()
}
