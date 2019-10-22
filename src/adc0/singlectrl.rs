#[doc = "Reader of register SINGLECTRL"]
pub type R = crate::R<u32, super::SINGLECTRL>;
#[doc = "Writer for register SINGLECTRL"]
pub type W = crate::W<u32, super::SINGLECTRL>;
#[doc = "Register SINGLECTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SINGLECTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REP`"]
pub type REP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REP`"]
pub struct REP_W<'a> {
    w: &'a mut W,
}
impl<'a> REP_W<'a> {
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
#[doc = "Reader of field `DIFF`"]
pub type DIFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIFF`"]
pub struct DIFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF_W<'a> {
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
#[doc = "Reader of field `ADJ`"]
pub type ADJ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADJ`"]
pub struct ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADJ_W<'a> {
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
#[doc = "Single Sample Resolution Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RES_A {
    #[doc = "0: 12-bit resolution"]
    _12BIT,
    #[doc = "1: 8-bit resolution"]
    _8BIT,
    #[doc = "2: 6-bit resolution"]
    _6BIT,
    #[doc = "3: Oversampling enabled. Oversampling rate is set in OVSRSEL"]
    OVS,
}
impl From<RES_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_A) -> Self {
        match variant {
            RES_A::_12BIT => 0,
            RES_A::_8BIT => 1,
            RES_A::_6BIT => 2,
            RES_A::OVS => 3,
        }
    }
}
#[doc = "Reader of field `RES`"]
pub type RES_R = crate::R<u8, RES_A>;
impl RES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RES_A {
        match self.bits {
            0 => RES_A::_12BIT,
            1 => RES_A::_8BIT,
            2 => RES_A::_6BIT,
            3 => RES_A::OVS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_12BIT`"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == RES_A::_12BIT
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == RES_A::_8BIT
    }
    #[doc = "Checks if the value of the field is `_6BIT`"]
    #[inline(always)]
    pub fn is_6bit(&self) -> bool {
        *self == RES_A::_6BIT
    }
    #[doc = "Checks if the value of the field is `OVS`"]
    #[inline(always)]
    pub fn is_ovs(&self) -> bool {
        *self == RES_A::OVS
    }
}
#[doc = "Write proxy for field `RES`"]
pub struct RES_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "12-bit resolution"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut W {
        self.variant(RES_A::_12BIT)
    }
    #[doc = "8-bit resolution"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(RES_A::_8BIT)
    }
    #[doc = "6-bit resolution"]
    #[inline(always)]
    pub fn _6bit(self) -> &'a mut W {
        self.variant(RES_A::_6BIT)
    }
    #[doc = "Oversampling enabled. Oversampling rate is set in OVSRSEL"]
    #[inline(always)]
    pub fn ovs(self) -> &'a mut W {
        self.variant(RES_A::OVS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `INPUTSEL`"]
pub type INPUTSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INPUTSEL`"]
pub struct INPUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Single Sample Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REF_A {
    #[doc = "0: Internal 1.25 V reference"]
    _1V25,
    #[doc = "1: Internal 2.5 V reference"]
    _2V5,
    #[doc = "2: Buffered VDD"]
    VDD,
    #[doc = "3: Internal differential 5 V reference"]
    _5VDIFF,
    #[doc = "4: Single ended external reference from ADCn_CH6"]
    EXTSINGLE,
    #[doc = "5: Differential external reference, 2x(ADCn_CH6 - ADCn_CH7)"]
    _2XEXTDIFF,
    #[doc = "6: Unbuffered 2xVDD"]
    _2XVDD,
}
impl From<REF_A> for u8 {
    #[inline(always)]
    fn from(variant: REF_A) -> Self {
        match variant {
            REF_A::_1V25 => 0,
            REF_A::_2V5 => 1,
            REF_A::VDD => 2,
            REF_A::_5VDIFF => 3,
            REF_A::EXTSINGLE => 4,
            REF_A::_2XEXTDIFF => 5,
            REF_A::_2XVDD => 6,
        }
    }
}
#[doc = "Reader of field `REF`"]
pub type REF_R = crate::R<u8, REF_A>;
impl REF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REF_A::_1V25),
            1 => Val(REF_A::_2V5),
            2 => Val(REF_A::VDD),
            3 => Val(REF_A::_5VDIFF),
            4 => Val(REF_A::EXTSINGLE),
            5 => Val(REF_A::_2XEXTDIFF),
            6 => Val(REF_A::_2XVDD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1V25`"]
    #[inline(always)]
    pub fn is_1v25(&self) -> bool {
        *self == REF_A::_1V25
    }
    #[doc = "Checks if the value of the field is `_2V5`"]
    #[inline(always)]
    pub fn is_2v5(&self) -> bool {
        *self == REF_A::_2V5
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == REF_A::VDD
    }
    #[doc = "Checks if the value of the field is `_5VDIFF`"]
    #[inline(always)]
    pub fn is_5vdiff(&self) -> bool {
        *self == REF_A::_5VDIFF
    }
    #[doc = "Checks if the value of the field is `EXTSINGLE`"]
    #[inline(always)]
    pub fn is_extsingle(&self) -> bool {
        *self == REF_A::EXTSINGLE
    }
    #[doc = "Checks if the value of the field is `_2XEXTDIFF`"]
    #[inline(always)]
    pub fn is_2xextdiff(&self) -> bool {
        *self == REF_A::_2XEXTDIFF
    }
    #[doc = "Checks if the value of the field is `_2XVDD`"]
    #[inline(always)]
    pub fn is_2xvdd(&self) -> bool {
        *self == REF_A::_2XVDD
    }
}
#[doc = "Write proxy for field `REF`"]
pub struct REF_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Internal 1.25 V reference"]
    #[inline(always)]
    pub fn _1v25(self) -> &'a mut W {
        self.variant(REF_A::_1V25)
    }
    #[doc = "Internal 2.5 V reference"]
    #[inline(always)]
    pub fn _2v5(self) -> &'a mut W {
        self.variant(REF_A::_2V5)
    }
    #[doc = "Buffered VDD"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut W {
        self.variant(REF_A::VDD)
    }
    #[doc = "Internal differential 5 V reference"]
    #[inline(always)]
    pub fn _5vdiff(self) -> &'a mut W {
        self.variant(REF_A::_5VDIFF)
    }
    #[doc = "Single ended external reference from ADCn_CH6"]
    #[inline(always)]
    pub fn extsingle(self) -> &'a mut W {
        self.variant(REF_A::EXTSINGLE)
    }
    #[doc = "Differential external reference, 2x(ADCn_CH6 - ADCn_CH7)"]
    #[inline(always)]
    pub fn _2xextdiff(self) -> &'a mut W {
        self.variant(REF_A::_2XEXTDIFF)
    }
    #[doc = "Unbuffered 2xVDD"]
    #[inline(always)]
    pub fn _2xvdd(self) -> &'a mut W {
        self.variant(REF_A::_2XVDD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Single Sample Acquisition Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AT_A {
    #[doc = "0: 1 ADC_CLK cycle acquisition time for single sample"]
    _1CYCLE,
    #[doc = "1: 2 ADC_CLK cycles acquisition time for single sample"]
    _2CYCLES,
    #[doc = "2: 4 ADC_CLK cycles acquisition time for single sample"]
    _4CYCLES,
    #[doc = "3: 8 ADC_CLK cycles acquisition time for single sample"]
    _8CYCLES,
    #[doc = "4: 16 ADC_CLK cycles acquisition time for single sample"]
    _16CYCLES,
    #[doc = "5: 32 ADC_CLK cycles acquisition time for single sample"]
    _32CYCLES,
    #[doc = "6: 64 ADC_CLK cycles acquisition time for single sample"]
    _64CYCLES,
    #[doc = "7: 128 ADC_CLK cycles acquisition time for single sample"]
    _128CYCLES,
    #[doc = "8: 256 ADC_CLK cycles acquisition time for single sample"]
    _256CYCLES,
}
impl From<AT_A> for u8 {
    #[inline(always)]
    fn from(variant: AT_A) -> Self {
        match variant {
            AT_A::_1CYCLE => 0,
            AT_A::_2CYCLES => 1,
            AT_A::_4CYCLES => 2,
            AT_A::_8CYCLES => 3,
            AT_A::_16CYCLES => 4,
            AT_A::_32CYCLES => 5,
            AT_A::_64CYCLES => 6,
            AT_A::_128CYCLES => 7,
            AT_A::_256CYCLES => 8,
        }
    }
}
#[doc = "Reader of field `AT`"]
pub type AT_R = crate::R<u8, AT_A>;
impl AT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AT_A::_1CYCLE),
            1 => Val(AT_A::_2CYCLES),
            2 => Val(AT_A::_4CYCLES),
            3 => Val(AT_A::_8CYCLES),
            4 => Val(AT_A::_16CYCLES),
            5 => Val(AT_A::_32CYCLES),
            6 => Val(AT_A::_64CYCLES),
            7 => Val(AT_A::_128CYCLES),
            8 => Val(AT_A::_256CYCLES),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1CYCLE`"]
    #[inline(always)]
    pub fn is_1cycle(&self) -> bool {
        *self == AT_A::_1CYCLE
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == AT_A::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == AT_A::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_8CYCLES`"]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == AT_A::_8CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == AT_A::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == AT_A::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == AT_A::_64CYCLES
    }
    #[doc = "Checks if the value of the field is `_128CYCLES`"]
    #[inline(always)]
    pub fn is_128cycles(&self) -> bool {
        *self == AT_A::_128CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == AT_A::_256CYCLES
    }
}
#[doc = "Write proxy for field `AT`"]
pub struct AT_W<'a> {
    w: &'a mut W,
}
impl<'a> AT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 ADC_CLK cycle acquisition time for single sample"]
    #[inline(always)]
    pub fn _1cycle(self) -> &'a mut W {
        self.variant(AT_A::_1CYCLE)
    }
    #[doc = "2 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(AT_A::_2CYCLES)
    }
    #[doc = "4 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(AT_A::_4CYCLES)
    }
    #[doc = "8 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut W {
        self.variant(AT_A::_8CYCLES)
    }
    #[doc = "16 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(AT_A::_16CYCLES)
    }
    #[doc = "32 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(AT_A::_32CYCLES)
    }
    #[doc = "64 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(AT_A::_64CYCLES)
    }
    #[doc = "128 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut W {
        self.variant(AT_A::_128CYCLES)
    }
    #[doc = "256 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(AT_A::_256CYCLES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `PRSEN`"]
pub type PRSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRSEN`"]
pub struct PRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Single Sample PRS Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSSEL_A {
    #[doc = "0: PRS ch 0 triggers single sample"]
    PRSCH0,
    #[doc = "1: PRS ch 1 triggers single sample"]
    PRSCH1,
    #[doc = "2: PRS ch 2 triggers single sample"]
    PRSCH2,
    #[doc = "3: PRS ch 3 triggers single sample"]
    PRSCH3,
    #[doc = "4: PRS ch 4 triggers single sample"]
    PRSCH4,
    #[doc = "5: PRS ch 5 triggers single sample"]
    PRSCH5,
}
impl From<PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL_A) -> Self {
        match variant {
            PRSSEL_A::PRSCH0 => 0,
            PRSSEL_A::PRSCH1 => 1,
            PRSSEL_A::PRSCH2 => 2,
            PRSSEL_A::PRSCH3 => 3,
            PRSSEL_A::PRSCH4 => 4,
            PRSSEL_A::PRSCH5 => 5,
        }
    }
}
#[doc = "Reader of field `PRSSEL`"]
pub type PRSSEL_R = crate::R<u8, PRSSEL_A>;
impl PRSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRSSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRSSEL_A::PRSCH0),
            1 => Val(PRSSEL_A::PRSCH1),
            2 => Val(PRSSEL_A::PRSCH2),
            3 => Val(PRSSEL_A::PRSCH3),
            4 => Val(PRSSEL_A::PRSCH4),
            5 => Val(PRSSEL_A::PRSCH5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL_A::PRSCH5
    }
}
#[doc = "Write proxy for field `PRSSEL`"]
pub struct PRSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRSSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PRS ch 0 triggers single sample"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS ch 1 triggers single sample"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS ch 2 triggers single sample"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS ch 3 triggers single sample"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS ch 4 triggers single sample"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS ch 5 triggers single sample"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Single Sample Repetitive Mode"]
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Single Sample Differential Mode"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Single Sample Result Adjustment"]
    #[inline(always)]
    pub fn adj(&self) -> ADJ_R {
        ADJ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Single Sample Resolution Select"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Single Sample Input Selection"]
    #[inline(always)]
    pub fn inputsel(&self) -> INPUTSEL_R {
        INPUTSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - Single Sample Reference Selection"]
    #[inline(always)]
    pub fn ref_(&self) -> REF_R {
        REF_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:23 - Single Sample Acquisition Time"]
    #[inline(always)]
    pub fn at(&self) -> AT_R {
        AT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Single Sample PRS Trigger Enable"]
    #[inline(always)]
    pub fn prsen(&self) -> PRSEN_R {
        PRSEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - Single Sample PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Single Sample Repetitive Mode"]
    #[inline(always)]
    pub fn rep(&mut self) -> REP_W {
        REP_W { w: self }
    }
    #[doc = "Bit 1 - Single Sample Differential Mode"]
    #[inline(always)]
    pub fn diff(&mut self) -> DIFF_W {
        DIFF_W { w: self }
    }
    #[doc = "Bit 2 - Single Sample Result Adjustment"]
    #[inline(always)]
    pub fn adj(&mut self) -> ADJ_W {
        ADJ_W { w: self }
    }
    #[doc = "Bits 4:5 - Single Sample Resolution Select"]
    #[inline(always)]
    pub fn res(&mut self) -> RES_W {
        RES_W { w: self }
    }
    #[doc = "Bits 8:11 - Single Sample Input Selection"]
    #[inline(always)]
    pub fn inputsel(&mut self) -> INPUTSEL_W {
        INPUTSEL_W { w: self }
    }
    #[doc = "Bits 16:18 - Single Sample Reference Selection"]
    #[inline(always)]
    pub fn ref_(&mut self) -> REF_W {
        REF_W { w: self }
    }
    #[doc = "Bits 20:23 - Single Sample Acquisition Time"]
    #[inline(always)]
    pub fn at(&mut self) -> AT_W {
        AT_W { w: self }
    }
    #[doc = "Bit 24 - Single Sample PRS Trigger Enable"]
    #[inline(always)]
    pub fn prsen(&mut self) -> PRSEN_W {
        PRSEN_W { w: self }
    }
    #[doc = "Bits 28:30 - Single Sample PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PRSSEL_W {
        PRSSEL_W { w: self }
    }
}
