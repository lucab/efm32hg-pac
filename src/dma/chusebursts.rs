#[doc = "Reader of register CHUSEBURSTS"]
pub type R = crate::R<u32, super::CHUSEBURSTS>;
#[doc = "Writer for register CHUSEBURSTS"]
pub type W = crate::W<u32, super::CHUSEBURSTS>;
#[doc = "Register CHUSEBURSTS `reset()`'s with value 0"]
impl crate::ResetValue for super::CHUSEBURSTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH0USEBURSTS`"]
pub type CH0USEBURSTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0USEBURSTS`"]
pub struct CH0USEBURSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0USEBURSTS_W<'a> {
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
#[doc = "Reader of field `CH1USEBURSTS`"]
pub type CH1USEBURSTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1USEBURSTS`"]
pub struct CH1USEBURSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1USEBURSTS_W<'a> {
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
#[doc = "Reader of field `CH2USEBURSTS`"]
pub type CH2USEBURSTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2USEBURSTS`"]
pub struct CH2USEBURSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2USEBURSTS_W<'a> {
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
#[doc = "Reader of field `CH3USEBURSTS`"]
pub type CH3USEBURSTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3USEBURSTS`"]
pub struct CH3USEBURSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3USEBURSTS_W<'a> {
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
#[doc = "Reader of field `CH4USEBURSTS`"]
pub type CH4USEBURSTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH4USEBURSTS`"]
pub struct CH4USEBURSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4USEBURSTS_W<'a> {
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
#[doc = "Reader of field `CH5USEBURSTS`"]
pub type CH5USEBURSTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH5USEBURSTS`"]
pub struct CH5USEBURSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5USEBURSTS_W<'a> {
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
    #[doc = "Bit 0 - Channel 0 Useburst Set"]
    #[inline(always)]
    pub fn ch0usebursts(&self) -> CH0USEBURSTS_R {
        CH0USEBURSTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Useburst Set"]
    #[inline(always)]
    pub fn ch1usebursts(&self) -> CH1USEBURSTS_R {
        CH1USEBURSTS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Useburst Set"]
    #[inline(always)]
    pub fn ch2usebursts(&self) -> CH2USEBURSTS_R {
        CH2USEBURSTS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Useburst Set"]
    #[inline(always)]
    pub fn ch3usebursts(&self) -> CH3USEBURSTS_R {
        CH3USEBURSTS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Useburst Set"]
    #[inline(always)]
    pub fn ch4usebursts(&self) -> CH4USEBURSTS_R {
        CH4USEBURSTS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Useburst Set"]
    #[inline(always)]
    pub fn ch5usebursts(&self) -> CH5USEBURSTS_R {
        CH5USEBURSTS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Useburst Set"]
    #[inline(always)]
    pub fn ch0usebursts(&mut self) -> CH0USEBURSTS_W {
        CH0USEBURSTS_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Useburst Set"]
    #[inline(always)]
    pub fn ch1usebursts(&mut self) -> CH1USEBURSTS_W {
        CH1USEBURSTS_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Useburst Set"]
    #[inline(always)]
    pub fn ch2usebursts(&mut self) -> CH2USEBURSTS_W {
        CH2USEBURSTS_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Useburst Set"]
    #[inline(always)]
    pub fn ch3usebursts(&mut self) -> CH3USEBURSTS_W {
        CH3USEBURSTS_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Useburst Set"]
    #[inline(always)]
    pub fn ch4usebursts(&mut self) -> CH4USEBURSTS_W {
        CH4USEBURSTS_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Useburst Set"]
    #[inline(always)]
    pub fn ch5usebursts(&mut self) -> CH5USEBURSTS_W {
        CH5USEBURSTS_W { w: self }
    }
}
