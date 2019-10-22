#[doc = "Reader of register GRSTCTL"]
pub type R = crate::R<u32, super::GRSTCTL>;
#[doc = "Writer for register GRSTCTL"]
pub type W = crate::W<u32, super::GRSTCTL>;
#[doc = "Register GRSTCTL `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::GRSTCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Reader of field `CSFTRST`"]
pub type CSFTRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSFTRST`"]
pub struct CSFTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CSFTRST_W<'a> {
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
#[doc = "Reader of field `PIUFSSFTRST`"]
pub type PIUFSSFTRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIUFSSFTRST`"]
pub struct PIUFSSFTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PIUFSSFTRST_W<'a> {
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
#[doc = "Reader of field `RXFFLSH`"]
pub type RXFFLSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFFLSH`"]
pub struct RXFFLSH_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFFLSH_W<'a> {
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
#[doc = "Reader of field `TXFFLSH`"]
pub type TXFFLSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFFLSH`"]
pub struct TXFFLSH_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFFLSH_W<'a> {
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
#[doc = "TxFIFO Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFNUM_A {
    #[doc = "0: Host mode: Non-periodic TxFIFO flush. Device: Tx FIFO 0 flush"]
    F0,
    #[doc = "1: Host mode: Periodic TxFIFO flush. Device: TXFIFO 1 flush."]
    F1,
    #[doc = "2: Device mode: TXFIFO 2 flush."]
    F2,
    #[doc = "3: Device mode: TXFIFO 3 flush."]
    F3,
    #[doc = "4: Device mode: TXFIFO 4 flush."]
    F4,
    #[doc = "5: Device mode: TXFIFO 5 flush."]
    F5,
    #[doc = "6: Device mode: TXFIFO 6 flush."]
    F6,
    #[doc = "16: Flush all the transmit FIFOs in device or host mode."]
    FALL,
}
impl From<TXFNUM_A> for u8 {
    #[inline(always)]
    fn from(variant: TXFNUM_A) -> Self {
        match variant {
            TXFNUM_A::F0 => 0,
            TXFNUM_A::F1 => 1,
            TXFNUM_A::F2 => 2,
            TXFNUM_A::F3 => 3,
            TXFNUM_A::F4 => 4,
            TXFNUM_A::F5 => 5,
            TXFNUM_A::F6 => 6,
            TXFNUM_A::FALL => 16,
        }
    }
}
#[doc = "Reader of field `TXFNUM`"]
pub type TXFNUM_R = crate::R<u8, TXFNUM_A>;
impl TXFNUM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXFNUM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXFNUM_A::F0),
            1 => Val(TXFNUM_A::F1),
            2 => Val(TXFNUM_A::F2),
            3 => Val(TXFNUM_A::F3),
            4 => Val(TXFNUM_A::F4),
            5 => Val(TXFNUM_A::F5),
            6 => Val(TXFNUM_A::F6),
            16 => Val(TXFNUM_A::FALL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `F0`"]
    #[inline(always)]
    pub fn is_f0(&self) -> bool {
        *self == TXFNUM_A::F0
    }
    #[doc = "Checks if the value of the field is `F1`"]
    #[inline(always)]
    pub fn is_f1(&self) -> bool {
        *self == TXFNUM_A::F1
    }
    #[doc = "Checks if the value of the field is `F2`"]
    #[inline(always)]
    pub fn is_f2(&self) -> bool {
        *self == TXFNUM_A::F2
    }
    #[doc = "Checks if the value of the field is `F3`"]
    #[inline(always)]
    pub fn is_f3(&self) -> bool {
        *self == TXFNUM_A::F3
    }
    #[doc = "Checks if the value of the field is `F4`"]
    #[inline(always)]
    pub fn is_f4(&self) -> bool {
        *self == TXFNUM_A::F4
    }
    #[doc = "Checks if the value of the field is `F5`"]
    #[inline(always)]
    pub fn is_f5(&self) -> bool {
        *self == TXFNUM_A::F5
    }
    #[doc = "Checks if the value of the field is `F6`"]
    #[inline(always)]
    pub fn is_f6(&self) -> bool {
        *self == TXFNUM_A::F6
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == TXFNUM_A::FALL
    }
}
#[doc = "Write proxy for field `TXFNUM`"]
pub struct TXFNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFNUM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFNUM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Host mode: Non-periodic TxFIFO flush. Device: Tx FIFO 0 flush"]
    #[inline(always)]
    pub fn f0(self) -> &'a mut W {
        self.variant(TXFNUM_A::F0)
    }
    #[doc = "Host mode: Periodic TxFIFO flush. Device: TXFIFO 1 flush."]
    #[inline(always)]
    pub fn f1(self) -> &'a mut W {
        self.variant(TXFNUM_A::F1)
    }
    #[doc = "Device mode: TXFIFO 2 flush."]
    #[inline(always)]
    pub fn f2(self) -> &'a mut W {
        self.variant(TXFNUM_A::F2)
    }
    #[doc = "Device mode: TXFIFO 3 flush."]
    #[inline(always)]
    pub fn f3(self) -> &'a mut W {
        self.variant(TXFNUM_A::F3)
    }
    #[doc = "Device mode: TXFIFO 4 flush."]
    #[inline(always)]
    pub fn f4(self) -> &'a mut W {
        self.variant(TXFNUM_A::F4)
    }
    #[doc = "Device mode: TXFIFO 5 flush."]
    #[inline(always)]
    pub fn f5(self) -> &'a mut W {
        self.variant(TXFNUM_A::F5)
    }
    #[doc = "Device mode: TXFIFO 6 flush."]
    #[inline(always)]
    pub fn f6(self) -> &'a mut W {
        self.variant(TXFNUM_A::F6)
    }
    #[doc = "Flush all the transmit FIFOs in device or host mode."]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(TXFNUM_A::FALL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | (((value as u32) & 0x1f) << 6);
        self.w
    }
}
#[doc = "Reader of field `DMAREQ`"]
pub type DMAREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `AHBIDLE`"]
pub type AHBIDLE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Core Soft Reset"]
    #[inline(always)]
    pub fn csftrst(&self) -> CSFTRST_R {
        CSFTRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PIU FS Dedicated Controller Soft Reset"]
    #[inline(always)]
    pub fn piufssftrst(&self) -> PIUFSSFTRST_R {
        PIUFSSFTRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RxFIFO Flush"]
    #[inline(always)]
    pub fn rxfflsh(&self) -> RXFFLSH_R {
        RXFFLSH_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TxFIFO Flush"]
    #[inline(always)]
    pub fn txfflsh(&self) -> TXFFLSH_R {
        TXFFLSH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:10 - TxFIFO Number"]
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - DMA Request Signal"]
    #[inline(always)]
    pub fn dmareq(&self) -> DMAREQ_R {
        DMAREQ_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - AHB Master Idle"]
    #[inline(always)]
    pub fn ahbidle(&self) -> AHBIDLE_R {
        AHBIDLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core Soft Reset"]
    #[inline(always)]
    pub fn csftrst(&mut self) -> CSFTRST_W {
        CSFTRST_W { w: self }
    }
    #[doc = "Bit 1 - PIU FS Dedicated Controller Soft Reset"]
    #[inline(always)]
    pub fn piufssftrst(&mut self) -> PIUFSSFTRST_W {
        PIUFSSFTRST_W { w: self }
    }
    #[doc = "Bit 4 - RxFIFO Flush"]
    #[inline(always)]
    pub fn rxfflsh(&mut self) -> RXFFLSH_W {
        RXFFLSH_W { w: self }
    }
    #[doc = "Bit 5 - TxFIFO Flush"]
    #[inline(always)]
    pub fn txfflsh(&mut self) -> TXFFLSH_W {
        TXFFLSH_W { w: self }
    }
    #[doc = "Bits 6:10 - TxFIFO Number"]
    #[inline(always)]
    pub fn txfnum(&mut self) -> TXFNUM_W {
        TXFNUM_W { w: self }
    }
}
