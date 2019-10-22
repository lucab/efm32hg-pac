#[doc = "Reader of register _RESERVED"]
pub type R = crate::R<u32, super::_RESERVED>;
#[doc = "Reader of field `_RESERVED`"]
pub type _RESERVED_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved"]
    #[inline(always)]
    pub fn _reserved(&self) -> _RESERVED_R {
        _RESERVED_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
