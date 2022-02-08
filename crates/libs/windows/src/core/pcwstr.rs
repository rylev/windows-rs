use super::*;

#[repr(transparent)]
pub struct PCWSTR(pub *const u16);
impl PCWSTR {
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
}
impl ::core::default::Default for PCWSTR {
    fn default() -> Self {
        Self(::core::ptr::null())
    }
}
impl ::core::clone::Clone for PCWSTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for PCWSTR {}
impl ::core::cmp::PartialEq for PCWSTR {
    fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
    }
}
impl ::core::cmp::Eq for PCWSTR {}
impl ::core::fmt::Debug for PCWSTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PCWSTR").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for PCWSTR {
    type Abi = Self;

    #[cfg(feature = "alloc")]
    unsafe fn drop_param(param: &mut ::windows::core::Param<'_, Self>) {
        if let ::windows::core::Param::Boxed(value) = param {
            if !value.is_null() {
                ::windows::core::alloc::boxed::Box::from_raw(value.0 as *mut u16);
            }
        }
    }
}
#[cfg(feature = "alloc")]
impl<'a> ::windows::core::IntoParam<'a, PCWSTR> for &str {
    fn into_param(self) -> ::windows::core::Param<'a, PCWSTR> {
        ::windows::core::Param::Boxed(PCWSTR(::windows::core::alloc::boxed::Box::<[u16]>::into_raw(self.encode_utf16().chain(::core::iter::once(0)).collect::<::windows::core::alloc::vec::Vec<u16>>().into_boxed_slice()) as _))
    }
}
#[cfg(feature = "alloc")]
impl<'a> ::windows::core::IntoParam<'a, PCWSTR> for ::windows::core::alloc::string::String {
    fn into_param(self) -> ::windows::core::Param<'a, PCWSTR> {
        ::windows::core::IntoParam::into_param(self.as_str())
    }
}
impl<'a> ::windows::core::IntoParam<'a, PCWSTR> for &::std::ffi::OsStr {
    fn into_param(self) -> ::windows::core::Param<'a, PCWSTR> {
        use ::std::os::windows::ffi::OsStrExt;
        ::windows::core::Param::Boxed(PCWSTR(::windows::core::alloc::boxed::Box::<[u16]>::into_raw(self.encode_wide().chain(::core::iter::once(0)).collect::<::windows::core::alloc::vec::Vec<u16>>().into_boxed_slice()) as _))
    }
}
impl<'a> ::windows::core::IntoParam<'a, PCWSTR> for ::std::ffi::OsString {
    fn into_param(self) -> ::windows::core::Param<'a, PCWSTR> {
        ::windows::core::IntoParam::into_param(self.as_os_str())
    }
}
