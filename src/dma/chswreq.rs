#[doc = "Writer for register CHSWREQ"]
pub type W = crate::W<u32, super::CHSWREQ>;
#[doc = "Register CHSWREQ `reset()`'s with value 0"]
impl crate::ResetValue for super::CHSWREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CH0SWREQ`"]
pub struct CH0SWREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0SWREQ_W<'a> {
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
#[doc = "Write proxy for field `CH1SWREQ`"]
pub struct CH1SWREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1SWREQ_W<'a> {
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
#[doc = "Write proxy for field `CH2SWREQ`"]
pub struct CH2SWREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2SWREQ_W<'a> {
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
#[doc = "Write proxy for field `CH3SWREQ`"]
pub struct CH3SWREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3SWREQ_W<'a> {
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
#[doc = "Write proxy for field `CH4SWREQ`"]
pub struct CH4SWREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4SWREQ_W<'a> {
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
#[doc = "Write proxy for field `CH5SWREQ`"]
pub struct CH5SWREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5SWREQ_W<'a> {
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
    #[doc = "Bit 0 - Channel 0 Software Request"]
    #[inline(always)]
    pub fn ch0swreq(&mut self) -> CH0SWREQ_W {
        CH0SWREQ_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Software Request"]
    #[inline(always)]
    pub fn ch1swreq(&mut self) -> CH1SWREQ_W {
        CH1SWREQ_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Software Request"]
    #[inline(always)]
    pub fn ch2swreq(&mut self) -> CH2SWREQ_W {
        CH2SWREQ_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Software Request"]
    #[inline(always)]
    pub fn ch3swreq(&mut self) -> CH3SWREQ_W {
        CH3SWREQ_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Software Request"]
    #[inline(always)]
    pub fn ch4swreq(&mut self) -> CH4SWREQ_W {
        CH4SWREQ_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Software Request"]
    #[inline(always)]
    pub fn ch5swreq(&mut self) -> CH5SWREQ_W {
        CH5SWREQ_W { w: self }
    }
}
