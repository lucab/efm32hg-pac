#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Encryption/Decryption Done Interrupt Flag"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 0x01) != 0)
    }
}
