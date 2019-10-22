#[doc = "Writer for register CHENS"]
pub type W = crate::W<u32, super::CHENS>;
#[doc = "Register CHENS `reset()`'s with value 0"]
impl crate::ResetValue for super::CHENS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CH0ENS`"]
pub struct CH0ENS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0ENS_W<'a> {
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
#[doc = "Write proxy for field `CH1ENS`"]
pub struct CH1ENS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1ENS_W<'a> {
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
#[doc = "Write proxy for field `CH2ENS`"]
pub struct CH2ENS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2ENS_W<'a> {
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
#[doc = "Write proxy for field `CH3ENS`"]
pub struct CH3ENS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3ENS_W<'a> {
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
#[doc = "Write proxy for field `CH4ENS`"]
pub struct CH4ENS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4ENS_W<'a> {
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
#[doc = "Write proxy for field `CH5ENS`"]
pub struct CH5ENS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5ENS_W<'a> {
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
    #[doc = "Bit 0 - Channel 0 Enable Set"]
    #[inline(always)]
    pub fn ch0ens(&mut self) -> CH0ENS_W {
        CH0ENS_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Enable Set"]
    #[inline(always)]
    pub fn ch1ens(&mut self) -> CH1ENS_W {
        CH1ENS_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Enable Set"]
    #[inline(always)]
    pub fn ch2ens(&mut self) -> CH2ENS_W {
        CH2ENS_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Enable Set"]
    #[inline(always)]
    pub fn ch3ens(&mut self) -> CH3ENS_W {
        CH3ENS_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Enable Set"]
    #[inline(always)]
    pub fn ch4ens(&mut self) -> CH4ENS_W {
        CH4ENS_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Enable Set"]
    #[inline(always)]
    pub fn ch5ens(&mut self) -> CH5ENS_W {
        CH5ENS_W { w: self }
    }
}
