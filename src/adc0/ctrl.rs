#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x001f_0000"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x001f_0000
    }
}
#[doc = "Warm-up Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WARMUPMODE_A {
    #[doc = "0: ADC is shut down after each conversion"]
    NORMAL,
    #[doc = "1: Bandgap references do not need warm up, but have reduced accuracy."]
    FASTBG,
    #[doc = "2: Reference selected for scan mode is kept warm."]
    KEEPSCANREFWARM,
    #[doc = "3: ADC is kept warmed up and scan reference is kept warm"]
    KEEPADCWARM,
}
impl From<WARMUPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: WARMUPMODE_A) -> Self {
        match variant {
            WARMUPMODE_A::NORMAL => 0,
            WARMUPMODE_A::FASTBG => 1,
            WARMUPMODE_A::KEEPSCANREFWARM => 2,
            WARMUPMODE_A::KEEPADCWARM => 3,
        }
    }
}
#[doc = "Reader of field `WARMUPMODE`"]
pub type WARMUPMODE_R = crate::R<u8, WARMUPMODE_A>;
impl WARMUPMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WARMUPMODE_A {
        match self.bits {
            0 => WARMUPMODE_A::NORMAL,
            1 => WARMUPMODE_A::FASTBG,
            2 => WARMUPMODE_A::KEEPSCANREFWARM,
            3 => WARMUPMODE_A::KEEPADCWARM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == WARMUPMODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `FASTBG`"]
    #[inline(always)]
    pub fn is_fastbg(&self) -> bool {
        *self == WARMUPMODE_A::FASTBG
    }
    #[doc = "Checks if the value of the field is `KEEPSCANREFWARM`"]
    #[inline(always)]
    pub fn is_keepscanrefwarm(&self) -> bool {
        *self == WARMUPMODE_A::KEEPSCANREFWARM
    }
    #[doc = "Checks if the value of the field is `KEEPADCWARM`"]
    #[inline(always)]
    pub fn is_keepadcwarm(&self) -> bool {
        *self == WARMUPMODE_A::KEEPADCWARM
    }
}
#[doc = "Write proxy for field `WARMUPMODE`"]
pub struct WARMUPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WARMUPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WARMUPMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADC is shut down after each conversion"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(WARMUPMODE_A::NORMAL)
    }
    #[doc = "Bandgap references do not need warm up, but have reduced accuracy."]
    #[inline(always)]
    pub fn fastbg(self) -> &'a mut W {
        self.variant(WARMUPMODE_A::FASTBG)
    }
    #[doc = "Reference selected for scan mode is kept warm."]
    #[inline(always)]
    pub fn keepscanrefwarm(self) -> &'a mut W {
        self.variant(WARMUPMODE_A::KEEPSCANREFWARM)
    }
    #[doc = "ADC is kept warmed up and scan reference is kept warm"]
    #[inline(always)]
    pub fn keepadcwarm(self) -> &'a mut W {
        self.variant(WARMUPMODE_A::KEEPADCWARM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `TAILGATE`"]
pub type TAILGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAILGATE`"]
pub struct TAILGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAILGATE_W<'a> {
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
#[doc = "Low Pass Filter Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPFMODE_A {
    #[doc = "0: No filter or decoupling capacitor"]
    BYPASS,
    #[doc = "1: On chip decoupling capacitor selected"]
    DECAP,
    #[doc = "2: On chip RC filter selected"]
    RCFILT,
}
impl From<LPFMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LPFMODE_A) -> Self {
        match variant {
            LPFMODE_A::BYPASS => 0,
            LPFMODE_A::DECAP => 1,
            LPFMODE_A::RCFILT => 2,
        }
    }
}
#[doc = "Reader of field `LPFMODE`"]
pub type LPFMODE_R = crate::R<u8, LPFMODE_A>;
impl LPFMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LPFMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LPFMODE_A::BYPASS),
            1 => Val(LPFMODE_A::DECAP),
            2 => Val(LPFMODE_A::RCFILT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == LPFMODE_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `DECAP`"]
    #[inline(always)]
    pub fn is_decap(&self) -> bool {
        *self == LPFMODE_A::DECAP
    }
    #[doc = "Checks if the value of the field is `RCFILT`"]
    #[inline(always)]
    pub fn is_rcfilt(&self) -> bool {
        *self == LPFMODE_A::RCFILT
    }
}
#[doc = "Write proxy for field `LPFMODE`"]
pub struct LPFMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPFMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPFMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No filter or decoupling capacitor"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(LPFMODE_A::BYPASS)
    }
    #[doc = "On chip decoupling capacitor selected"]
    #[inline(always)]
    pub fn decap(self) -> &'a mut W {
        self.variant(LPFMODE_A::DECAP)
    }
    #[doc = "On chip RC filter selected"]
    #[inline(always)]
    pub fn rcfilt(self) -> &'a mut W {
        self.variant(LPFMODE_A::RCFILT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Prescaler Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESC_A {
    #[doc = "0: \"\""]
    NODIVISION,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        match variant {
            PRESC_A::NODIVISION => 0,
        }
    }
}
#[doc = "Reader of field `PRESC`"]
pub type PRESC_R = crate::R<u8, PRESC_A>;
impl PRESC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRESC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRESC_A::NODIVISION),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NODIVISION`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == PRESC_A::NODIVISION
    }
}
#[doc = "Write proxy for field `PRESC`"]
pub struct PRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "\"\""]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(PRESC_A::NODIVISION)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TIMEBASE`"]
pub type TIMEBASE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMEBASE`"]
pub struct TIMEBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEBASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Oversample Rate Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVSRSEL_A {
    #[doc = "0: 2 samples for each conversion result"]
    X2,
    #[doc = "1: 4 samples for each conversion result"]
    X4,
    #[doc = "2: 8 samples for each conversion result"]
    X8,
    #[doc = "3: 16 samples for each conversion result"]
    X16,
    #[doc = "4: 32 samples for each conversion result"]
    X32,
    #[doc = "5: 64 samples for each conversion result"]
    X64,
    #[doc = "6: 128 samples for each conversion result"]
    X128,
    #[doc = "7: 256 samples for each conversion result"]
    X256,
    #[doc = "8: 512 samples for each conversion result"]
    X512,
    #[doc = "9: 1024 samples for each conversion result"]
    X1024,
    #[doc = "10: 2048 samples for each conversion result"]
    X2048,
    #[doc = "11: 4096 samples for each conversion result"]
    X4096,
}
impl From<OVSRSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OVSRSEL_A) -> Self {
        match variant {
            OVSRSEL_A::X2 => 0,
            OVSRSEL_A::X4 => 1,
            OVSRSEL_A::X8 => 2,
            OVSRSEL_A::X16 => 3,
            OVSRSEL_A::X32 => 4,
            OVSRSEL_A::X64 => 5,
            OVSRSEL_A::X128 => 6,
            OVSRSEL_A::X256 => 7,
            OVSRSEL_A::X512 => 8,
            OVSRSEL_A::X1024 => 9,
            OVSRSEL_A::X2048 => 10,
            OVSRSEL_A::X4096 => 11,
        }
    }
}
#[doc = "Reader of field `OVSRSEL`"]
pub type OVSRSEL_R = crate::R<u8, OVSRSEL_A>;
impl OVSRSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OVSRSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OVSRSEL_A::X2),
            1 => Val(OVSRSEL_A::X4),
            2 => Val(OVSRSEL_A::X8),
            3 => Val(OVSRSEL_A::X16),
            4 => Val(OVSRSEL_A::X32),
            5 => Val(OVSRSEL_A::X64),
            6 => Val(OVSRSEL_A::X128),
            7 => Val(OVSRSEL_A::X256),
            8 => Val(OVSRSEL_A::X512),
            9 => Val(OVSRSEL_A::X1024),
            10 => Val(OVSRSEL_A::X2048),
            11 => Val(OVSRSEL_A::X4096),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `X2`"]
    #[inline(always)]
    pub fn is_x2(&self) -> bool {
        *self == OVSRSEL_A::X2
    }
    #[doc = "Checks if the value of the field is `X4`"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == OVSRSEL_A::X4
    }
    #[doc = "Checks if the value of the field is `X8`"]
    #[inline(always)]
    pub fn is_x8(&self) -> bool {
        *self == OVSRSEL_A::X8
    }
    #[doc = "Checks if the value of the field is `X16`"]
    #[inline(always)]
    pub fn is_x16(&self) -> bool {
        *self == OVSRSEL_A::X16
    }
    #[doc = "Checks if the value of the field is `X32`"]
    #[inline(always)]
    pub fn is_x32(&self) -> bool {
        *self == OVSRSEL_A::X32
    }
    #[doc = "Checks if the value of the field is `X64`"]
    #[inline(always)]
    pub fn is_x64(&self) -> bool {
        *self == OVSRSEL_A::X64
    }
    #[doc = "Checks if the value of the field is `X128`"]
    #[inline(always)]
    pub fn is_x128(&self) -> bool {
        *self == OVSRSEL_A::X128
    }
    #[doc = "Checks if the value of the field is `X256`"]
    #[inline(always)]
    pub fn is_x256(&self) -> bool {
        *self == OVSRSEL_A::X256
    }
    #[doc = "Checks if the value of the field is `X512`"]
    #[inline(always)]
    pub fn is_x512(&self) -> bool {
        *self == OVSRSEL_A::X512
    }
    #[doc = "Checks if the value of the field is `X1024`"]
    #[inline(always)]
    pub fn is_x1024(&self) -> bool {
        *self == OVSRSEL_A::X1024
    }
    #[doc = "Checks if the value of the field is `X2048`"]
    #[inline(always)]
    pub fn is_x2048(&self) -> bool {
        *self == OVSRSEL_A::X2048
    }
    #[doc = "Checks if the value of the field is `X4096`"]
    #[inline(always)]
    pub fn is_x4096(&self) -> bool {
        *self == OVSRSEL_A::X4096
    }
}
#[doc = "Write proxy for field `OVSRSEL`"]
pub struct OVSRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSRSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVSRSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "2 samples for each conversion result"]
    #[inline(always)]
    pub fn x2(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X2)
    }
    #[doc = "4 samples for each conversion result"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X4)
    }
    #[doc = "8 samples for each conversion result"]
    #[inline(always)]
    pub fn x8(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X8)
    }
    #[doc = "16 samples for each conversion result"]
    #[inline(always)]
    pub fn x16(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X16)
    }
    #[doc = "32 samples for each conversion result"]
    #[inline(always)]
    pub fn x32(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X32)
    }
    #[doc = "64 samples for each conversion result"]
    #[inline(always)]
    pub fn x64(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X64)
    }
    #[doc = "128 samples for each conversion result"]
    #[inline(always)]
    pub fn x128(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X128)
    }
    #[doc = "256 samples for each conversion result"]
    #[inline(always)]
    pub fn x256(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X256)
    }
    #[doc = "512 samples for each conversion result"]
    #[inline(always)]
    pub fn x512(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X512)
    }
    #[doc = "1024 samples for each conversion result"]
    #[inline(always)]
    pub fn x1024(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X1024)
    }
    #[doc = "2048 samples for each conversion result"]
    #[inline(always)]
    pub fn x2048(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X2048)
    }
    #[doc = "4096 samples for each conversion result"]
    #[inline(always)]
    pub fn x4096(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X4096)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `CHCONIDLE`"]
pub type CHCONIDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHCONIDLE`"]
pub struct CHCONIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHCONIDLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Warm-up Mode"]
    #[inline(always)]
    pub fn warmupmode(&self) -> WARMUPMODE_R {
        WARMUPMODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3 - Conversion Tailgating"]
    #[inline(always)]
    pub fn tailgate(&self) -> TAILGATE_R {
        TAILGATE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Low Pass Filter Mode"]
    #[inline(always)]
    pub fn lpfmode(&self) -> LPFMODE_R {
        LPFMODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:14 - Prescaler Setting"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Time Base"]
    #[inline(always)]
    pub fn timebase(&self) -> TIMEBASE_R {
        TIMEBASE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:27 - Oversample Rate Select"]
    #[inline(always)]
    pub fn ovsrsel(&self) -> OVSRSEL_R {
        OVSRSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Input channel connected when ADC is IDLE"]
    #[inline(always)]
    pub fn chconidle(&self) -> CHCONIDLE_R {
        CHCONIDLE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Warm-up Mode"]
    #[inline(always)]
    pub fn warmupmode(&mut self) -> WARMUPMODE_W {
        WARMUPMODE_W { w: self }
    }
    #[doc = "Bit 3 - Conversion Tailgating"]
    #[inline(always)]
    pub fn tailgate(&mut self) -> TAILGATE_W {
        TAILGATE_W { w: self }
    }
    #[doc = "Bits 4:5 - Low Pass Filter Mode"]
    #[inline(always)]
    pub fn lpfmode(&mut self) -> LPFMODE_W {
        LPFMODE_W { w: self }
    }
    #[doc = "Bits 8:14 - Prescaler Setting"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W { w: self }
    }
    #[doc = "Bits 16:22 - Time Base"]
    #[inline(always)]
    pub fn timebase(&mut self) -> TIMEBASE_W {
        TIMEBASE_W { w: self }
    }
    #[doc = "Bits 24:27 - Oversample Rate Select"]
    #[inline(always)]
    pub fn ovsrsel(&mut self) -> OVSRSEL_W {
        OVSRSEL_W { w: self }
    }
    #[doc = "Bit 28 - Input channel connected when ADC is IDLE"]
    #[inline(always)]
    pub fn chconidle(&mut self) -> CHCONIDLE_W {
        CHCONIDLE_W { w: self }
    }
}
