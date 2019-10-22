#[doc = "Reader of register DCTL"]
pub type R = crate::R<u32, super::DCTL>;
#[doc = "Writer for register DCTL"]
pub type W = crate::W<u32, super::DCTL>;
#[doc = "Register DCTL `reset()`'s with value 0x02"]
impl crate::ResetValue for super::DCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `RMTWKUPSIG`"]
pub type RMTWKUPSIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMTWKUPSIG`"]
pub struct RMTWKUPSIG_W<'a> {
    w: &'a mut W,
}
impl<'a> RMTWKUPSIG_W<'a> {
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
#[doc = "Reader of field `SFTDISCON`"]
pub type SFTDISCON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SFTDISCON`"]
pub struct SFTDISCON_W<'a> {
    w: &'a mut W,
}
impl<'a> SFTDISCON_W<'a> {
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
#[doc = "Reader of field `GNPINNAKSTS`"]
pub type GNPINNAKSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `GOUTNAKSTS`"]
pub type GOUTNAKSTS_R = crate::R<bool, bool>;
#[doc = "Test Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTCTL_A {
    #[doc = "0: Test mode disabled."]
    DISABLE,
    #[doc = "1: Test_J mode."]
    J,
    #[doc = "2: Test_K mode."]
    K,
    #[doc = "3: Test_SE0_NAK mode."]
    SE0NAK,
    #[doc = "4: Test_Packet mode."]
    PACKET,
    #[doc = "5: Test_Force_Enable."]
    FORCE,
}
impl From<TSTCTL_A> for u8 {
    #[inline(always)]
    fn from(variant: TSTCTL_A) -> Self {
        match variant {
            TSTCTL_A::DISABLE => 0,
            TSTCTL_A::J => 1,
            TSTCTL_A::K => 2,
            TSTCTL_A::SE0NAK => 3,
            TSTCTL_A::PACKET => 4,
            TSTCTL_A::FORCE => 5,
        }
    }
}
#[doc = "Reader of field `TSTCTL`"]
pub type TSTCTL_R = crate::R<u8, TSTCTL_A>;
impl TSTCTL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSTCTL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSTCTL_A::DISABLE),
            1 => Val(TSTCTL_A::J),
            2 => Val(TSTCTL_A::K),
            3 => Val(TSTCTL_A::SE0NAK),
            4 => Val(TSTCTL_A::PACKET),
            5 => Val(TSTCTL_A::FORCE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TSTCTL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `J`"]
    #[inline(always)]
    pub fn is_j(&self) -> bool {
        *self == TSTCTL_A::J
    }
    #[doc = "Checks if the value of the field is `K`"]
    #[inline(always)]
    pub fn is_k(&self) -> bool {
        *self == TSTCTL_A::K
    }
    #[doc = "Checks if the value of the field is `SE0NAK`"]
    #[inline(always)]
    pub fn is_se0nak(&self) -> bool {
        *self == TSTCTL_A::SE0NAK
    }
    #[doc = "Checks if the value of the field is `PACKET`"]
    #[inline(always)]
    pub fn is_packet(&self) -> bool {
        *self == TSTCTL_A::PACKET
    }
    #[doc = "Checks if the value of the field is `FORCE`"]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        *self == TSTCTL_A::FORCE
    }
}
#[doc = "Write proxy for field `TSTCTL`"]
pub struct TSTCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTCTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSTCTL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Test mode disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TSTCTL_A::DISABLE)
    }
    #[doc = "Test_J mode."]
    #[inline(always)]
    pub fn j(self) -> &'a mut W {
        self.variant(TSTCTL_A::J)
    }
    #[doc = "Test_K mode."]
    #[inline(always)]
    pub fn k(self) -> &'a mut W {
        self.variant(TSTCTL_A::K)
    }
    #[doc = "Test_SE0_NAK mode."]
    #[inline(always)]
    pub fn se0nak(self) -> &'a mut W {
        self.variant(TSTCTL_A::SE0NAK)
    }
    #[doc = "Test_Packet mode."]
    #[inline(always)]
    pub fn packet(self) -> &'a mut W {
        self.variant(TSTCTL_A::PACKET)
    }
    #[doc = "Test_Force_Enable."]
    #[inline(always)]
    pub fn force(self) -> &'a mut W {
        self.variant(TSTCTL_A::FORCE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `SGNPINNAK`"]
pub struct SGNPINNAK_W<'a> {
    w: &'a mut W,
}
impl<'a> SGNPINNAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Write proxy for field `CGNPINNAK`"]
pub struct CGNPINNAK_W<'a> {
    w: &'a mut W,
}
impl<'a> CGNPINNAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `SGOUTNAK`"]
pub struct SGOUTNAK_W<'a> {
    w: &'a mut W,
}
impl<'a> SGOUTNAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Write proxy for field `CGOUTNAK`"]
pub struct CGOUTNAK_W<'a> {
    w: &'a mut W,
}
impl<'a> CGOUTNAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `PWRONPRGDONE`"]
pub type PWRONPRGDONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWRONPRGDONE`"]
pub struct PWRONPRGDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRONPRGDONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `IGNRFRMNUM`"]
pub type IGNRFRMNUM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IGNRFRMNUM`"]
pub struct IGNRFRMNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> IGNRFRMNUM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `NAKONBBLE`"]
pub type NAKONBBLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NAKONBBLE`"]
pub struct NAKONBBLE_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKONBBLE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Remote Wakeup Signaling"]
    #[inline(always)]
    pub fn rmtwkupsig(&self) -> RMTWKUPSIG_R {
        RMTWKUPSIG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Soft Disconnect"]
    #[inline(always)]
    pub fn sftdiscon(&self) -> SFTDISCON_R {
        SFTDISCON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Global Non-periodic IN NAK Status"]
    #[inline(always)]
    pub fn gnpinnaksts(&self) -> GNPINNAKSTS_R {
        GNPINNAKSTS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Global OUT NAK Status"]
    #[inline(always)]
    pub fn goutnaksts(&self) -> GOUTNAKSTS_R {
        GOUTNAKSTS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Test Control"]
    #[inline(always)]
    pub fn tstctl(&self) -> TSTCTL_R {
        TSTCTL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Power-On Programming Done"]
    #[inline(always)]
    pub fn pwronprgdone(&self) -> PWRONPRGDONE_R {
        PWRONPRGDONE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Ignore Frame number For Isochronous End points"]
    #[inline(always)]
    pub fn ignrfrmnum(&self) -> IGNRFRMNUM_R {
        IGNRFRMNUM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - NAK on Babble Error"]
    #[inline(always)]
    pub fn nakonbble(&self) -> NAKONBBLE_R {
        NAKONBBLE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remote Wakeup Signaling"]
    #[inline(always)]
    pub fn rmtwkupsig(&mut self) -> RMTWKUPSIG_W {
        RMTWKUPSIG_W { w: self }
    }
    #[doc = "Bit 1 - Soft Disconnect"]
    #[inline(always)]
    pub fn sftdiscon(&mut self) -> SFTDISCON_W {
        SFTDISCON_W { w: self }
    }
    #[doc = "Bits 4:6 - Test Control"]
    #[inline(always)]
    pub fn tstctl(&mut self) -> TSTCTL_W {
        TSTCTL_W { w: self }
    }
    #[doc = "Bit 7 - Set Global Non-periodic IN NAK"]
    #[inline(always)]
    pub fn sgnpinnak(&mut self) -> SGNPINNAK_W {
        SGNPINNAK_W { w: self }
    }
    #[doc = "Bit 8 - Clear Global Non-periodic IN NAK"]
    #[inline(always)]
    pub fn cgnpinnak(&mut self) -> CGNPINNAK_W {
        CGNPINNAK_W { w: self }
    }
    #[doc = "Bit 9 - Set Global OUT NAK"]
    #[inline(always)]
    pub fn sgoutnak(&mut self) -> SGOUTNAK_W {
        SGOUTNAK_W { w: self }
    }
    #[doc = "Bit 10 - Clear Global OUT NAK"]
    #[inline(always)]
    pub fn cgoutnak(&mut self) -> CGOUTNAK_W {
        CGOUTNAK_W { w: self }
    }
    #[doc = "Bit 11 - Power-On Programming Done"]
    #[inline(always)]
    pub fn pwronprgdone(&mut self) -> PWRONPRGDONE_W {
        PWRONPRGDONE_W { w: self }
    }
    #[doc = "Bit 15 - Ignore Frame number For Isochronous End points"]
    #[inline(always)]
    pub fn ignrfrmnum(&mut self) -> IGNRFRMNUM_W {
        IGNRFRMNUM_W { w: self }
    }
    #[doc = "Bit 16 - NAK on Babble Error"]
    #[inline(always)]
    pub fn nakonbble(&mut self) -> NAKONBBLE_W {
        NAKONBBLE_W { w: self }
    }
}
