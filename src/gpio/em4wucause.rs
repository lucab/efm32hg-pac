#[doc = "Reader of register EM4WUCAUSE"]
pub type R = crate::R<u32, super::EM4WUCAUSE>;
#[doc = "EM4 wake-up cause\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM4WUCAUSE_A {
    #[doc = "1: This bit indicates an em4 wake-up request occurred on pin A0"]
    A0,
    #[doc = "4: This bit indicates an em4 wake-up request occurred on pin C9"]
    C9,
    #[doc = "8: This bit indicates an em4 wake-up request occurred on pin F1"]
    F1,
    #[doc = "16: This bit indicates an em4 wake-up request occurred on pin F2"]
    F2,
    #[doc = "32: This bit indicates an em4 wake-up request occurred on pin E13"]
    E13,
    #[doc = "64: This bit indicates an em4 wake-up request occurred on pin C4"]
    C4,
}
impl From<EM4WUCAUSE_A> for u8 {
    #[inline(always)]
    fn from(variant: EM4WUCAUSE_A) -> Self {
        match variant {
            EM4WUCAUSE_A::A0 => 1,
            EM4WUCAUSE_A::C9 => 4,
            EM4WUCAUSE_A::F1 => 8,
            EM4WUCAUSE_A::F2 => 16,
            EM4WUCAUSE_A::E13 => 32,
            EM4WUCAUSE_A::C4 => 64,
        }
    }
}
#[doc = "Reader of field `EM4WUCAUSE`"]
pub type EM4WUCAUSE_R = crate::R<u8, EM4WUCAUSE_A>;
impl EM4WUCAUSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EM4WUCAUSE_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(EM4WUCAUSE_A::A0),
            4 => Val(EM4WUCAUSE_A::C9),
            8 => Val(EM4WUCAUSE_A::F1),
            16 => Val(EM4WUCAUSE_A::F2),
            32 => Val(EM4WUCAUSE_A::E13),
            64 => Val(EM4WUCAUSE_A::C4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `A0`"]
    #[inline(always)]
    pub fn is_a0(&self) -> bool {
        *self == EM4WUCAUSE_A::A0
    }
    #[doc = "Checks if the value of the field is `C9`"]
    #[inline(always)]
    pub fn is_c9(&self) -> bool {
        *self == EM4WUCAUSE_A::C9
    }
    #[doc = "Checks if the value of the field is `F1`"]
    #[inline(always)]
    pub fn is_f1(&self) -> bool {
        *self == EM4WUCAUSE_A::F1
    }
    #[doc = "Checks if the value of the field is `F2`"]
    #[inline(always)]
    pub fn is_f2(&self) -> bool {
        *self == EM4WUCAUSE_A::F2
    }
    #[doc = "Checks if the value of the field is `E13`"]
    #[inline(always)]
    pub fn is_e13(&self) -> bool {
        *self == EM4WUCAUSE_A::E13
    }
    #[doc = "Checks if the value of the field is `C4`"]
    #[inline(always)]
    pub fn is_c4(&self) -> bool {
        *self == EM4WUCAUSE_A::C4
    }
}
impl R {
    #[doc = "Bits 0:6 - EM4 wake-up cause"]
    #[inline(always)]
    pub fn em4wucause(&self) -> EM4WUCAUSE_R {
        EM4WUCAUSE_R::new((self.bits & 0x7f) as u8)
    }
}
