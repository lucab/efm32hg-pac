#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Current Programming Register"]
    pub curprog: CURPROG,
    #[doc = "0x08 - Calibration Register"]
    pub cal: CAL,
    #[doc = "0x0c - Duty Cycle Configauration Register"]
    pub dutyconfig: DUTYCONFIG,
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Current Programming Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [curprog](curprog) module"]
pub type CURPROG = crate::Reg<u32, _CURPROG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CURPROG;
#[doc = "`read()` method returns [curprog::R](curprog::R) reader structure"]
impl crate::Readable for CURPROG {}
#[doc = "`write(|w| ..)` method takes [curprog::W](curprog::W) writer structure"]
impl crate::Writable for CURPROG {}
#[doc = "Current Programming Register"]
pub mod curprog;
#[doc = "Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cal](cal) module"]
pub type CAL = crate::Reg<u32, _CAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAL;
#[doc = "`read()` method returns [cal::R](cal::R) reader structure"]
impl crate::Readable for CAL {}
#[doc = "`write(|w| ..)` method takes [cal::W](cal::W) writer structure"]
impl crate::Writable for CAL {}
#[doc = "Calibration Register"]
pub mod cal;
#[doc = "Duty Cycle Configauration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dutyconfig](dutyconfig) module"]
pub type DUTYCONFIG = crate::Reg<u32, _DUTYCONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DUTYCONFIG;
#[doc = "`read()` method returns [dutyconfig::R](dutyconfig::R) reader structure"]
impl crate::Readable for DUTYCONFIG {}
#[doc = "`write(|w| ..)` method takes [dutyconfig::W](dutyconfig::W) writer structure"]
impl crate::Writable for DUTYCONFIG {}
#[doc = "Duty Cycle Configauration Register"]
pub mod dutyconfig;
