#[doc = "Reader of register DOEP0_TSIZ"]
pub type R = crate::R<u32, super::DOEP0_TSIZ>;
#[doc = "Writer for register DOEP0_TSIZ"]
pub type W = crate::W<u32, super::DOEP0_TSIZ>;
#[doc = "Register DOEP0_TSIZ `reset()`'s with value 0"]
impl crate::ResetValue for super::DOEP0_TSIZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XFERSIZE`"]
pub type XFERSIZE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `XFERSIZE`"]
pub struct XFERSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> XFERSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0007_ffff) | ((value as u32) & 0x0007_ffff);
        self.w
    }
}
#[doc = "Reader of field `PKTCNT`"]
pub type PKTCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PKTCNT`"]
pub struct PKTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PKTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 19)) | (((value as u32) & 0x03ff) << 19);
        self.w
    }
}
#[doc = "Receive Data PID / SETUP Packet Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDPIDSUPCNT_A {
    #[doc = "0: DATA0 PID."]
    DATA0,
    #[doc = "1: DATA2 PID / 1 Packet."]
    DATA2,
    #[doc = "2: DATA1 PID / 2 Packets."]
    DATA1,
    #[doc = "3: MDATA PID / 3 Packets."]
    MDATA,
}
impl From<RXDPIDSUPCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: RXDPIDSUPCNT_A) -> Self {
        match variant {
            RXDPIDSUPCNT_A::DATA0 => 0,
            RXDPIDSUPCNT_A::DATA2 => 1,
            RXDPIDSUPCNT_A::DATA1 => 2,
            RXDPIDSUPCNT_A::MDATA => 3,
        }
    }
}
#[doc = "Reader of field `RXDPIDSUPCNT`"]
pub type RXDPIDSUPCNT_R = crate::R<u8, RXDPIDSUPCNT_A>;
impl RXDPIDSUPCNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDPIDSUPCNT_A {
        match self.bits {
            0 => RXDPIDSUPCNT_A::DATA0,
            1 => RXDPIDSUPCNT_A::DATA2,
            2 => RXDPIDSUPCNT_A::DATA1,
            3 => RXDPIDSUPCNT_A::MDATA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == RXDPIDSUPCNT_A::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA2`"]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == RXDPIDSUPCNT_A::DATA2
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == RXDPIDSUPCNT_A::DATA1
    }
    #[doc = "Checks if the value of the field is `MDATA`"]
    #[inline(always)]
    pub fn is_mdata(&self) -> bool {
        *self == RXDPIDSUPCNT_A::MDATA
    }
}
impl R {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&self) -> XFERSIZE_R {
        XFERSIZE_R::new((self.bits & 0x0007_ffff) as u32)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - Receive Data PID / SETUP Packet Count"]
    #[inline(always)]
    pub fn rxdpidsupcnt(&self) -> RXDPIDSUPCNT_R {
        RXDPIDSUPCNT_R::new(((self.bits >> 29) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&mut self) -> XFERSIZE_W {
        XFERSIZE_W { w: self }
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W {
        PKTCNT_W { w: self }
    }
}
