#[doc = "Reader of register ALTCTRLBASE"]
pub type R = crate::R<u32, super::ALTCTRLBASE>;
#[doc = "Reader of field `ALTCTRLBASE`"]
pub type ALTCTRLBASE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel Alternate Control Data Base Pointer"]
    #[inline(always)]
    pub fn altctrlbase(&self) -> ALTCTRLBASE_R {
        ALTCTRLBASE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
