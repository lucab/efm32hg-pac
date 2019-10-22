#[doc = "Reader of register HFRCOCAL1"]
pub type R = crate::R<u32, super::HFRCOCAL1>;
#[doc = "Reader of field `BAND21`"]
pub type BAND21_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - 21MHz tuning value for HFRCO"]
    #[inline(always)]
    pub fn band21(&self) -> BAND21_R {
        BAND21_R::new((self.bits & 0xff) as u8)
    }
}
