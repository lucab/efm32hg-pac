#[doc = "Reader of register USHFRCOCAL0"]
pub type R = crate::R<u32, super::USHFRCOCAL0>;
#[doc = "Reader of field `BAND24_TUNING`"]
pub type BAND24_TUNING_R = crate::R<u8, u8>;
#[doc = "Reader of field `BAND24_FINETUNING`"]
pub type BAND24_FINETUNING_R = crate::R<u8, u8>;
#[doc = "Reader of field `BAND48_TUNING`"]
pub type BAND48_TUNING_R = crate::R<u8, u8>;
#[doc = "Reader of field `BAND48_FINETUNING`"]
pub type BAND48_FINETUNING_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - 24 MHz TUNING value for USFRCO"]
    #[inline(always)]
    pub fn band24_tuning(&self) -> BAND24_TUNING_R {
        BAND24_TUNING_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - 24 MHz FINETUNING value for USFRCO"]
    #[inline(always)]
    pub fn band24_finetuning(&self) -> BAND24_FINETUNING_R {
        BAND24_FINETUNING_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:22 - 24 MHz TUNING value for USFRCO"]
    #[inline(always)]
    pub fn band48_tuning(&self) -> BAND48_TUNING_R {
        BAND48_TUNING_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:29 - 24 MHz FINETUNING value for USFRCO"]
    #[inline(always)]
    pub fn band48_finetuning(&self) -> BAND48_FINETUNING_R {
        BAND48_FINETUNING_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
