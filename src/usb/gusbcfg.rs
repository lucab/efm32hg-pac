#[doc = "Reader of register GUSBCFG"]
pub type R = crate::R<u32, super::GUSBCFG>;
#[doc = "Writer for register GUSBCFG"]
pub type W = crate::W<u32, super::GUSBCFG>;
#[doc = "Register GUSBCFG `reset()`'s with value 0x1440"]
impl crate::ResetValue for super::GUSBCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1440
    }
}
#[doc = "Reader of field `TOUTCAL`"]
pub type TOUTCAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOUTCAL`"]
pub struct TOUTCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUTCAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `FSINTF`"]
pub type FSINTF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSINTF`"]
pub struct FSINTF_W<'a> {
    w: &'a mut W,
}
impl<'a> FSINTF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `USBTRDTIM`"]
pub type USBTRDTIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USBTRDTIM`"]
pub struct USBTRDTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> USBTRDTIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | (((value as u32) & 0x0f) << 10);
        self.w
    }
}
#[doc = "Reader of field `TERMSELDLPULSE`"]
pub type TERMSELDLPULSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TERMSELDLPULSE`"]
pub struct TERMSELDLPULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TERMSELDLPULSE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `TXENDDELAY`"]
pub type TXENDDELAY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXENDDELAY`"]
pub struct TXENDDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXENDDELAY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Write proxy for field `CORRUPTTXPKT`"]
pub struct CORRUPTTXPKT_W<'a> {
    w: &'a mut W,
}
impl<'a> CORRUPTTXPKT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Timeout Calibration"]
    #[inline(always)]
    pub fn toutcal(&self) -> TOUTCAL_R {
        TOUTCAL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 5 - Full-Speed Serial Interface Select"]
    #[inline(always)]
    pub fn fsintf(&self) -> FSINTF_R {
        FSINTF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 10:13 - USB Turnaround Time"]
    #[inline(always)]
    pub fn usbtrdtim(&self) -> USBTRDTIM_R {
        USBTRDTIM_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - TermSel DLine Pulsing Selection"]
    #[inline(always)]
    pub fn termseldlpulse(&self) -> TERMSELDLPULSE_R {
        TERMSELDLPULSE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Tx End Delay"]
    #[inline(always)]
    pub fn txenddelay(&self) -> TXENDDELAY_R {
        TXENDDELAY_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Timeout Calibration"]
    #[inline(always)]
    pub fn toutcal(&mut self) -> TOUTCAL_W {
        TOUTCAL_W { w: self }
    }
    #[doc = "Bit 5 - Full-Speed Serial Interface Select"]
    #[inline(always)]
    pub fn fsintf(&mut self) -> FSINTF_W {
        FSINTF_W { w: self }
    }
    #[doc = "Bits 10:13 - USB Turnaround Time"]
    #[inline(always)]
    pub fn usbtrdtim(&mut self) -> USBTRDTIM_W {
        USBTRDTIM_W { w: self }
    }
    #[doc = "Bit 22 - TermSel DLine Pulsing Selection"]
    #[inline(always)]
    pub fn termseldlpulse(&mut self) -> TERMSELDLPULSE_W {
        TERMSELDLPULSE_W { w: self }
    }
    #[doc = "Bit 28 - Tx End Delay"]
    #[inline(always)]
    pub fn txenddelay(&mut self) -> TXENDDELAY_W {
        TXENDDELAY_W { w: self }
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    pub fn corrupttxpkt(&mut self) -> CORRUPTTXPKT_W {
        CORRUPTTXPKT_W { w: self }
    }
}
