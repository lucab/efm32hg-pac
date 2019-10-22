#[doc = "Reader of register DIEP1_TXFSTS"]
pub type R = crate::R<u32, super::DIEP1_TXFSTS>;
#[doc = "Reader of field `SPCAVAIL`"]
pub type SPCAVAIL_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - TxFIFO Space Available"]
    #[inline(always)]
    pub fn spcavail(&self) -> SPCAVAIL_R {
        SPCAVAIL_R::new((self.bits & 0xffff) as u16)
    }
}
