#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MTB Trace Position Register."]
    pub position: POSITION,
    #[doc = "0x04 - MTB Trace Control Register"]
    pub master: MASTER,
    #[doc = "0x08 - MTB Trace Flow Register"]
    pub flow: FLOW,
    #[doc = "0x0c - MTB Trace Base Register"]
    pub base: BASE,
}
#[doc = "MTB Trace Position Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [position](position) module"]
pub type POSITION = crate::Reg<u32, _POSITION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POSITION;
#[doc = "`read()` method returns [position::R](position::R) reader structure"]
impl crate::Readable for POSITION {}
#[doc = "`write(|w| ..)` method takes [position::W](position::W) writer structure"]
impl crate::Writable for POSITION {}
#[doc = "MTB Trace Position Register."]
pub mod position;
#[doc = "MTB Trace Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [master](master) module"]
pub type MASTER = crate::Reg<u32, _MASTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASTER;
#[doc = "`read()` method returns [master::R](master::R) reader structure"]
impl crate::Readable for MASTER {}
#[doc = "`write(|w| ..)` method takes [master::W](master::W) writer structure"]
impl crate::Writable for MASTER {}
#[doc = "MTB Trace Control Register"]
pub mod master;
#[doc = "MTB Trace Flow Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flow](flow) module"]
pub type FLOW = crate::Reg<u32, _FLOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLOW;
#[doc = "`read()` method returns [flow::R](flow::R) reader structure"]
impl crate::Readable for FLOW {}
#[doc = "`write(|w| ..)` method takes [flow::W](flow::W) writer structure"]
impl crate::Writable for FLOW {}
#[doc = "MTB Trace Flow Register"]
pub mod flow;
#[doc = "MTB Trace Base Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [base](base) module"]
pub type BASE = crate::Reg<u32, _BASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BASE;
#[doc = "`read()` method returns [base::R](base::R) reader structure"]
impl crate::Readable for BASE {}
#[doc = "`write(|w| ..)` method takes [base::W](base::W) writer structure"]
impl crate::Writable for BASE {}
#[doc = "MTB Trace Base Register"]
pub mod base;
