#[doc = "Reader of register UNIQUEL"]
pub type R = crate::R<u32, super::UNIQUEL>;
#[doc = "Reader of field `UNIQUEL`"]
pub type UNIQUEL_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Lower part of 64-bit device unique number"]
    #[inline(always)]
    pub fn uniquel(&self) -> UNIQUEL_R {
        UNIQUEL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
