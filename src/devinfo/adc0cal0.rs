#[doc = "Reader of register ADC0CAL0"]
pub type R = crate::R<u32, super::ADC0CAL0>;
#[doc = "Reader of field `1V25_OFFSET`"]
pub type _1V25_OFFSET_R = crate::R<u8, u8>;
#[doc = "Reader of field `1V25_GAIN`"]
pub type _1V25_GAIN_R = crate::R<u8, u8>;
#[doc = "Reader of field `2V5_OFFSET`"]
pub type _2V5_OFFSET_R = crate::R<u8, u8>;
#[doc = "Reader of field `2V5_GAIN`"]
pub type _2V5_GAIN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - Offset for 1V25 reference"]
    #[inline(always)]
    pub fn _1v25_offset(&self) -> _1V25_OFFSET_R {
        _1V25_OFFSET_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Gain for 1V25 reference"]
    #[inline(always)]
    pub fn _1v25_gain(&self) -> _1V25_GAIN_R {
        _1V25_GAIN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Offset for 2V5 reference"]
    #[inline(always)]
    pub fn _2v5_offset(&self) -> _2V5_OFFSET_R {
        _2V5_OFFSET_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Gain for 2V5 reference"]
    #[inline(always)]
    pub fn _2v5_gain(&self) -> _2V5_GAIN_R {
        _2V5_GAIN_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
