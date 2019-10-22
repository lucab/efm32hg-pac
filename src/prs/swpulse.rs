#[doc = "Writer for register SWPULSE"]
pub type W = crate::W<u32, super::SWPULSE>;
#[doc = "Register SWPULSE `reset()`'s with value 0"]
impl crate::ResetValue for super::SWPULSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CH0PULSE`"]
pub struct CH0PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH1PULSE`"]
pub struct CH1PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH2PULSE`"]
pub struct CH2PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH3PULSE`"]
pub struct CH3PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH4PULSE`"]
pub struct CH4PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH5PULSE`"]
pub struct CH5PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5PULSE_W<'a> {
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
    #[doc = "Bit 0 - Channel 0 Pulse Generation"]
    #[inline(always)]
    pub fn ch0pulse(&mut self) -> CH0PULSE_W {
        CH0PULSE_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Pulse Generation"]
    #[inline(always)]
    pub fn ch1pulse(&mut self) -> CH1PULSE_W {
        CH1PULSE_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Pulse Generation"]
    #[inline(always)]
    pub fn ch2pulse(&mut self) -> CH2PULSE_W {
        CH2PULSE_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Pulse Generation"]
    #[inline(always)]
    pub fn ch3pulse(&mut self) -> CH3PULSE_W {
        CH3PULSE_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Pulse Generation"]
    #[inline(always)]
    pub fn ch4pulse(&mut self) -> CH4PULSE_W {
        CH4PULSE_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Pulse Generation"]
    #[inline(always)]
    pub fn ch5pulse(&mut self) -> CH5PULSE_W {
        CH5PULSE_W { w: self }
    }
}
