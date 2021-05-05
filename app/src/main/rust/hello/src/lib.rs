mod generator;
use crate::generator::human::first_name;
use crate::generator::human::last_name;
use crate::generator::human::gender;
use crate::generator::human::birthday;
use jni::objects::{JObject, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use std::ffi::{CString, CStr};

#[no_mangle]
pub unsafe extern "C" fn Java_com_vitantonio_nagauzzi_rustandroidsample_MainActivity_someGender(
    env: JNIEnv,
    _this: JObject,
) -> jstring {
    env.new_string(gender())
        .expect("Couldn't create some gender!")
        .into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_vitantonio_nagauzzi_rustandroidsample_MainActivity_someFirstName(
    env: JNIEnv,
    _this: JObject,
    j_gender: JString,
) -> jstring {
    let gender = CString::from(
        CStr::from_ptr(
            env.get_string(j_gender).unwrap().as_ptr()
        )
    );
    env.new_string(first_name(gender))
        .expect("Couldn't create some firstname!")
        .into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_vitantonio_nagauzzi_rustandroidsample_MainActivity_someLastName(
    env: JNIEnv,
    _this: JObject,
) -> jstring {
    env.new_string(last_name())
        .expect("Couldn't create some lastname!")
        .into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_vitantonio_nagauzzi_rustandroidsample_MainActivity_someBirthday(
    env: JNIEnv,
    _this: JObject,
) -> jstring {
    env.new_string(birthday())
        .expect("Couldn't create some birthday!")
        .into_inner()
}
