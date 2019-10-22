#[doc = "Writer for register CHPRIS"]
pub type W = crate::W<u32, super::CHPRIS>;
#[doc = "Register CHPRIS `reset()`'s with value 0"]
impl crate::ResetValue for super::CHPRIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CH0PRIS`"]
pub struct CH0PRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0PRIS_W<'a> {
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
#[doc = "Write proxy for field `CH1PRIS`"]
pub struct CH1PRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1PRIS_W<'a> {
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
#[doc = "Write proxy for field `CH2PRIS`"]
pub struct CH2PRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2PRIS_W<'a> {
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
#[doc = "Write proxy for field `CH3PRIS`"]
pub struct CH3PRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3PRIS_W<'a> {
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
#[doc = "Write proxy for field `CH4PRIS`"]
pub struct CH4PRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4PRIS_W<'a> {
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
#[doc = "Write proxy for field `CH5PRIS`"]
pub struct CH5PRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5PRIS_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Channel 0 High Priority Set"]
    #[inline(always)]
    pub fn ch0pris(&mut self) -> CH0PRIS_W {
        CH0PRIS_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 High Priority Set"]
    #[inline(always)]
    pub fn ch1pris(&mut self) -> CH1PRIS_W {
        CH1PRIS_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 High Priority Set"]
    #[inline(always)]
    pub fn ch2pris(&mut self) -> CH2PRIS_W {
        CH2PRIS_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 High Priority Set"]
    #[inline(always)]
    pub fn ch3pris(&mut self) -> CH3PRIS_W {
        CH3PRIS_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 High Priority Set"]
    #[inline(always)]
    pub fn ch4pris(&mut self) -> CH4PRIS_W {
        CH4PRIS_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 High Priority Set"]
    #[inline(always)]
    pub fn ch5pris(&mut self) -> CH5PRIS_W {
        CH5PRIS_W { w: self }
    }
}
