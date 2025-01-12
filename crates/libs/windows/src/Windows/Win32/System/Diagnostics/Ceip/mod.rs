#[doc = "*Required features: `\"Win32_System_Diagnostics_Ceip\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CeipIsOptedIn() -> super::super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CeipIsOptedIn() -> super::super::super::Foundation::BOOL;
    }
    ::core::mem::transmute(CeipIsOptedIn())
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
