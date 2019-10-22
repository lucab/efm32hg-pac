#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DECRYPT`"]
pub type DECRYPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DECRYPT`"]
pub struct DECRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> DECRYPT_W<'a> {
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
#[doc = "Reader of field `DATASTART`"]
pub type DATASTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATASTART`"]
pub struct DATASTART_W<'a> {
    w: &'a mut W,
}
impl<'a> DATASTART_W<'a> {
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
#[doc = "Reader of field `XORSTART`"]
pub type XORSTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XORSTART`"]
pub struct XORSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> XORSTART_W<'a> {
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
#[doc = "Reader of field `BYTEORDER`"]
pub type BYTEORDER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYTEORDER`"]
pub struct BYTEORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTEORDER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Decryption/Encryption Mode"]
    #[inline(always)]
    pub fn decrypt(&self) -> DECRYPT_R {
        DECRYPT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - AES_DATA Write Start"]
    #[inline(always)]
    pub fn datastart(&self) -> DATASTART_R {
        DATASTART_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AES_XORDATA Write Start"]
    #[inline(always)]
    pub fn xorstart(&self) -> XORSTART_R {
        XORSTART_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Configure byte order in data and key registers"]
    #[inline(always)]
    pub fn byteorder(&self) -> BYTEORDER_R {
        BYTEORDER_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Decryption/Encryption Mode"]
    #[inline(always)]
    pub fn decrypt(&mut self) -> DECRYPT_W {
        DECRYPT_W { w: self }
    }
    #[doc = "Bit 4 - AES_DATA Write Start"]
    #[inline(always)]
    pub fn datastart(&mut self) -> DATASTART_W {
        DATASTART_W { w: self }
    }
    #[doc = "Bit 5 - AES_XORDATA Write Start"]
    #[inline(always)]
    pub fn xorstart(&mut self) -> XORSTART_W {
        XORSTART_W { w: self }
    }
    #[doc = "Bit 6 - Configure byte order in data and key registers"]
    #[inline(always)]
    pub fn byteorder(&mut self) -> BYTEORDER_W {
        BYTEORDER_W { w: self }
    }
}
