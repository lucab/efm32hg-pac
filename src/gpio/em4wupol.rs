#[doc = "Reader of register EM4WUPOL"]
pub type R = crate::R<u32, super::EM4WUPOL>;
#[doc = "Writer for register EM4WUPOL"]
pub type W = crate::W<u32, super::EM4WUPOL>;
#[doc = "Register EM4WUPOL `reset()`'s with value 0"]
impl crate::ResetValue for super::EM4WUPOL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "EM4 Wake-up Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM4WUPOL_A {
    #[doc = "1: Determines polarity on pin A0"]
    A0,
    #[doc = "4: Determines polarity on pin C9"]
    C9,
    #[doc = "8: Determines polarity on pin F1"]
    F1,
    #[doc = "16: Determines polarity on pin F2"]
    F2,
    #[doc = "32: Determines polarity on pin E13"]
    E13,
    #[doc = "64: Determines polarity on pin C4"]
    C4,
}
impl From<EM4WUPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: EM4WUPOL_A) -> Self {
        match variant {
            EM4WUPOL_A::A0 => 1,
            EM4WUPOL_A::C9 => 4,
            EM4WUPOL_A::F1 => 8,
            EM4WUPOL_A::F2 => 16,
            EM4WUPOL_A::E13 => 32,
            EM4WUPOL_A::C4 => 64,
        }
    }
}
#[doc = "Reader of field `EM4WUPOL`"]
pub type EM4WUPOL_R = crate::R<u8, EM4WUPOL_A>;
impl EM4WUPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EM4WUPOL_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(EM4WUPOL_A::A0),
            4 => Val(EM4WUPOL_A::C9),
            8 => Val(EM4WUPOL_A::F1),
            16 => Val(EM4WUPOL_A::F2),
            32 => Val(EM4WUPOL_A::E13),
            64 => Val(EM4WUPOL_A::C4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `A0`"]
    #[inline(always)]
    pub fn is_a0(&self) -> bool {
        *self == EM4WUPOL_A::A0
    }
    #[doc = "Checks if the value of the field is `C9`"]
    #[inline(always)]
    pub fn is_c9(&self) -> bool {
        *self == EM4WUPOL_A::C9
    }
    #[doc = "Checks if the value of the field is `F1`"]
    #[inline(always)]
    pub fn is_f1(&self) -> bool {
        *self == EM4WUPOL_A::F1
    }
    #[doc = "Checks if the value of the field is `F2`"]
    #[inline(always)]
    pub fn is_f2(&self) -> bool {
        *self == EM4WUPOL_A::F2
    }
    #[doc = "Checks if the value of the field is `E13`"]
    #[inline(always)]
    pub fn is_e13(&self) -> bool {
        *self == EM4WUPOL_A::E13
    }
    #[doc = "Checks if the value of the field is `C4`"]
    #[inline(always)]
    pub fn is_c4(&self) -> bool {
        *self == EM4WUPOL_A::C4
    }
}
#[doc = "Write proxy for field `EM4WUPOL`"]
pub struct EM4WUPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4WUPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM4WUPOL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Determines polarity on pin A0"]
    #[inline(always)]
    pub fn a0(self) -> &'a mut W {
        self.variant(EM4WUPOL_A::A0)
    }
    #[doc = "Determines polarity on pin C9"]
    #[inline(always)]
    pub fn c9(self) -> &'a mut W {
        self.variant(EM4WUPOL_A::C9)
    }
    #[doc = "Determines polarity on pin F1"]
    #[inline(always)]
    pub fn f1(self) -> &'a mut W {
        self.variant(EM4WUPOL_A::F1)
    }
    #[doc = "Determines polarity on pin F2"]
    #[inline(always)]
    pub fn f2(self) -> &'a mut W {
        self.variant(EM4WUPOL_A::F2)
    }
    #[doc = "Determines polarity on pin E13"]
    #[inline(always)]
    pub fn e13(self) -> &'a mut W {
        self.variant(EM4WUPOL_A::E13)
    }
    #[doc = "Determines polarity on pin C4"]
    #[inline(always)]
    pub fn c4(self) -> &'a mut W {
        self.variant(EM4WUPOL_A::C4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - EM4 Wake-up Polarity"]
    #[inline(always)]
    pub fn em4wupol(&self) -> EM4WUPOL_R {
        EM4WUPOL_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - EM4 Wake-up Polarity"]
    #[inline(always)]
    pub fn em4wupol(&mut self) -> EM4WUPOL_W {
        EM4WUPOL_W { w: self }
    }
}
