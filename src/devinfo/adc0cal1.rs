#[doc = "Reader of register ADC0CAL1"]
pub type R = crate::R<u32, super::ADC0CAL1>;
#[doc = "Reader of field `VDD_OFFSET`"]
pub type VDD_OFFSET_R = crate::R<u8, u8>;
#[doc = "Reader of field `VDD_GAIN`"]
pub type VDD_GAIN_R = crate::R<u8, u8>;
#[doc = "Reader of field `5VDIFF_OFFSET`"]
pub type _5VDIFF_OFFSET_R = crate::R<u8, u8>;
#[doc = "Reader of field `5VDIFF_GAIN`"]
pub type _5VDIFF_GAIN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - Offset for VDD reference"]
    #[inline(always)]
    pub fn vdd_offset(&self) -> VDD_OFFSET_R {
        VDD_OFFSET_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Gain for VDD reference"]
    #[inline(always)]
    pub fn vdd_gain(&self) -> VDD_GAIN_R {
        VDD_GAIN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Offset for 5VDIFF reference"]
    #[inline(always)]
    pub fn _5vdiff_offset(&self) -> _5VDIFF_OFFSET_R {
        _5VDIFF_OFFSET_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Gain for 5VDIFF reference"]
    #[inline(always)]
    pub fn _5vdiff_gain(&self) -> _5VDIFF_GAIN_R {
        _5VDIFF_GAIN_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
