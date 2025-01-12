#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateAppContainerProfile(pszappcontainername: ::windows::core::PCWSTR, pszdisplayname: ::windows::core::PCWSTR, pszdescription: ::windows::core::PCWSTR, pcapabilities: &[super::SID_AND_ATTRIBUTES]) -> ::windows::core::Result<super::super::Foundation::PSID> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateAppContainerProfile(pszappcontainername: ::windows::core::PCWSTR, pszdisplayname: ::windows::core::PCWSTR, pszdescription: ::windows::core::PCWSTR, pcapabilities: *const super::SID_AND_ATTRIBUTES, dwcapabilitycount: u32, ppsidappcontainersid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::PSID>::zeroed();
    CreateAppContainerProfile(::core::mem::transmute(pszappcontainername), ::core::mem::transmute(pszdisplayname), ::core::mem::transmute(pszdescription), ::core::mem::transmute(::windows::core::as_ptr_or_null(pcapabilities)), pcapabilities.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::PSID>(result__)
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`*"]
#[inline]
pub unsafe fn DeleteAppContainerProfile(pszappcontainername: ::windows::core::PCWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DeleteAppContainerProfile(pszappcontainername: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    DeleteAppContainerProfile(::core::mem::transmute(pszappcontainername)).ok()
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeriveAppContainerSidFromAppContainerName(pszappcontainername: ::windows::core::PCWSTR) -> ::windows::core::Result<super::super::Foundation::PSID> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DeriveAppContainerSidFromAppContainerName(pszappcontainername: ::windows::core::PCWSTR, ppsidappcontainersid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::PSID>::zeroed();
    DeriveAppContainerSidFromAppContainerName(::core::mem::transmute(pszappcontainername), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::PSID>(result__)
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName<'a, P0>(psidappcontainersid: P0, pszrestrictedappcontainername: ::windows::core::PCWSTR) -> ::windows::core::Result<super::super::Foundation::PSID>
where
    P0: ::std::convert::Into<super::super::Foundation::PSID>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(psidappcontainersid: super::super::Foundation::PSID, pszrestrictedappcontainername: ::windows::core::PCWSTR, ppsidrestrictedappcontainersid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::PSID>::zeroed();
    DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(psidappcontainersid.into(), ::core::mem::transmute(pszrestrictedappcontainername), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::PSID>(result__)
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`*"]
#[inline]
pub unsafe fn GetAppContainerFolderPath(pszappcontainersid: ::windows::core::PCWSTR) -> ::windows::core::Result<::windows::core::PWSTR> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetAppContainerFolderPath(pszappcontainersid: ::windows::core::PCWSTR, ppszpath: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::<::windows::core::PWSTR>::zeroed();
    GetAppContainerFolderPath(::core::mem::transmute(pszappcontainersid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAppContainerNamedObjectPath<'a, P0, P1>(token: P0, appcontainersid: P1, objectpath: &mut [u16], returnlength: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::PSID>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetAppContainerNamedObjectPath(token: super::super::Foundation::HANDLE, appcontainersid: super::super::Foundation::PSID, objectpathlength: u32, objectpath: ::windows::core::PWSTR, returnlength: *mut u32) -> super::super::Foundation::BOOL;
    }
    ::core::mem::transmute(GetAppContainerNamedObjectPath(token.into(), appcontainersid.into(), objectpath.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(objectpath)), ::core::mem::transmute(returnlength)))
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetAppContainerRegistryLocation(desiredaccess: u32) -> ::windows::core::Result<super::super::System::Registry::HKEY> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetAppContainerRegistryLocation(desiredaccess: u32, phappcontainerkey: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::<super::super::System::Registry::HKEY>::zeroed();
    GetAppContainerRegistryLocation(::core::mem::transmute(desiredaccess), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Registry::HKEY>(result__)
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IIsolatedAppLauncher(::windows::core::IUnknown);
impl IIsolatedAppLauncher {
    #[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Launch(&self, appusermodelid: ::windows::core::PCWSTR, arguments: ::windows::core::PCWSTR, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Launch)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(appusermodelid), ::core::mem::transmute(arguments), ::core::mem::transmute(telemetryparameters)).ok()
    }
}
impl ::core::convert::From<IIsolatedAppLauncher> for ::windows::core::IUnknown {
    fn from(value: IIsolatedAppLauncher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IIsolatedAppLauncher> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IIsolatedAppLauncher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IIsolatedAppLauncher> for ::windows::core::IUnknown {
    fn from(value: &IIsolatedAppLauncher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IIsolatedAppLauncher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IIsolatedAppLauncher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IIsolatedAppLauncher {}
impl ::core::fmt::Debug for IIsolatedAppLauncher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIsolatedAppLauncher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IIsolatedAppLauncher {
    type Vtable = IIsolatedAppLauncher_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf686878f_7b42_4cc4_96fb_f4f3b6e3d24d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedAppLauncher_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Launch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appusermodelid: ::windows::core::PCWSTR, arguments: ::windows::core::PCWSTR, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Launch: usize,
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsProcessInIsolatedContainer() -> ::windows::core::Result<super::super::Foundation::BOOL> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsProcessInIsolatedContainer(isprocessinisolatedcontainer: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
    IsProcessInIsolatedContainer(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsProcessInIsolatedWindowsEnvironment() -> ::windows::core::Result<super::super::Foundation::BOOL> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsProcessInIsolatedWindowsEnvironment(isprocessinisolatedwindowsenvironment: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
    IsProcessInIsolatedWindowsEnvironment(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsProcessInWDAGContainer(reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<super::super::Foundation::BOOL> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsProcessInWDAGContainer(reserved: *const ::core::ffi::c_void, isprocessinwdagcontainer: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
    IsProcessInWDAGContainer(::core::mem::transmute(reserved), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
}
pub const IsolatedAppLauncher: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc812430_e75e_4fd1_9641_1f9f1e2d9a1f);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IsolatedAppLauncherTelemetryParameters {
    pub EnableForLaunch: super::super::Foundation::BOOL,
    pub CorrelationGUID: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IsolatedAppLauncherTelemetryParameters {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IsolatedAppLauncherTelemetryParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IsolatedAppLauncherTelemetryParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IsolatedAppLauncherTelemetryParameters").field("EnableForLaunch", &self.EnableForLaunch).field("CorrelationGUID", &self.CorrelationGUID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for IsolatedAppLauncherTelemetryParameters {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IsolatedAppLauncherTelemetryParameters {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IsolatedAppLauncherTelemetryParameters>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IsolatedAppLauncherTelemetryParameters {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IsolatedAppLauncherTelemetryParameters {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
