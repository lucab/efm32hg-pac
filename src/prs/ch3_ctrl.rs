#[doc = "Reader of register CH3_CTRL"]
pub type R = crate::R<u32, super::CH3_CTRL>;
#[doc = "Writer for register CH3_CTRL"]
pub type W = crate::W<u32, super::CH3_CTRL>;
#[doc = "Register CH3_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CH3_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SIGSEL`"]
pub type SIGSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIGSEL`"]
pub struct SIGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOURCESEL_A {
    #[doc = "0: No source selected"]
    NONE,
    #[doc = "1: Voltage Comparator"]
    VCMP,
    #[doc = "2: Analog Comparator 0"]
    ACMP0,
    #[doc = "8: Analog to Digital Converter 0"]
    ADC0,
    #[doc = "16: Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    USART0,
    #[doc = "17: Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    USART1,
    #[doc = "28: Timer 0"]
    TIMER0,
    #[doc = "29: Timer 1"]
    TIMER1,
    #[doc = "30: Timer 2"]
    TIMER2,
    #[doc = "36: Universal Serial Bus Interface"]
    USB,
    #[doc = "40: Real-Time Counter"]
    RTC,
    #[doc = "48: General purpose Input/Output"]
    GPIOL,
    #[doc = "49: General purpose Input/Output"]
    GPIOH,
    #[doc = "54: Pulse Counter 0"]
    PCNT0,
}
impl From<SOURCESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SOURCESEL_A) -> Self {
        match variant {
            SOURCESEL_A::NONE => 0,
            SOURCESEL_A::VCMP => 1,
            SOURCESEL_A::ACMP0 => 2,
            SOURCESEL_A::ADC0 => 8,
            SOURCESEL_A::USART0 => 16,
            SOURCESEL_A::USART1 => 17,
            SOURCESEL_A::TIMER0 => 28,
            SOURCESEL_A::TIMER1 => 29,
            SOURCESEL_A::TIMER2 => 30,
            SOURCESEL_A::USB => 36,
            SOURCESEL_A::RTC => 40,
            SOURCESEL_A::GPIOL => 48,
            SOURCESEL_A::GPIOH => 49,
            SOURCESEL_A::PCNT0 => 54,
        }
    }
}
#[doc = "Reader of field `SOURCESEL`"]
pub type SOURCESEL_R = crate::R<u8, SOURCESEL_A>;
impl SOURCESEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SOURCESEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SOURCESEL_A::NONE),
            1 => Val(SOURCESEL_A::VCMP),
            2 => Val(SOURCESEL_A::ACMP0),
            8 => Val(SOURCESEL_A::ADC0),
            16 => Val(SOURCESEL_A::USART0),
            17 => Val(SOURCESEL_A::USART1),
            28 => Val(SOURCESEL_A::TIMER0),
            29 => Val(SOURCESEL_A::TIMER1),
            30 => Val(SOURCESEL_A::TIMER2),
            36 => Val(SOURCESEL_A::USB),
            40 => Val(SOURCESEL_A::RTC),
            48 => Val(SOURCESEL_A::GPIOL),
            49 => Val(SOURCESEL_A::GPIOH),
            54 => Val(SOURCESEL_A::PCNT0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SOURCESEL_A::NONE
    }
    #[doc = "Checks if the value of the field is `VCMP`"]
    #[inline(always)]
    pub fn is_vcmp(&self) -> bool {
        *self == SOURCESEL_A::VCMP
    }
    #[doc = "Checks if the value of the field is `ACMP0`"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == SOURCESEL_A::ACMP0
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == SOURCESEL_A::ADC0
    }
    #[doc = "Checks if the value of the field is `USART0`"]
    #[inline(always)]
    pub fn is_usart0(&self) -> bool {
        *self == SOURCESEL_A::USART0
    }
    #[doc = "Checks if the value of the field is `USART1`"]
    #[inline(always)]
    pub fn is_usart1(&self) -> bool {
        *self == SOURCESEL_A::USART1
    }
    #[doc = "Checks if the value of the field is `TIMER0`"]
    #[inline(always)]
    pub fn is_timer0(&self) -> bool {
        *self == SOURCESEL_A::TIMER0
    }
    #[doc = "Checks if the value of the field is `TIMER1`"]
    #[inline(always)]
    pub fn is_timer1(&self) -> bool {
        *self == SOURCESEL_A::TIMER1
    }
    #[doc = "Checks if the value of the field is `TIMER2`"]
    #[inline(always)]
    pub fn is_timer2(&self) -> bool {
        *self == SOURCESEL_A::TIMER2
    }
    #[doc = "Checks if the value of the field is `USB`"]
    #[inline(always)]
    pub fn is_usb(&self) -> bool {
        *self == SOURCESEL_A::USB
    }
    #[doc = "Checks if the value of the field is `RTC`"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == SOURCESEL_A::RTC
    }
    #[doc = "Checks if the value of the field is `GPIOL`"]
    #[inline(always)]
    pub fn is_gpiol(&self) -> bool {
        *self == SOURCESEL_A::GPIOL
    }
    #[doc = "Checks if the value of the field is `GPIOH`"]
    #[inline(always)]
    pub fn is_gpioh(&self) -> bool {
        *self == SOURCESEL_A::GPIOH
    }
    #[doc = "Checks if the value of the field is `PCNT0`"]
    #[inline(always)]
    pub fn is_pcnt0(&self) -> bool {
        *self == SOURCESEL_A::PCNT0
    }
}
#[doc = "Write proxy for field `SOURCESEL`"]
pub struct SOURCESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SOURCESEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOURCESEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No source selected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SOURCESEL_A::NONE)
    }
    #[doc = "Voltage Comparator"]
    #[inline(always)]
    pub fn vcmp(self) -> &'a mut W {
        self.variant(SOURCESEL_A::VCMP)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::ACMP0)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::ADC0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn usart0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USART0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn usart1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USART1)
    }
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn timer0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::TIMER0)
    }
    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn timer1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::TIMER1)
    }
    #[doc = "Timer 2"]
    #[inline(always)]
    pub fn timer2(self) -> &'a mut W {
        self.variant(SOURCESEL_A::TIMER2)
    }
    #[doc = "Universal Serial Bus Interface"]
    #[inline(always)]
    pub fn usb(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USB)
    }
    #[doc = "Real-Time Counter"]
    #[inline(always)]
    pub fn rtc(self) -> &'a mut W {
        self.variant(SOURCESEL_A::RTC)
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn gpiol(self) -> &'a mut W {
        self.variant(SOURCESEL_A::GPIOL)
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn gpioh(self) -> &'a mut W {
        self.variant(SOURCESEL_A::GPIOH)
    }
    #[doc = "Pulse Counter 0"]
    #[inline(always)]
    pub fn pcnt0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::PCNT0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Edge Detect Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDSEL_A {
    #[doc = "0: Signal is left as it is"]
    OFF,
    #[doc = "1: A one HFPERCLK cycle pulse is generated for every positive edge of the incoming signal"]
    POSEDGE,
    #[doc = "2: A one HFPERCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    NEGEDGE,
    #[doc = "3: A one HFPERCLK clock cycle pulse is generated for every edge of the incoming signal"]
    BOTHEDGES,
}
impl From<EDSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EDSEL_A) -> Self {
        match variant {
            EDSEL_A::OFF => 0,
            EDSEL_A::POSEDGE => 1,
            EDSEL_A::NEGEDGE => 2,
            EDSEL_A::BOTHEDGES => 3,
        }
    }
}
#[doc = "Reader of field `EDSEL`"]
pub type EDSEL_R = crate::R<u8, EDSEL_A>;
impl EDSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDSEL_A {
        match self.bits {
            0 => EDSEL_A::OFF,
            1 => EDSEL_A::POSEDGE,
            2 => EDSEL_A::NEGEDGE,
            3 => EDSEL_A::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == EDSEL_A::OFF
    }
    #[doc = "Checks if the value of the field is `POSEDGE`"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        *self == EDSEL_A::POSEDGE
    }
    #[doc = "Checks if the value of the field is `NEGEDGE`"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == EDSEL_A::NEGEDGE
    }
    #[doc = "Checks if the value of the field is `BOTHEDGES`"]
    #[inline(always)]
    pub fn is_bothedges(&self) -> bool {
        *self == EDSEL_A::BOTHEDGES
    }
}
#[doc = "Write proxy for field `EDSEL`"]
pub struct EDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Signal is left as it is"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(EDSEL_A::OFF)
    }
    #[doc = "A one HFPERCLK cycle pulse is generated for every positive edge of the incoming signal"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut W {
        self.variant(EDSEL_A::POSEDGE)
    }
    #[doc = "A one HFPERCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut W {
        self.variant(EDSEL_A::NEGEDGE)
    }
    #[doc = "A one HFPERCLK clock cycle pulse is generated for every edge of the incoming signal"]
    #[inline(always)]
    pub fn bothedges(self) -> &'a mut W {
        self.variant(EDSEL_A::BOTHEDGES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `ASYNC_`"]
pub type ASYNC__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASYNC_`"]
pub struct ASYNC__W<'a> {
    w: &'a mut W,
}
impl<'a> ASYNC__W<'a> {
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
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SIGSEL_R {
        SIGSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&self) -> SOURCESEL_R {
        SOURCESEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - Edge Detect Select"]
    #[inline(always)]
    pub fn edsel(&self) -> EDSEL_R {
        EDSEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 28 - Asynchronous reflex"]
    #[inline(always)]
    pub fn async_(&self) -> ASYNC__R {
        ASYNC__R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&mut self) -> SIGSEL_W {
        SIGSEL_W { w: self }
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&mut self) -> SOURCESEL_W {
        SOURCESEL_W { w: self }
    }
    #[doc = "Bits 24:25 - Edge Detect Select"]
    #[inline(always)]
    pub fn edsel(&mut self) -> EDSEL_W {
        EDSEL_W { w: self }
    }
    #[doc = "Bit 28 - Asynchronous reflex"]
    #[inline(always)]
    pub fn async_(&mut self) -> ASYNC__W {
        ASYNC__W { w: self }
    }
}
