#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x08 - Status Register"]
    pub status: STATUS,
    #[doc = "0x0c - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x10 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x14 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x18 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x1c - Counter Top Value Register"]
    pub top: TOP,
    #[doc = "0x20 - Counter Top Value Buffer Register"]
    pub topb: TOPB,
    #[doc = "0x24 - Counter Value Register"]
    pub cnt: CNT,
    #[doc = "0x28 - I/O Routing Register"]
    pub route: ROUTE,
    _reserved11: [u8; 4usize],
    #[doc = "0x30 - CC Channel Control Register"]
    pub cc0_ctrl: CC0_CTRL,
    #[doc = "0x34 - CC Channel Value Register"]
    pub cc0_ccv: CC0_CCV,
    #[doc = "0x38 - CC Channel Value Peek Register"]
    pub cc0_ccvp: CC0_CCVP,
    #[doc = "0x3c - CC Channel Buffer Register"]
    pub cc0_ccvb: CC0_CCVB,
    #[doc = "0x40 - CC Channel Control Register"]
    pub cc1_ctrl: CC1_CTRL,
    #[doc = "0x44 - CC Channel Value Register"]
    pub cc1_ccv: CC1_CCV,
    #[doc = "0x48 - CC Channel Value Peek Register"]
    pub cc1_ccvp: CC1_CCVP,
    #[doc = "0x4c - CC Channel Buffer Register"]
    pub cc1_ccvb: CC1_CCVB,
    #[doc = "0x50 - CC Channel Control Register"]
    pub cc2_ctrl: CC2_CTRL,
    #[doc = "0x54 - CC Channel Value Register"]
    pub cc2_ccv: CC2_CCV,
    #[doc = "0x58 - CC Channel Value Peek Register"]
    pub cc2_ccvp: CC2_CCVP,
    #[doc = "0x5c - CC Channel Buffer Register"]
    pub cc2_ccvb: CC2_CCVB,
    _reserved23: [u8; 16usize],
    #[doc = "0x70 - DTI Control Register"]
    pub dtctrl: DTCTRL,
    #[doc = "0x74 - DTI Time Control Register"]
    pub dttime: DTTIME,
    #[doc = "0x78 - DTI Fault Configuration Register"]
    pub dtfc: DTFC,
    #[doc = "0x7c - DTI Output Generation Enable Register"]
    pub dtogen: DTOGEN,
    #[doc = "0x80 - DTI Fault Register"]
    pub dtfault: DTFAULT,
    #[doc = "0x84 - DTI Fault Clear Register"]
    pub dtfaultc: DTFAULTC,
    #[doc = "0x88 - DTI Configuration Lock Register"]
    pub dtlock: DTLOCK,
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
#[doc = "Command Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status Register"]
pub mod status;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ien](ien) module"]
pub type IEN = crate::Reg<u32, _IEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEN;
#[doc = "`read()` method returns [ien::R](ien::R) reader structure"]
impl crate::Readable for IEN {}
#[doc = "`write(|w| ..)` method takes [ien::W](ien::W) writer structure"]
impl crate::Writable for IEN {}
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [if_](if_) module"]
pub type IF = crate::Reg<u32, _IF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF;
#[doc = "`read()` method returns [if_::R](if_::R) reader structure"]
impl crate::Readable for IF {}
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "Interrupt Flag Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ifs](ifs) module"]
pub type IFS = crate::Reg<u32, _IFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFS;
#[doc = "`write(|w| ..)` method takes [ifs::W](ifs::W) writer structure"]
impl crate::Writable for IFS {}
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "Interrupt Flag Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ifc](ifc) module"]
pub type IFC = crate::Reg<u32, _IFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFC;
#[doc = "`write(|w| ..)` method takes [ifc::W](ifc::W) writer structure"]
impl crate::Writable for IFC {}
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "Counter Top Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [top](top) module"]
pub type TOP = crate::Reg<u32, _TOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOP;
#[doc = "`read()` method returns [top::R](top::R) reader structure"]
impl crate::Readable for TOP {}
#[doc = "`write(|w| ..)` method takes [top::W](top::W) writer structure"]
impl crate::Writable for TOP {}
#[doc = "Counter Top Value Register"]
pub mod top;
#[doc = "Counter Top Value Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [topb](topb) module"]
pub type TOPB = crate::Reg<u32, _TOPB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOPB;
#[doc = "`read()` method returns [topb::R](topb::R) reader structure"]
impl crate::Readable for TOPB {}
#[doc = "`write(|w| ..)` method takes [topb::W](topb::W) writer structure"]
impl crate::Writable for TOPB {}
#[doc = "Counter Top Value Buffer Register"]
pub mod topb;
#[doc = "Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cnt](cnt) module"]
pub type CNT = crate::Reg<u32, _CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT;
#[doc = "`read()` method returns [cnt::R](cnt::R) reader structure"]
impl crate::Readable for CNT {}
#[doc = "`write(|w| ..)` method takes [cnt::W](cnt::W) writer structure"]
impl crate::Writable for CNT {}
#[doc = "Counter Value Register"]
pub mod cnt;
#[doc = "I/O Routing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [route](route) module"]
pub type ROUTE = crate::Reg<u32, _ROUTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTE;
#[doc = "`read()` method returns [route::R](route::R) reader structure"]
impl crate::Readable for ROUTE {}
#[doc = "`write(|w| ..)` method takes [route::W](route::W) writer structure"]
impl crate::Writable for ROUTE {}
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "CC Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc0_ctrl](cc0_ctrl) module"]
pub type CC0_CTRL = crate::Reg<u32, _CC0_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC0_CTRL;
#[doc = "`read()` method returns [cc0_ctrl::R](cc0_ctrl::R) reader structure"]
impl crate::Readable for CC0_CTRL {}
#[doc = "`write(|w| ..)` method takes [cc0_ctrl::W](cc0_ctrl::W) writer structure"]
impl crate::Writable for CC0_CTRL {}
#[doc = "CC Channel Control Register"]
pub mod cc0_ctrl;
#[doc = "CC Channel Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc0_ccv](cc0_ccv) module"]
pub type CC0_CCV = crate::Reg<u32, _CC0_CCV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC0_CCV;
#[doc = "`read()` method returns [cc0_ccv::R](cc0_ccv::R) reader structure"]
impl crate::Readable for CC0_CCV {}
#[doc = "`write(|w| ..)` method takes [cc0_ccv::W](cc0_ccv::W) writer structure"]
impl crate::Writable for CC0_CCV {}
#[doc = "CC Channel Value Register"]
pub mod cc0_ccv;
#[doc = "CC Channel Value Peek Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc0_ccvp](cc0_ccvp) module"]
pub type CC0_CCVP = crate::Reg<u32, _CC0_CCVP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC0_CCVP;
#[doc = "`read()` method returns [cc0_ccvp::R](cc0_ccvp::R) reader structure"]
impl crate::Readable for CC0_CCVP {}
#[doc = "CC Channel Value Peek Register"]
pub mod cc0_ccvp;
#[doc = "CC Channel Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc0_ccvb](cc0_ccvb) module"]
pub type CC0_CCVB = crate::Reg<u32, _CC0_CCVB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC0_CCVB;
#[doc = "`read()` method returns [cc0_ccvb::R](cc0_ccvb::R) reader structure"]
impl crate::Readable for CC0_CCVB {}
#[doc = "`write(|w| ..)` method takes [cc0_ccvb::W](cc0_ccvb::W) writer structure"]
impl crate::Writable for CC0_CCVB {}
#[doc = "CC Channel Buffer Register"]
pub mod cc0_ccvb;
#[doc = "CC Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc1_ctrl](cc1_ctrl) module"]
pub type CC1_CTRL = crate::Reg<u32, _CC1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC1_CTRL;
#[doc = "`read()` method returns [cc1_ctrl::R](cc1_ctrl::R) reader structure"]
impl crate::Readable for CC1_CTRL {}
#[doc = "`write(|w| ..)` method takes [cc1_ctrl::W](cc1_ctrl::W) writer structure"]
impl crate::Writable for CC1_CTRL {}
#[doc = "CC Channel Control Register"]
pub mod cc1_ctrl;
#[doc = "CC Channel Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc1_ccv](cc1_ccv) module"]
pub type CC1_CCV = crate::Reg<u32, _CC1_CCV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC1_CCV;
#[doc = "`read()` method returns [cc1_ccv::R](cc1_ccv::R) reader structure"]
impl crate::Readable for CC1_CCV {}
#[doc = "`write(|w| ..)` method takes [cc1_ccv::W](cc1_ccv::W) writer structure"]
impl crate::Writable for CC1_CCV {}
#[doc = "CC Channel Value Register"]
pub mod cc1_ccv;
#[doc = "CC Channel Value Peek Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc1_ccvp](cc1_ccvp) module"]
pub type CC1_CCVP = crate::Reg<u32, _CC1_CCVP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC1_CCVP;
#[doc = "`read()` method returns [cc1_ccvp::R](cc1_ccvp::R) reader structure"]
impl crate::Readable for CC1_CCVP {}
#[doc = "CC Channel Value Peek Register"]
pub mod cc1_ccvp;
#[doc = "CC Channel Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc1_ccvb](cc1_ccvb) module"]
pub type CC1_CCVB = crate::Reg<u32, _CC1_CCVB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC1_CCVB;
#[doc = "`read()` method returns [cc1_ccvb::R](cc1_ccvb::R) reader structure"]
impl crate::Readable for CC1_CCVB {}
#[doc = "`write(|w| ..)` method takes [cc1_ccvb::W](cc1_ccvb::W) writer structure"]
impl crate::Writable for CC1_CCVB {}
#[doc = "CC Channel Buffer Register"]
pub mod cc1_ccvb;
#[doc = "CC Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc2_ctrl](cc2_ctrl) module"]
pub type CC2_CTRL = crate::Reg<u32, _CC2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC2_CTRL;
#[doc = "`read()` method returns [cc2_ctrl::R](cc2_ctrl::R) reader structure"]
impl crate::Readable for CC2_CTRL {}
#[doc = "`write(|w| ..)` method takes [cc2_ctrl::W](cc2_ctrl::W) writer structure"]
impl crate::Writable for CC2_CTRL {}
#[doc = "CC Channel Control Register"]
pub mod cc2_ctrl;
#[doc = "CC Channel Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc2_ccv](cc2_ccv) module"]
pub type CC2_CCV = crate::Reg<u32, _CC2_CCV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC2_CCV;
#[doc = "`read()` method returns [cc2_ccv::R](cc2_ccv::R) reader structure"]
impl crate::Readable for CC2_CCV {}
#[doc = "`write(|w| ..)` method takes [cc2_ccv::W](cc2_ccv::W) writer structure"]
impl crate::Writable for CC2_CCV {}
#[doc = "CC Channel Value Register"]
pub mod cc2_ccv;
#[doc = "CC Channel Value Peek Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc2_ccvp](cc2_ccvp) module"]
pub type CC2_CCVP = crate::Reg<u32, _CC2_CCVP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC2_CCVP;
#[doc = "`read()` method returns [cc2_ccvp::R](cc2_ccvp::R) reader structure"]
impl crate::Readable for CC2_CCVP {}
#[doc = "CC Channel Value Peek Register"]
pub mod cc2_ccvp;
#[doc = "CC Channel Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc2_ccvb](cc2_ccvb) module"]
pub type CC2_CCVB = crate::Reg<u32, _CC2_CCVB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC2_CCVB;
#[doc = "`read()` method returns [cc2_ccvb::R](cc2_ccvb::R) reader structure"]
impl crate::Readable for CC2_CCVB {}
#[doc = "`write(|w| ..)` method takes [cc2_ccvb::W](cc2_ccvb::W) writer structure"]
impl crate::Writable for CC2_CCVB {}
#[doc = "CC Channel Buffer Register"]
pub mod cc2_ccvb;
#[doc = "DTI Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dtctrl](dtctrl) module"]
pub type DTCTRL = crate::Reg<u32, _DTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTCTRL;
#[doc = "`read()` method returns [dtctrl::R](dtctrl::R) reader structure"]
impl crate::Readable for DTCTRL {}
#[doc = "`write(|w| ..)` method takes [dtctrl::W](dtctrl::W) writer structure"]
impl crate::Writable for DTCTRL {}
#[doc = "DTI Control Register"]
pub mod dtctrl;
#[doc = "DTI Time Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dttime](dttime) module"]
pub type DTTIME = crate::Reg<u32, _DTTIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTTIME;
#[doc = "`read()` method returns [dttime::R](dttime::R) reader structure"]
impl crate::Readable for DTTIME {}
#[doc = "`write(|w| ..)` method takes [dttime::W](dttime::W) writer structure"]
impl crate::Writable for DTTIME {}
#[doc = "DTI Time Control Register"]
pub mod dttime;
#[doc = "DTI Fault Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dtfc](dtfc) module"]
pub type DTFC = crate::Reg<u32, _DTFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTFC;
#[doc = "`read()` method returns [dtfc::R](dtfc::R) reader structure"]
impl crate::Readable for DTFC {}
#[doc = "`write(|w| ..)` method takes [dtfc::W](dtfc::W) writer structure"]
impl crate::Writable for DTFC {}
#[doc = "DTI Fault Configuration Register"]
pub mod dtfc;
#[doc = "DTI Output Generation Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dtogen](dtogen) module"]
pub type DTOGEN = crate::Reg<u32, _DTOGEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTOGEN;
#[doc = "`read()` method returns [dtogen::R](dtogen::R) reader structure"]
impl crate::Readable for DTOGEN {}
#[doc = "`write(|w| ..)` method takes [dtogen::W](dtogen::W) writer structure"]
impl crate::Writable for DTOGEN {}
#[doc = "DTI Output Generation Enable Register"]
pub mod dtogen;
#[doc = "DTI Fault Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dtfault](dtfault) module"]
pub type DTFAULT = crate::Reg<u32, _DTFAULT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTFAULT;
#[doc = "`read()` method returns [dtfault::R](dtfault::R) reader structure"]
impl crate::Readable for DTFAULT {}
#[doc = "DTI Fault Register"]
pub mod dtfault;
#[doc = "DTI Fault Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dtfaultc](dtfaultc) module"]
pub type DTFAULTC = crate::Reg<u32, _DTFAULTC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTFAULTC;
#[doc = "`write(|w| ..)` method takes [dtfaultc::W](dtfaultc::W) writer structure"]
impl crate::Writable for DTFAULTC {}
#[doc = "DTI Fault Clear Register"]
pub mod dtfaultc;
#[doc = "DTI Configuration Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dtlock](dtlock) module"]
pub type DTLOCK = crate::Reg<u32, _DTLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTLOCK;
#[doc = "`read()` method returns [dtlock::R](dtlock::R) reader structure"]
impl crate::Readable for DTLOCK {}
#[doc = "`write(|w| ..)` method takes [dtlock::W](dtlock::W) writer structure"]
impl crate::Writable for DTLOCK {}
#[doc = "DTI Configuration Lock Register"]
pub mod dtlock;
