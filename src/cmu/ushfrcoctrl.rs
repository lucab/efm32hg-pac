#[doc = "Reader of register USHFRCOCTRL"]
pub type R = crate::R<u32, super::USHFRCOCTRL>;
#[doc = "Writer for register USHFRCOCTRL"]
pub type W = crate::W<u32, super::USHFRCOCTRL>;
#[doc = "Register USHFRCOCTRL `reset()`'s with value 0x000f_f040"]
impl crate::ResetValue for super::USHFRCOCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x000f_f040
    }
}
#[doc = "Reader of field `TUNING`"]
pub type TUNING_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TUNING`"]
pub struct TUNING_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `DITHEN`"]
pub type DITHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DITHEN`"]
pub struct DITHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DITHEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SUSPEND`"]
pub type SUSPEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUSPEND`"]
pub struct SUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPEND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `TIMEOUT`"]
pub type TIMEOUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMEOUT`"]
pub struct TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 12)) | (((value as u32) & 0xff) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - USHFRCO frequency adjust"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - USHFRCO dither enable"]
    #[inline(always)]
    pub fn dithen(&self) -> DITHEN_R {
        DITHEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - USHFRCO suspend"]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 12:19 - USHFRCO Timeout"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - USHFRCO frequency adjust"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TUNING_W {
        TUNING_W { w: self }
    }
    #[doc = "Bit 8 - USHFRCO dither enable"]
    #[inline(always)]
    pub fn dithen(&mut self) -> DITHEN_W {
        DITHEN_W { w: self }
    }
    #[doc = "Bit 9 - USHFRCO suspend"]
    #[inline(always)]
    pub fn suspend(&mut self) -> SUSPEND_W {
        SUSPEND_W { w: self }
    }
    #[doc = "Bits 12:19 - USHFRCO Timeout"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W { w: self }
    }
}
