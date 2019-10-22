#[doc = "Reader of register ADC0CAL2"]
pub type R = crate::R<u32, super::ADC0CAL2>;
#[doc = "Reader of field `2XVDDVSS_OFFSET`"]
pub type _2XVDDVSS_OFFSET_R = crate::R<u8, u8>;
#[doc = "Reader of field `TEMP1V25`"]
pub type TEMP1V25_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:6 - Offset for 2XVDDVSS reference"]
    #[inline(always)]
    pub fn _2xvddvss_offset(&self) -> _2XVDDVSS_OFFSET_R {
        _2XVDDVSS_OFFSET_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 20:31 - Temperature reading at 1V25 reference"]
    #[inline(always)]
    pub fn temp1v25(&self) -> TEMP1V25_R {
        TEMP1V25_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
