extern crate jni;
#[macro_use]
extern crate rust_swig;

// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JString};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::jstring;
use jni::sys::jlong;
use jni::sys::jboolean;
use crate::opa::{createOPAContext, createRego, GoString, partialEval, eval};
use std::ffi::{CString, c_void};
use combine::stream::Range;

// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
// This turns off linter warnings because the name doesn't conform to
// conventions.
#[allow(non_snake_case)]
pub extern "system" fn Java_HelloWorld_hello(env: JNIEnv,
// This is the class that owns our static method. It's not going to be used,
// but still must be present to match the expected signature of a static
// native method.
                                             class: JClass,
                                             input: JString)
                                             -> jstring {
    // First, we have to get the string out of Java. Check out the `strings`
    // module for more info on how this works.
    let input: String =
        env.get_string(input).expect("Couldn't get java string!").into();

    // Then we have to create a new Java string to return. Again, more info
    // in the `strings` module.
    let output = env.new_string(format!("Hello, {}!", input))
        .expect("Couldn't create java string!");

    // Finally, extract the raw pointer to return.
    output.into_inner()
}
use std::ffi::CString;
use std::error::Error;

#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(C)]
pub struct __BindgenComplex<T> {
    pub re: T,
    pub im: T,
}
pub type wchar_t = ::std::os::raw::c_ushort;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}
#[test]
fn bindgen_test_layout_max_align_t() {
    assert_eq!(
        ::std::mem::size_of::<max_align_t>(),
        32usize,
        concat!("Size of: ", stringify!(max_align_t))
    );
    assert_eq!(
        ::std::mem::align_of::<max_align_t>(),
        16usize,
        concat!("Alignment of ", stringify!(max_align_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<max_align_t>())).__clang_max_align_nonce1 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<max_align_t>())).__clang_max_align_nonce2 as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce2)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GoString_ {
    pub p: *const ::std::os::raw::c_char,
    pub n: isize,
}
#[test]
fn bindgen_test_layout__GoString_() {
    assert_eq!(
        ::std::mem::size_of::<_GoString_>(),
        16usize,
        concat!("Size of: ", stringify!(_GoString_))
    );
    assert_eq!(
        ::std::mem::align_of::<_GoString_>(),
        8usize,
        concat!("Alignment of ", stringify!(_GoString_))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GoString_>())).p as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GoString_),
            "::",
            stringify!(p)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GoString_>())).n as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_GoString_),
            "::",
            stringify!(n)
        )
    );
}
pub type GoInt8 = ::std::os::raw::c_schar;
pub type GoUint8 = ::std::os::raw::c_uchar;
pub type GoInt16 = ::std::os::raw::c_short;
pub type GoUint16 = ::std::os::raw::c_ushort;
pub type GoInt32 = ::std::os::raw::c_int;
pub type GoUint32 = ::std::os::raw::c_uint;
pub type GoInt64 = ::std::os::raw::c_longlong;
pub type GoUint64 = ::std::os::raw::c_ulonglong;
pub type GoInt = GoInt64;
pub type GoUint = GoUint64;
pub type GoUintptr = ::std::os::raw::c_ulonglong;
pub type GoFloat32 = f32;
pub type GoFloat64 = f64;
pub type GoComplex64 = __BindgenComplex<f32>;
pub type GoComplex128 = __BindgenComplex<f64>;
pub type _check_for_64_bit_pointer_matching_GoInt = [::std::os::raw::c_char; 1usize];
pub type GoString = _GoString_;
pub type GoMap = *mut ::std::os::raw::c_void;
pub type GoChan = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GoInterface {
    pub t: *mut ::std::os::raw::c_void,
    pub v: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_GoInterface() {
    assert_eq!(
        ::std::mem::size_of::<GoInterface>(),
        16usize,
        concat!("Size of: ", stringify!(GoInterface))
    );
    assert_eq!(
        ::std::mem::align_of::<GoInterface>(),
        8usize,
        concat!("Alignment of ", stringify!(GoInterface))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GoInterface>())).t as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GoInterface),
            "::",
            stringify!(t)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GoInterface>())).v as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(GoInterface),
            "::",
            stringify!(v)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GoSlice {
    pub data: *mut ::std::os::raw::c_void,
    pub len: GoInt,
    pub cap: GoInt,
}
#[test]
fn bindgen_test_layout_GoSlice() {
    assert_eq!(
        ::std::mem::size_of::<GoSlice>(),
        24usize,
        concat!("Size of: ", stringify!(GoSlice))
    );
    assert_eq!(
        ::std::mem::align_of::<GoSlice>(),
        8usize,
        concat!("Alignment of ", stringify!(GoSlice))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GoSlice>())).data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GoSlice),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GoSlice>())).len as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(GoSlice),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GoSlice>())).cap as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(GoSlice),
            "::",
            stringify!(cap)
        )
    );
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct partialEval_return {
    pub r0: *mut ::std::os::raw::c_void,
    pub r1: *mut GoString,
}
#[test]
fn bindgen_test_layout_partialEval_return() {
    assert_eq!(
        ::std::mem::size_of::<partialEval_return>(),
        16usize,
        concat!("Size of: ", stringify!(partialEval_return))
    );
    assert_eq!(
        ::std::mem::align_of::<partialEval_return>(),
        8usize,
        concat!("Alignment of ", stringify!(partialEval_return))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<partialEval_return>())).r0 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(partialEval_return),
            "::",
            stringify!(r0)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<partialEval_return>())).r1 as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(partialEval_return),
            "::",
            stringify!(r1)
        )
    );
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct eval_return {
    pub r0: *mut GoUint8,
    pub r1: *mut GoString,
}
#[test]
fn bindgen_test_layout_eval_return() {
    assert_eq!(
        ::std::mem::size_of::<eval_return>(),
        16usize,
        concat!("Size of: ", stringify!(eval_return))
    );
    assert_eq!(
        ::std::mem::align_of::<eval_return>(),
        8usize,
        concat!("Alignment of ", stringify!(eval_return))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<eval_return>())).r0 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(eval_return),
            "::",
            stringify!(r0)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<eval_return>())).r1 as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(eval_return),
            "::",
            stringify!(r1)
        )
    );
}

#[link(name="libopa", kind="static")]
extern "C" {
    pub fn createOPAContext() -> *mut ::std::os::raw::c_void;

    pub fn createRego(
        p0: GoString,
        p1: GoString,
        p2: GoString,
        p3: GoString,
    ) -> *mut ::std::os::raw::c_void;

    pub fn partialEval(
        p0: *mut ::std::os::raw::c_void,
        p1: *mut ::std::os::raw::c_void,
    ) -> partialEval_return;

    pub fn eval(
        p0: *mut ::std::os::raw::c_void,
        p1: GoString,
        p2: *mut ::std::os::raw::c_void,
    ) -> eval_return;
}

fn create_go_string(string: &str) -> GoString {
    let strt = CString::new(string).unwrap();
    let length = strt.to_bytes().len();
    GoString { p:  CString::into_raw(strt), n: length as isize }
}

struct OPA {
    context: *mut c_void,
    rego: *mut c_void,
    partial_eval: *mut c_void
}

impl OPA {
    fn new(query: &str, policy_name: &str, policy: &str, data_json: &str) -> OPA {
        unsafe {
            let ctx = createOPAContext();
            let rego = createRego(
                create_go_string(query),
                create_go_string(policy_name),
                create_go_string(policy),
                create_go_string(data_json)
            );
        }

        OPA { context: ctx, rego, partial_eval: 0 as *mut c_void }
    }

    fn partial_eval(&mut self) {
        unsafe {
            let eval = partialEval(self.rego, self.context);
            self.partial_eval = eval.r0;
        }
    }

    fn eval(&self, input: &str) -> bool {
        let eval = unsafe { eval(self.rego, create_go_string(input), self.partial_eval) };
        if eval.r1.is_null() {
            *unsafe { *(result.r0 as *mut bool) }
        } else {
            false
        }
    }
}

foreigner_class!(
/// Class comment description for Foo.
class OpenPolicyAgent {
    self_type OPA;
    /// some text about the new function
    ///
    /// ```
    /// some markdown example in the text
    /// ```
    ///
    /// @param val - some number
    /// @param name - more information
    constructor OPA::new(query: &str, policy_name: &str, policy: &str, data_json: &str) -> OPA;
    method OPA::partial_eval(&mut self);  alias partialEval;
    method OPA::eval(&self, input: &str) -> Result<bool, &str>;
});

/*
JNIEXPORT jlong JNICALL Java_edu_lasalle_oop_PolicyEvaluator_init
  (JNIEnv *, jclass);

JNIEXPORT jboolean JNICALL Java_edu_lasalle_oop_PolicyEvaluator_evalute
  (JNIEnv *, jclass, jstring, jstring, jstring);

JNIEXPORT jboolean JNICALL Java_edu_lasalle_oop_PolicyEvaluator_stop
  (JNIEnv *, jclass, jlong);
*/