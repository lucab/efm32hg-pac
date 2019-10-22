#[doc = "Reader of register SWLEVEL"]
pub type R = crate::R<u32, super::SWLEVEL>;
#[doc = "Writer for register SWLEVEL"]
pub type W = crate::W<u32, super::SWLEVEL>;
#[doc = "Register SWLEVEL `reset()`'s with value 0"]
impl crate::ResetValue for super::SWLEVEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH0LEVEL`"]
pub type CH0LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0LEVEL`"]
pub struct CH0LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0LEVEL_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `CH1LEVEL`"]
pub type CH1LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1LEVEL`"]
pub struct CH1LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1LEVEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CH2LEVEL`"]
pub type CH2LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2LEVEL`"]
pub struct CH2LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2LEVEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CH3LEVEL`"]
pub type CH3LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3LEVEL`"]
pub struct CH3LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3LEVEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `CH4LEVEL`"]
pub type CH4LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH4LEVEL`"]
pub struct CH4LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4LEVEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CH5LEVEL`"]
pub type CH5LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH5LEVEL`"]
pub struct CH5LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5LEVEL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Channel 0 Software Level"]
    #[inline(always)]
    pub fn ch0level(&self) -> CH0LEVEL_R {
        CH0LEVEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Level"]
    #[inline(always)]
    pub fn ch1level(&self) -> CH1LEVEL_R {
        CH1LEVEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Software Level"]
    #[inline(always)]
    pub fn ch2level(&self) -> CH2LEVEL_R {
        CH2LEVEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Software Level"]
    #[inline(always)]
    pub fn ch3level(&self) -> CH3LEVEL_R {
        CH3LEVEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Software Level"]
    #[inline(always)]
    pub fn ch4level(&self) -> CH4LEVEL_R {
        CH4LEVEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Software Level"]
    #[inline(always)]
    pub fn ch5level(&self) -> CH5LEVEL_R {
        CH5LEVEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Software Level"]
    #[inline(always)]
    pub fn ch0level(&mut self) -> CH0LEVEL_W {
        CH0LEVEL_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Software Level"]
    #[inline(always)]
    pub fn ch1level(&mut self) -> CH1LEVEL_W {
        CH1LEVEL_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Software Level"]
    #[inline(always)]
    pub fn ch2level(&mut self) -> CH2LEVEL_W {
        CH2LEVEL_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Software Level"]
    #[inline(always)]
    pub fn ch3level(&mut self) -> CH3LEVEL_W {
        CH3LEVEL_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Software Level"]
    #[inline(always)]
    pub fn ch4level(&mut self) -> CH4LEVEL_W {
        CH4LEVEL_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Software Level"]
    #[inline(always)]
    pub fn ch5level(&mut self) -> CH5LEVEL_W {
        CH5LEVEL_W { w: self }
    }
}
