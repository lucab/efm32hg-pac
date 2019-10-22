#[doc = "Writer for register CHPRIC"]
pub type W = crate::W<u32, super::CHPRIC>;
#[doc = "Register CHPRIC `reset()`'s with value 0"]
impl crate::ResetValue for super::CHPRIC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CH0PRIC`"]
pub struct CH0PRIC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0PRIC_W<'a> {
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
#[doc = "Write proxy for field `CH1PRIC`"]
pub struct CH1PRIC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1PRIC_W<'a> {
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
#[doc = "Write proxy for field `CH2PRIC`"]
pub struct CH2PRIC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2PRIC_W<'a> {
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
#[doc = "Write proxy for field `CH3PRIC`"]
pub struct CH3PRIC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3PRIC_W<'a> {
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
#[doc = "Write proxy for field `CH4PRIC`"]
pub struct CH4PRIC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4PRIC_W<'a> {
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
#[doc = "Write proxy for field `CH5PRIC`"]
pub struct CH5PRIC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5PRIC_W<'a> {
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
    #[doc = "Bit 0 - Channel 0 High Priority Clear"]
    #[inline(always)]
    pub fn ch0pric(&mut self) -> CH0PRIC_W {
        CH0PRIC_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 High Priority Clear"]
    #[inline(always)]
    pub fn ch1pric(&mut self) -> CH1PRIC_W {
        CH1PRIC_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 High Priority Clear"]
    #[inline(always)]
    pub fn ch2pric(&mut self) -> CH2PRIC_W {
        CH2PRIC_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 High Priority Clear"]
    #[inline(always)]
    pub fn ch3pric(&mut self) -> CH3PRIC_W {
        CH3PRIC_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 High Priority Clear"]
    #[inline(always)]
    pub fn ch4pric(&mut self) -> CH4PRIC_W {
        CH4PRIC_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 High Priority Clear"]
    #[inline(always)]
    pub fn ch5pric(&mut self) -> CH5PRIC_W {
        CH5PRIC_W { w: self }
    }
}
