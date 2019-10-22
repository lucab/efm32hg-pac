#[doc = "Writer for register CHENC"]
pub type W = crate::W<u32, super::CHENC>;
#[doc = "Register CHENC `reset()`'s with value 0"]
impl crate::ResetValue for super::CHENC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CH0ENC`"]
pub struct CH0ENC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0ENC_W<'a> {
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
#[doc = "Write proxy for field `CH1ENC`"]
pub struct CH1ENC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1ENC_W<'a> {
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
#[doc = "Write proxy for field `CH2ENC`"]
pub struct CH2ENC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2ENC_W<'a> {
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
#[doc = "Write proxy for field `CH3ENC`"]
pub struct CH3ENC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3ENC_W<'a> {
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
#[doc = "Write proxy for field `CH4ENC`"]
pub struct CH4ENC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4ENC_W<'a> {
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
#[doc = "Write proxy for field `CH5ENC`"]
pub struct CH5ENC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5ENC_W<'a> {
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
    #[doc = "Bit 0 - Channel 0 Enable Clear"]
    #[inline(always)]
    pub fn ch0enc(&mut self) -> CH0ENC_W {
        CH0ENC_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Enable Clear"]
    #[inline(always)]
    pub fn ch1enc(&mut self) -> CH1ENC_W {
        CH1ENC_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Enable Clear"]
    #[inline(always)]
    pub fn ch2enc(&mut self) -> CH2ENC_W {
        CH2ENC_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Enable Clear"]
    #[inline(always)]
    pub fn ch3enc(&mut self) -> CH3ENC_W {
        CH3ENC_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Enable Clear"]
    #[inline(always)]
    pub fn ch4enc(&mut self) -> CH4ENC_W {
        CH4ENC_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Enable Clear"]
    #[inline(always)]
    pub fn ch5enc(&mut self) -> CH5ENC_W {
        CH5ENC_W { w: self }
    }
}
