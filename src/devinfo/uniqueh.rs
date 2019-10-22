#[doc = "Reader of register UNIQUEH"]
pub type R = crate::R<u32, super::UNIQUEH>;
#[doc = "Reader of field `UNIQUEH`"]
pub type UNIQUEH_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - High part of 64-bit device unique number"]
    #[inline(always)]
    pub fn uniqueh(&self) -> UNIQUEH_R {
        UNIQUEH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
