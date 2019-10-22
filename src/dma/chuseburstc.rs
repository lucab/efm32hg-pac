#[doc = "Writer for register CHUSEBURSTC"]
pub type W = crate::W<u32, super::CHUSEBURSTC>;
#[doc = "Register CHUSEBURSTC `reset()`'s with value 0"]
impl crate::ResetValue for super::CHUSEBURSTC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CH0USEBURSTC`"]
pub struct CH0USEBURSTC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0USEBURSTC_W<'a> {
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
#[doc = "Write proxy for field `CH1USEBURSTC`"]
pub struct CH1USEBURSTC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1USEBURSTC_W<'a> {
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
#[doc = "Write proxy for field `CH2USEBURSTC`"]
pub struct CH2USEBURSTC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2USEBURSTC_W<'a> {
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
#[doc = "Write proxy for field `CH3USEBURSTC`"]
pub struct CH3USEBURSTC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3USEBURSTC_W<'a> {
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
#[doc = "Write proxy for field `CH4USEBURSTC`"]
pub struct CH4USEBURSTC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4USEBURSTC_W<'a> {
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
#[doc = "Write proxy for field `CH5USEBURSTC`"]
pub struct CH5USEBURSTC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5USEBURSTC_W<'a> {
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
    #[doc = "Bit 0 - Channel 0 Useburst Clear"]
    #[inline(always)]
    pub fn ch0useburstc(&mut self) -> CH0USEBURSTC_W {
        CH0USEBURSTC_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Useburst Clear"]
    #[inline(always)]
    pub fn ch1useburstc(&mut self) -> CH1USEBURSTC_W {
        CH1USEBURSTC_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Useburst Clear"]
    #[inline(always)]
    pub fn ch2useburstc(&mut self) -> CH2USEBURSTC_W {
        CH2USEBURSTC_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Useburst Clear"]
    #[inline(always)]
    pub fn ch3useburstc(&mut self) -> CH3USEBURSTC_W {
        CH3USEBURSTC_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Useburst Clear"]
    #[inline(always)]
    pub fn ch4useburstc(&mut self) -> CH4USEBURSTC_W {
        CH4USEBURSTC_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Useburst Clear"]
    #[inline(always)]
    pub fn ch5useburstc(&mut self) -> CH5USEBURSTC_W {
        CH5USEBURSTC_W { w: self }
    }
}
