#[doc = "Reader of register ROUTE"]
pub type R = crate::R<u32, super::ROUTE>;
#[doc = "Writer for register ROUTE"]
pub type W = crate::W<u32, super::ROUTE>;
#[doc = "Register ROUTE `reset()`'s with value 0"]
impl crate::ResetValue for super::ROUTE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PHYPEN`"]
pub type PHYPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PHYPEN`"]
pub struct PHYPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYPEN_W<'a> {
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
#[doc = "Reader of field `DMPUPEN`"]
pub type DMPUPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMPUPEN`"]
pub struct DMPUPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMPUPEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - USB PHY Pin Enable"]
    #[inline(always)]
    pub fn phypen(&self) -> PHYPEN_R {
        PHYPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMPU Pin Enable"]
    #[inline(always)]
    pub fn dmpupen(&self) -> DMPUPEN_R {
        DMPUPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB PHY Pin Enable"]
    #[inline(always)]
    pub fn phypen(&mut self) -> PHYPEN_W {
        PHYPEN_W { w: self }
    }
    #[doc = "Bit 2 - DMPU Pin Enable"]
    #[inline(always)]
    pub fn dmpupen(&mut self) -> DMPUPEN_W {
        DMPUPEN_W { w: self }
    }
}
