#[doc = "Reader of register LFAPRESC0"]
pub type R = crate::R<u32, super::LFAPRESC0>;
#[doc = "Writer for register LFAPRESC0"]
pub type W = crate::W<u32, super::LFAPRESC0>;
#[doc = "Register LFAPRESC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LFAPRESC0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Real-Time Counter Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_A {
    #[doc = "0: LFACLKRTC = LFACLK"]
    DIV1,
    #[doc = "1: LFACLKRTC = LFACLK/2"]
    DIV2,
    #[doc = "2: LFACLKRTC = LFACLK/4"]
    DIV4,
    #[doc = "3: LFACLKRTC = LFACLK/8"]
    DIV8,
    #[doc = "4: LFACLKRTC = LFACLK/16"]
    DIV16,
    #[doc = "5: LFACLKRTC = LFACLK/32"]
    DIV32,
    #[doc = "6: LFACLKRTC = LFACLK/64"]
    DIV64,
    #[doc = "7: LFACLKRTC = LFACLK/128"]
    DIV128,
    #[doc = "8: LFACLKRTC = LFACLK/256"]
    DIV256,
    #[doc = "9: LFACLKRTC = LFACLK/512"]
    DIV512,
    #[doc = "10: LFACLKRTC = LFACLK/1024"]
    DIV1024,
    #[doc = "11: LFACLKRTC = LFACLK/2048"]
    DIV2048,
    #[doc = "12: LFACLKRTC = LFACLK/4096"]
    DIV4096,
    #[doc = "13: LFACLKRTC = LFACLK/8192"]
    DIV8192,
    #[doc = "14: LFACLKRTC = LFACLK/16384"]
    DIV16384,
    #[doc = "15: LFACLKRTC = LFACLK/32768"]
    DIV32768,
}
impl From<RTC_A> for u8 {
    #[inline(always)]
    fn from(variant: RTC_A) -> Self {
        match variant {
            RTC_A::DIV1 => 0,
            RTC_A::DIV2 => 1,
            RTC_A::DIV4 => 2,
            RTC_A::DIV8 => 3,
            RTC_A::DIV16 => 4,
            RTC_A::DIV32 => 5,
            RTC_A::DIV64 => 6,
            RTC_A::DIV128 => 7,
            RTC_A::DIV256 => 8,
            RTC_A::DIV512 => 9,
            RTC_A::DIV1024 => 10,
            RTC_A::DIV2048 => 11,
            RTC_A::DIV4096 => 12,
            RTC_A::DIV8192 => 13,
            RTC_A::DIV16384 => 14,
            RTC_A::DIV32768 => 15,
        }
    }
}
#[doc = "Reader of field `RTC`"]
pub type RTC_R = crate::R<u8, RTC_A>;
impl RTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_A {
        match self.bits {
            0 => RTC_A::DIV1,
            1 => RTC_A::DIV2,
            2 => RTC_A::DIV4,
            3 => RTC_A::DIV8,
            4 => RTC_A::DIV16,
            5 => RTC_A::DIV32,
            6 => RTC_A::DIV64,
            7 => RTC_A::DIV128,
            8 => RTC_A::DIV256,
            9 => RTC_A::DIV512,
            10 => RTC_A::DIV1024,
            11 => RTC_A::DIV2048,
            12 => RTC_A::DIV4096,
            13 => RTC_A::DIV8192,
            14 => RTC_A::DIV16384,
            15 => RTC_A::DIV32768,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == RTC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == RTC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == RTC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == RTC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == RTC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == RTC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == RTC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == RTC_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == RTC_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == RTC_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == RTC_A::DIV1024
    }
    #[doc = "Checks if the value of the field is `DIV2048`"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == RTC_A::DIV2048
    }
    #[doc = "Checks if the value of the field is `DIV4096`"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == RTC_A::DIV4096
    }
    #[doc = "Checks if the value of the field is `DIV8192`"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == RTC_A::DIV8192
    }
    #[doc = "Checks if the value of the field is `DIV16384`"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == RTC_A::DIV16384
    }
    #[doc = "Checks if the value of the field is `DIV32768`"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == RTC_A::DIV32768
    }
}
#[doc = "Write proxy for field `RTC`"]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LFACLKRTC = LFACLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(RTC_A::DIV1)
    }
    #[doc = "LFACLKRTC = LFACLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(RTC_A::DIV2)
    }
    #[doc = "LFACLKRTC = LFACLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(RTC_A::DIV4)
    }
    #[doc = "LFACLKRTC = LFACLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(RTC_A::DIV8)
    }
    #[doc = "LFACLKRTC = LFACLK/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(RTC_A::DIV16)
    }
    #[doc = "LFACLKRTC = LFACLK/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(RTC_A::DIV32)
    }
    #[doc = "LFACLKRTC = LFACLK/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(RTC_A::DIV64)
    }
    #[doc = "LFACLKRTC = LFACLK/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(RTC_A::DIV128)
    }
    #[doc = "LFACLKRTC = LFACLK/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(RTC_A::DIV256)
    }
    #[doc = "LFACLKRTC = LFACLK/512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(RTC_A::DIV512)
    }
    #[doc = "LFACLKRTC = LFACLK/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(RTC_A::DIV1024)
    }
    #[doc = "LFACLKRTC = LFACLK/2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut W {
        self.variant(RTC_A::DIV2048)
    }
    #[doc = "LFACLKRTC = LFACLK/4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut W {
        self.variant(RTC_A::DIV4096)
    }
    #[doc = "LFACLKRTC = LFACLK/8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut W {
        self.variant(RTC_A::DIV8192)
    }
    #[doc = "LFACLKRTC = LFACLK/16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut W {
        self.variant(RTC_A::DIV16384)
    }
    #[doc = "LFACLKRTC = LFACLK/32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut W {
        self.variant(RTC_A::DIV32768)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Real-Time Counter Prescaler"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Real-Time Counter Prescaler"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
}
