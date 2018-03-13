extern crate std;
use std::os::raw::c_char;
use std::convert::From;

cpp_class!(pub struct QByteArray, "QByteArray");
impl QByteArray {
    pub fn to_string(&self) -> String {
        unsafe {
            let c_ptr = cpp!([self as "const QByteArray*"] -> *const c_char as "const char*"
                { return self->constData(); });
            std::ffi::CStr::from_ptr(c_ptr).to_string_lossy().into_owned()
        }
    }
}
impl Default for QByteArray {
    fn default() -> QByteArray {
        unsafe {cpp!([] -> QByteArray as "QByteArray" { return QByteArray(); })}
    }
}
impl<'a> From<&'a str> for QByteArray {
    fn from(s : &'a str) -> QByteArray {
        let len = s.len();
        let ptr = s.as_ptr();
        unsafe { cpp!([len as "size_t", ptr as "char*"] -> QByteArray as "QByteArray"
        { return QByteArray(ptr, len); })}
    }
}
// impl From<String> for QByteArray {
//     fn from(s : String) -> QByteArray {
//         let s : &str = &s;
//         s.into()
//     }
// }


cpp_class!(pub struct QString, "QString");
impl QString {
    pub fn to_string(&self) -> String {
        unsafe {
            let ba = cpp!([self as "const QString*"] -> QByteArray as "QByteArray"
                { return self->toUtf8(); });
            ba.to_string()
        }
    }
}
impl Default for QString {
    fn default() -> QString {
        unsafe {cpp!([] -> QString as "QString" { return QString(); })}
    }
}
impl<'a> From<&'a str> for QString {
    fn from(s : &'a str) -> QString {
        let len = s.len();
        let ptr = s.as_ptr();
        unsafe { cpp!([len as "size_t", ptr as "char*"] -> QString as "QString"
        { return QString::fromUtf8(ptr, len); })}
    }
}


cpp_class!(pub struct QVariant, "QVariant");
impl QVariant {
    pub fn to_qbytearray(&self) -> QByteArray {
        // FIXME
        unsafe {
            cpp!([self as "const QVariant*"] -> QByteArray as "QByteArray" { return self->toByteArray(); })
        }
    }

    pub fn from_qbytearray(a : QByteArray) -> QVariant {
        unsafe {cpp!([a as "QByteArray"] -> QVariant as "QVariant" { return QVariant(a); })}
    }
}
impl Default for QVariant {
    fn default() -> QVariant {
        unsafe {cpp!([] -> QVariant as "QVariant" { return QVariant(); })}
    }
}
impl From<QString> for QVariant {
    fn from(a : QString) -> QVariant {
        unsafe {cpp!([a as "QString"] -> QVariant as "QVariant" { return QVariant(a); })}
    }
}
impl From<i32> for QVariant {
    fn from(a : i32) -> QVariant {
        unsafe {cpp!([a as "int"] -> QVariant as "QVariant" { return QVariant(a); })}
    }
}
impl From<u32> for QVariant {
    fn from(a : u32) -> QVariant {
        unsafe {cpp!([a as "uint"] -> QVariant as "QVariant" { return QVariant(a); })}
    }
}


cpp_class!(pub struct QModelIndex, "QModelIndex");
impl QModelIndex {
    pub fn row(&self) -> i32 {
        unsafe {
            cpp!([self as "const QModelIndex*"] -> i32 as "int" { return self->row(); })
        }
    }
}
impl Default for QModelIndex {
    fn default() -> QModelIndex {
        unsafe {cpp!([] -> QModelIndex as "QModelIndex" { return QModelIndex(); })}
    }
}


