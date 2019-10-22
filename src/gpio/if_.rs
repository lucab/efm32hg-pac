#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `EXT`"]
pub type EXT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - External Interrupt Flag n"]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new((self.bits & 0xffff) as u16)
    }
}
