use std::ops::{Deref, DerefMut};

pub unsafe trait VTable {}

pub unsafe trait VData<VT: VTable> {}

#[repr(C)]
pub struct VPair<VD: VData<VT>, VT: VTable> {
    vtable: *const VT,
    pub vdata: VD,
}

impl<VD: VData<VT>, VT: VTable> VPair<VD, VT> {
    #[inline(always)]
    pub const unsafe fn new(vtable: *const VT, vdata: VD) -> VPair<VD, VT> {
        VPair { vtable, vdata }
    }

    #[inline(always)]
    pub fn vtable(&self) -> *const VT {
        self.vtable
    }
}

impl<VD: VData<VT>, VT: VTable> Deref for VPair<VD, VT> {
    type Target = VD;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.vdata
    }
}

impl<VD: VData<VT>, VT: VTable> DerefMut for VPair<VD, VT> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vdata
    }
}

pub type VBox<VD, VT> = Box<VPair<VD, VT>>;
