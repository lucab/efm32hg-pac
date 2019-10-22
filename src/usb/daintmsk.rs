#[doc = "Reader of register DAINTMSK"]
pub type R = crate::R<u32, super::DAINTMSK>;
#[doc = "Writer for register DAINTMSK"]
pub type W = crate::W<u32, super::DAINTMSK>;
#[doc = "Register DAINTMSK `reset()`'s with value 0"]
impl crate::ResetValue for super::DAINTMSK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INEPMSK0`"]
pub type INEPMSK0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEPMSK0`"]
pub struct INEPMSK0_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPMSK0_W<'a> {
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
#[doc = "Reader of field `INEPMSK1`"]
pub type INEPMSK1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEPMSK1`"]
pub struct INEPMSK1_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPMSK1_W<'a> {
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
#[doc = "Reader of field `INEPMSK2`"]
pub type INEPMSK2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEPMSK2`"]
pub struct INEPMSK2_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPMSK2_W<'a> {
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
#[doc = "Reader of field `INEPMSK3`"]
pub type INEPMSK3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEPMSK3`"]
pub struct INEPMSK3_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPMSK3_W<'a> {
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
#[doc = "Reader of field `OUTEPMSK0`"]
pub type OUTEPMSK0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTEPMSK0`"]
pub struct OUTEPMSK0_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTEPMSK0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `OUTEPMSK1`"]
pub type OUTEPMSK1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTEPMSK1`"]
pub struct OUTEPMSK1_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTEPMSK1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `OUTEPMSK2`"]
pub type OUTEPMSK2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTEPMSK2`"]
pub struct OUTEPMSK2_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTEPMSK2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `OUTEPMSK3`"]
pub type OUTEPMSK3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTEPMSK3`"]
pub struct OUTEPMSK3_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTEPMSK3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - IN Endpoint 0 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk0(&self) -> INEPMSK0_R {
        INEPMSK0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IN Endpoint 1 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk1(&self) -> INEPMSK1_R {
        INEPMSK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IN Endpoint 2 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk2(&self) -> INEPMSK2_R {
        INEPMSK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IN Endpoint 3 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk3(&self) -> INEPMSK3_R {
        INEPMSK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - OUT Endpoint 0 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk0(&self) -> OUTEPMSK0_R {
        OUTEPMSK0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - OUT Endpoint 1 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk1(&self) -> OUTEPMSK1_R {
        OUTEPMSK1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - OUT Endpoint 2 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk2(&self) -> OUTEPMSK2_R {
        OUTEPMSK2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - OUT Endpoint 3 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk3(&self) -> OUTEPMSK3_R {
        OUTEPMSK3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IN Endpoint 0 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk0(&mut self) -> INEPMSK0_W {
        INEPMSK0_W { w: self }
    }
    #[doc = "Bit 1 - IN Endpoint 1 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk1(&mut self) -> INEPMSK1_W {
        INEPMSK1_W { w: self }
    }
    #[doc = "Bit 2 - IN Endpoint 2 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk2(&mut self) -> INEPMSK2_W {
        INEPMSK2_W { w: self }
    }
    #[doc = "Bit 3 - IN Endpoint 3 Interrupt mask Bit"]
    #[inline(always)]
    pub fn inepmsk3(&mut self) -> INEPMSK3_W {
        INEPMSK3_W { w: self }
    }
    #[doc = "Bit 16 - OUT Endpoint 0 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk0(&mut self) -> OUTEPMSK0_W {
        OUTEPMSK0_W { w: self }
    }
    #[doc = "Bit 17 - OUT Endpoint 1 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk1(&mut self) -> OUTEPMSK1_W {
        OUTEPMSK1_W { w: self }
    }
    #[doc = "Bit 18 - OUT Endpoint 2 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk2(&mut self) -> OUTEPMSK2_W {
        OUTEPMSK2_W { w: self }
    }
    #[doc = "Bit 19 - OUT Endpoint 3 Interrupt mask Bit"]
    #[inline(always)]
    pub fn outepmsk3(&mut self) -> OUTEPMSK3_W {
        OUTEPMSK3_W { w: self }
    }
}
