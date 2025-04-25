
#[repr(C)]
pub struct AppDomain {
    pub vtbl: *const AppDomainVtbl,
}

#[repr(C)]
pub struct AppDomainVtbl {}