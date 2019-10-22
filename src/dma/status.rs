#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Control Current State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATE_A {
    #[doc = "0: Idle"]
    IDLE,
    #[doc = "1: Reading channel controller data"]
    RDCHCTRLDATA,
    #[doc = "2: Reading source data end pointer"]
    RDSRCENDPTR,
    #[doc = "3: Reading destination data end pointer"]
    RDDSTENDPTR,
    #[doc = "4: Reading source data"]
    RDSRCDATA,
    #[doc = "5: Writing destination data"]
    WRDSTDATA,
    #[doc = "6: Waiting for DMA request to clear"]
    WAITREQCLR,
    #[doc = "7: Writing channel controller data"]
    WRCHCTRLDATA,
    #[doc = "8: Stalled"]
    STALLED,
    #[doc = "9: Done"]
    DONE,
    #[doc = "10: Peripheral scatter-gather transition"]
    PERSCATTRANS,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        match variant {
            STATE_A::IDLE => 0,
            STATE_A::RDCHCTRLDATA => 1,
            STATE_A::RDSRCENDPTR => 2,
            STATE_A::RDDSTENDPTR => 3,
            STATE_A::RDSRCDATA => 4,
            STATE_A::WRDSTDATA => 5,
            STATE_A::WAITREQCLR => 6,
            STATE_A::WRCHCTRLDATA => 7,
            STATE_A::STALLED => 8,
            STATE_A::DONE => 9,
            STATE_A::PERSCATTRANS => 10,
        }
    }
}
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<u8, STATE_A>;
impl STATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STATE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STATE_A::IDLE),
            1 => Val(STATE_A::RDCHCTRLDATA),
            2 => Val(STATE_A::RDSRCENDPTR),
            3 => Val(STATE_A::RDDSTENDPTR),
            4 => Val(STATE_A::RDSRCDATA),
            5 => Val(STATE_A::WRDSTDATA),
            6 => Val(STATE_A::WAITREQCLR),
            7 => Val(STATE_A::WRCHCTRLDATA),
            8 => Val(STATE_A::STALLED),
            9 => Val(STATE_A::DONE),
            10 => Val(STATE_A::PERSCATTRANS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == STATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `RDCHCTRLDATA`"]
    #[inline(always)]
    pub fn is_rdchctrldata(&self) -> bool {
        *self == STATE_A::RDCHCTRLDATA
    }
    #[doc = "Checks if the value of the field is `RDSRCENDPTR`"]
    #[inline(always)]
    pub fn is_rdsrcendptr(&self) -> bool {
        *self == STATE_A::RDSRCENDPTR
    }
    #[doc = "Checks if the value of the field is `RDDSTENDPTR`"]
    #[inline(always)]
    pub fn is_rddstendptr(&self) -> bool {
        *self == STATE_A::RDDSTENDPTR
    }
    #[doc = "Checks if the value of the field is `RDSRCDATA`"]
    #[inline(always)]
    pub fn is_rdsrcdata(&self) -> bool {
        *self == STATE_A::RDSRCDATA
    }
    #[doc = "Checks if the value of the field is `WRDSTDATA`"]
    #[inline(always)]
    pub fn is_wrdstdata(&self) -> bool {
        *self == STATE_A::WRDSTDATA
    }
    #[doc = "Checks if the value of the field is `WAITREQCLR`"]
    #[inline(always)]
    pub fn is_waitreqclr(&self) -> bool {
        *self == STATE_A::WAITREQCLR
    }
    #[doc = "Checks if the value of the field is `WRCHCTRLDATA`"]
    #[inline(always)]
    pub fn is_wrchctrldata(&self) -> bool {
        *self == STATE_A::WRCHCTRLDATA
    }
    #[doc = "Checks if the value of the field is `STALLED`"]
    #[inline(always)]
    pub fn is_stalled(&self) -> bool {
        *self == STATE_A::STALLED
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == STATE_A::DONE
    }
    #[doc = "Checks if the value of the field is `PERSCATTRANS`"]
    #[inline(always)]
    pub fn is_perscattrans(&self) -> bool {
        *self == STATE_A::PERSCATTRANS
    }
}
#[doc = "Reader of field `CHNUM`"]
pub type CHNUM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - DMA Enable Status"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Control Current State"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Channel Number"]
    #[inline(always)]
    pub fn chnum(&self) -> CHNUM_R {
        CHNUM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
