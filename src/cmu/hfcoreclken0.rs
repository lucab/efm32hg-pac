#[doc = "Reader of register HFCORECLKEN0"]
pub type R = crate::R<u32, super::HFCORECLKEN0>;
#[doc = "Writer for register HFCORECLKEN0"]
pub type W = crate::W<u32, super::HFCORECLKEN0>;
#[doc = "Register HFCORECLKEN0 `reset()`'s with value 0"]
impl crate::ResetValue for super::HFCORECLKEN0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AES`"]
pub type AES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AES`"]
pub struct AES_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_W<'a> {
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
#[doc = "Reader of field `DMA`"]
pub type DMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA`"]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
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
#[doc = "Reader of field `LE`"]
pub type LE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE`"]
pub struct LE_W<'a> {
    w: &'a mut W,
}
impl<'a> LE_W<'a> {
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
#[doc = "Reader of field `USBC`"]
pub type USBC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBC`"]
pub struct USBC_W<'a> {
    w: &'a mut W,
}
impl<'a> USBC_W<'a> {
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
#[doc = "Reader of field `USB`"]
pub type USB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB`"]
pub struct USB_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Advanced Encryption Standard Accelerator Clock Enable"]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Direct Memory Access Controller Clock Enable"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Low Energy Peripheral Interface Clock Enable"]
    #[inline(always)]
    pub fn le(&self) -> LE_R {
        LE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Universal Serial Bus Interface Core Clock Enable"]
    #[inline(always)]
    pub fn usbc(&self) -> USBC_R {
        USBC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Universal Serial Bus Interface Clock Enable"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Advanced Encryption Standard Accelerator Clock Enable"]
    #[inline(always)]
    pub fn aes(&mut self) -> AES_W {
        AES_W { w: self }
    }
    #[doc = "Bit 1 - Direct Memory Access Controller Clock Enable"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Bit 2 - Low Energy Peripheral Interface Clock Enable"]
    #[inline(always)]
    pub fn le(&mut self) -> LE_W {
        LE_W { w: self }
    }
    #[doc = "Bit 3 - Universal Serial Bus Interface Core Clock Enable"]
    #[inline(always)]
    pub fn usbc(&mut self) -> USBC_W {
        USBC_W { w: self }
    }
    #[doc = "Bit 4 - Universal Serial Bus Interface Clock Enable"]
    #[inline(always)]
    pub fn usb(&mut self) -> USB_W {
        USB_W { w: self }
    }
}
