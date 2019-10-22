#[doc = "Reader of register GRXSTSR"]
pub type R = crate::R<u32, super::GRXSTSR>;
#[doc = "Reader of field `CHEPNUM`"]
pub type CHEPNUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `BCNT`"]
pub type BCNT_R = crate::R<u16, u16>;
#[doc = "Data PID (host or device)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPID_A {
    #[doc = "0: DATA0 PID."]
    DATA0,
    #[doc = "1: DATA1 PID."]
    DATA1,
    #[doc = "2: DATA2 PID."]
    DATA2,
    #[doc = "3: MDATA PID."]
    MDATA,
}
impl From<DPID_A> for u8 {
    #[inline(always)]
    fn from(variant: DPID_A) -> Self {
        match variant {
            DPID_A::DATA0 => 0,
            DPID_A::DATA1 => 1,
            DPID_A::DATA2 => 2,
            DPID_A::MDATA => 3,
        }
    }
}
#[doc = "Reader of field `DPID`"]
pub type DPID_R = crate::R<u8, DPID_A>;
impl DPID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPID_A {
        match self.bits {
            0 => DPID_A::DATA0,
            1 => DPID_A::DATA1,
            2 => DPID_A::DATA2,
            3 => DPID_A::MDATA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == DPID_A::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == DPID_A::DATA1
    }
    #[doc = "Checks if the value of the field is `DATA2`"]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == DPID_A::DATA2
    }
    #[doc = "Checks if the value of the field is `MDATA`"]
    #[inline(always)]
    pub fn is_mdata(&self) -> bool {
        *self == DPID_A::MDATA
    }
}
#[doc = "Packet Status (host or device)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PKTSTS_A {
    #[doc = "1: Device mode: Global OUT NAK (triggers an interrupt)."]
    GOUTNAK,
    #[doc = "2: Host mode: IN data packet received. Device mode: OUT data packet received."]
    PKTRCV,
    #[doc = "3: Host mode: IN transfer completed (triggers an interrupt). Device mode: OUT transfer completed (triggers an interrupt)."]
    XFERCOMPL,
    #[doc = "4: Device mode: SETUP transaction completed (triggers an interrupt)."]
    SETUPCOMPL,
    #[doc = "5: Host mode: Data toggle error (triggers an interrupt)."]
    TGLERR,
    #[doc = "6: Device mode: SETUP data packet received."]
    SETUPRCV,
    #[doc = "7: Host mode: Channel halted (triggers an interrupt)."]
    CHLT,
}
impl From<PKTSTS_A> for u8 {
    #[inline(always)]
    fn from(variant: PKTSTS_A) -> Self {
        match variant {
            PKTSTS_A::GOUTNAK => 1,
            PKTSTS_A::PKTRCV => 2,
            PKTSTS_A::XFERCOMPL => 3,
            PKTSTS_A::SETUPCOMPL => 4,
            PKTSTS_A::TGLERR => 5,
            PKTSTS_A::SETUPRCV => 6,
            PKTSTS_A::CHLT => 7,
        }
    }
}
#[doc = "Reader of field `PKTSTS`"]
pub type PKTSTS_R = crate::R<u8, PKTSTS_A>;
impl PKTSTS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PKTSTS_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(PKTSTS_A::GOUTNAK),
            2 => Val(PKTSTS_A::PKTRCV),
            3 => Val(PKTSTS_A::XFERCOMPL),
            4 => Val(PKTSTS_A::SETUPCOMPL),
            5 => Val(PKTSTS_A::TGLERR),
            6 => Val(PKTSTS_A::SETUPRCV),
            7 => Val(PKTSTS_A::CHLT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GOUTNAK`"]
    #[inline(always)]
    pub fn is_goutnak(&self) -> bool {
        *self == PKTSTS_A::GOUTNAK
    }
    #[doc = "Checks if the value of the field is `PKTRCV`"]
    #[inline(always)]
    pub fn is_pktrcv(&self) -> bool {
        *self == PKTSTS_A::PKTRCV
    }
    #[doc = "Checks if the value of the field is `XFERCOMPL`"]
    #[inline(always)]
    pub fn is_xfercompl(&self) -> bool {
        *self == PKTSTS_A::XFERCOMPL
    }
    #[doc = "Checks if the value of the field is `SETUPCOMPL`"]
    #[inline(always)]
    pub fn is_setupcompl(&self) -> bool {
        *self == PKTSTS_A::SETUPCOMPL
    }
    #[doc = "Checks if the value of the field is `TGLERR`"]
    #[inline(always)]
    pub fn is_tglerr(&self) -> bool {
        *self == PKTSTS_A::TGLERR
    }
    #[doc = "Checks if the value of the field is `SETUPRCV`"]
    #[inline(always)]
    pub fn is_setuprcv(&self) -> bool {
        *self == PKTSTS_A::SETUPRCV
    }
    #[doc = "Checks if the value of the field is `CHLT`"]
    #[inline(always)]
    pub fn is_chlt(&self) -> bool {
        *self == PKTSTS_A::CHLT
    }
}
#[doc = "Reader of field `FN`"]
pub type FN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Channel Number host only / Endpoint Number"]
    #[inline(always)]
    pub fn chepnum(&self) -> CHEPNUM_R {
        CHEPNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14 - Byte Count (host or device)"]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16 - Data PID (host or device)"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bits 17:20 - Packet Status (host or device)"]
    #[inline(always)]
    pub fn pktsts(&self) -> PKTSTS_R {
        PKTSTS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:24 - Frame Number"]
    #[inline(always)]
    pub fn fn_(&self) -> FN_R {
        FN_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
}
