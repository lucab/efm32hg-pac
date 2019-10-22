#[doc = "Writer for register CHALTS"]
pub type W = crate::W<u32, super::CHALTS>;
#[doc = "Register CHALTS `reset()`'s with value 0"]
impl crate::ResetValue for super::CHALTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CH0ALTS`"]
pub struct CH0ALTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0ALTS_W<'a> {
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
#[doc = "Write proxy for field `CH1ALTS`"]
pub struct CH1ALTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1ALTS_W<'a> {
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
#[doc = "Write proxy for field `CH2ALTS`"]
pub struct CH2ALTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2ALTS_W<'a> {
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
#[doc = "Write proxy for field `CH3ALTS`"]
pub struct CH3ALTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3ALTS_W<'a> {
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
#[doc = "Write proxy for field `CH4ALTS`"]
pub struct CH4ALTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4ALTS_W<'a> {
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
#[doc = "Write proxy for field `CH5ALTS`"]
pub struct CH5ALTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5ALTS_W<'a> {
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
    #[doc = "Bit 0 - Channel 0 Alternate Structure Set"]
    #[inline(always)]
    pub fn ch0alts(&mut self) -> CH0ALTS_W {
        CH0ALTS_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Alternate Structure Set"]
    #[inline(always)]
    pub fn ch1alts(&mut self) -> CH1ALTS_W {
        CH1ALTS_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Alternate Structure Set"]
    #[inline(always)]
    pub fn ch2alts(&mut self) -> CH2ALTS_W {
        CH2ALTS_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Alternate Structure Set"]
    #[inline(always)]
    pub fn ch3alts(&mut self) -> CH3ALTS_W {
        CH3ALTS_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Alternate Structure Set"]
    #[inline(always)]
    pub fn ch4alts(&mut self) -> CH4ALTS_W {
        CH4ALTS_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Alternate Structure Set"]
    #[inline(always)]
    pub fn ch5alts(&mut self) -> CH5ALTS_W {
        CH5ALTS_W { w: self }
    }
}
