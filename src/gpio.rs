#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port Control Register"]
    pub pa_ctrl: PA_CTRL,
    #[doc = "0x04 - Port Pin Mode Low Register"]
    pub pa_model: PA_MODEL,
    #[doc = "0x08 - Port Pin Mode High Register"]
    pub pa_modeh: PA_MODEH,
    #[doc = "0x0c - Port Data Out Register"]
    pub pa_dout: PA_DOUT,
    #[doc = "0x10 - Port Data Out Set Register"]
    pub pa_doutset: PA_DOUTSET,
    #[doc = "0x14 - Port Data Out Clear Register"]
    pub pa_doutclr: PA_DOUTCLR,
    #[doc = "0x18 - Port Data Out Toggle Register"]
    pub pa_douttgl: PA_DOUTTGL,
    #[doc = "0x1c - Port Data In Register"]
    pub pa_din: PA_DIN,
    #[doc = "0x20 - Port Unlocked Pins Register"]
    pub pa_pinlockn: PA_PINLOCKN,
    #[doc = "0x24 - Port Control Register"]
    pub pb_ctrl: PB_CTRL,
    #[doc = "0x28 - Port Pin Mode Low Register"]
    pub pb_model: PB_MODEL,
    #[doc = "0x2c - Port Pin Mode High Register"]
    pub pb_modeh: PB_MODEH,
    #[doc = "0x30 - Port Data Out Register"]
    pub pb_dout: PB_DOUT,
    #[doc = "0x34 - Port Data Out Set Register"]
    pub pb_doutset: PB_DOUTSET,
    #[doc = "0x38 - Port Data Out Clear Register"]
    pub pb_doutclr: PB_DOUTCLR,
    #[doc = "0x3c - Port Data Out Toggle Register"]
    pub pb_douttgl: PB_DOUTTGL,
    #[doc = "0x40 - Port Data In Register"]
    pub pb_din: PB_DIN,
    #[doc = "0x44 - Port Unlocked Pins Register"]
    pub pb_pinlockn: PB_PINLOCKN,
    #[doc = "0x48 - Port Control Register"]
    pub pc_ctrl: PC_CTRL,
    #[doc = "0x4c - Port Pin Mode Low Register"]
    pub pc_model: PC_MODEL,
    #[doc = "0x50 - Port Pin Mode High Register"]
    pub pc_modeh: PC_MODEH,
    #[doc = "0x54 - Port Data Out Register"]
    pub pc_dout: PC_DOUT,
    #[doc = "0x58 - Port Data Out Set Register"]
    pub pc_doutset: PC_DOUTSET,
    #[doc = "0x5c - Port Data Out Clear Register"]
    pub pc_doutclr: PC_DOUTCLR,
    #[doc = "0x60 - Port Data Out Toggle Register"]
    pub pc_douttgl: PC_DOUTTGL,
    #[doc = "0x64 - Port Data In Register"]
    pub pc_din: PC_DIN,
    #[doc = "0x68 - Port Unlocked Pins Register"]
    pub pc_pinlockn: PC_PINLOCKN,
    #[doc = "0x6c - Port Control Register"]
    pub pd_ctrl: PD_CTRL,
    #[doc = "0x70 - Port Pin Mode Low Register"]
    pub pd_model: PD_MODEL,
    #[doc = "0x74 - Port Pin Mode High Register"]
    pub pd_modeh: PD_MODEH,
    #[doc = "0x78 - Port Data Out Register"]
    pub pd_dout: PD_DOUT,
    #[doc = "0x7c - Port Data Out Set Register"]
    pub pd_doutset: PD_DOUTSET,
    #[doc = "0x80 - Port Data Out Clear Register"]
    pub pd_doutclr: PD_DOUTCLR,
    #[doc = "0x84 - Port Data Out Toggle Register"]
    pub pd_douttgl: PD_DOUTTGL,
    #[doc = "0x88 - Port Data In Register"]
    pub pd_din: PD_DIN,
    #[doc = "0x8c - Port Unlocked Pins Register"]
    pub pd_pinlockn: PD_PINLOCKN,
    #[doc = "0x90 - Port Control Register"]
    pub pe_ctrl: PE_CTRL,
    #[doc = "0x94 - Port Pin Mode Low Register"]
    pub pe_model: PE_MODEL,
    #[doc = "0x98 - Port Pin Mode High Register"]
    pub pe_modeh: PE_MODEH,
    #[doc = "0x9c - Port Data Out Register"]
    pub pe_dout: PE_DOUT,
    #[doc = "0xa0 - Port Data Out Set Register"]
    pub pe_doutset: PE_DOUTSET,
    #[doc = "0xa4 - Port Data Out Clear Register"]
    pub pe_doutclr: PE_DOUTCLR,
    #[doc = "0xa8 - Port Data Out Toggle Register"]
    pub pe_douttgl: PE_DOUTTGL,
    #[doc = "0xac - Port Data In Register"]
    pub pe_din: PE_DIN,
    #[doc = "0xb0 - Port Unlocked Pins Register"]
    pub pe_pinlockn: PE_PINLOCKN,
    #[doc = "0xb4 - Port Control Register"]
    pub pf_ctrl: PF_CTRL,
    #[doc = "0xb8 - Port Pin Mode Low Register"]
    pub pf_model: PF_MODEL,
    #[doc = "0xbc - Port Pin Mode High Register"]
    pub pf_modeh: PF_MODEH,
    #[doc = "0xc0 - Port Data Out Register"]
    pub pf_dout: PF_DOUT,
    #[doc = "0xc4 - Port Data Out Set Register"]
    pub pf_doutset: PF_DOUTSET,
    #[doc = "0xc8 - Port Data Out Clear Register"]
    pub pf_doutclr: PF_DOUTCLR,
    #[doc = "0xcc - Port Data Out Toggle Register"]
    pub pf_douttgl: PF_DOUTTGL,
    #[doc = "0xd0 - Port Data In Register"]
    pub pf_din: PF_DIN,
    #[doc = "0xd4 - Port Unlocked Pins Register"]
    pub pf_pinlockn: PF_PINLOCKN,
    _reserved54: [u8; 40usize],
    #[doc = "0x100 - External Interrupt Port Select Low Register"]
    pub extipsell: EXTIPSELL,
    #[doc = "0x104 - External Interrupt Port Select High Register"]
    pub extipselh: EXTIPSELH,
    #[doc = "0x108 - External Interrupt Rising Edge Trigger Register"]
    pub extirise: EXTIRISE,
    #[doc = "0x10c - External Interrupt Falling Edge Trigger Register"]
    pub extifall: EXTIFALL,
    #[doc = "0x110 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x114 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x118 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x11c - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x120 - I/O Routing Register"]
    pub route: ROUTE,
    #[doc = "0x124 - Input Sense Register"]
    pub insense: INSENSE,
    #[doc = "0x128 - Configuration Lock Register"]
    pub lock: LOCK,
    #[doc = "0x12c - GPIO Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x130 - GPIO Command Register"]
    pub cmd: CMD,
    #[doc = "0x134 - EM4 Wake-up Enable Register"]
    pub em4wuen: EM4WUEN,
    #[doc = "0x138 - EM4 Wake-up Polarity Register"]
    pub em4wupol: EM4WUPOL,
    #[doc = "0x13c - EM4 Wake-up Cause Register"]
    pub em4wucause: EM4WUCAUSE,
}
#[doc = "Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pa_ctrl](pa_ctrl) module"]
pub type PA_CTRL = crate::Reg<u32, _PA_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA_CTRL;
#[doc = "`read()` method returns [pa_ctrl::R](pa_ctrl::R) reader structure"]
impl crate::Readable for PA_CTRL {}
#[doc = "`write(|w| ..)` method takes [pa_ctrl::W](pa_ctrl::W) writer structure"]
impl crate::Writable for PA_CTRL {}
#[doc = "Port Control Register"]
pub mod pa_ctrl;
#[doc = "Port Pin Mode Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pa_model](pa_model) module"]
pub type PA_MODEL = crate::Reg<u32, _PA_MODEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA_MODEL;
#[doc = "`read()` method returns [pa_model::R](pa_model::R) reader structure"]
impl crate::Readable for PA_MODEL {}
#[doc = "`write(|w| ..)` method takes [pa_model::W](pa_model::W) writer structure"]
impl crate::Writable for PA_MODEL {}
#[doc = "Port Pin Mode Low Register"]
pub mod pa_model;
#[doc = "Port Pin Mode High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pa_modeh](pa_modeh) module"]
pub type PA_MODEH = crate::Reg<u32, _PA_MODEH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA_MODEH;
#[doc = "`read()` method returns [pa_modeh::R](pa_modeh::R) reader structure"]
impl crate::Readable for PA_MODEH {}
#[doc = "`write(|w| ..)` method takes [pa_modeh::W](pa_modeh::W) writer structure"]
impl crate::Writable for PA_MODEH {}
#[doc = "Port Pin Mode High Register"]
pub mod pa_modeh;
#[doc = "Port Data Out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pa_dout](pa_dout) module"]
pub type PA_DOUT = crate::Reg<u32, _PA_DOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA_DOUT;
#[doc = "`read()` method returns [pa_dout::R](pa_dout::R) reader structure"]
impl crate::Readable for PA_DOUT {}
#[doc = "`write(|w| ..)` method takes [pa_dout::W](pa_dout::W) writer structure"]
impl crate::Writable for PA_DOUT {}
#[doc = "Port Data Out Register"]
pub mod pa_dout;
#[doc = "Port Data Out Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pa_doutset](pa_doutset) module"]
pub type PA_DOUTSET = crate::Reg<u32, _PA_DOUTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA_DOUTSET;
#[doc = "`write(|w| ..)` method takes [pa_doutset::W](pa_doutset::W) writer structure"]
impl crate::Writable for PA_DOUTSET {}
#[doc = "Port Data Out Set Register"]
pub mod pa_doutset;
#[doc = "Port Data Out Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pa_doutclr](pa_doutclr) module"]
pub type PA_DOUTCLR = crate::Reg<u32, _PA_DOUTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA_DOUTCLR;
#[doc = "`write(|w| ..)` method takes [pa_doutclr::W](pa_doutclr::W) writer structure"]
impl crate::Writable for PA_DOUTCLR {}
#[doc = "Port Data Out Clear Register"]
pub mod pa_doutclr;
#[doc = "Port Data Out Toggle Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pa_douttgl](pa_douttgl) module"]
pub type PA_DOUTTGL = crate::Reg<u32, _PA_DOUTTGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA_DOUTTGL;
#[doc = "`write(|w| ..)` method takes [pa_douttgl::W](pa_douttgl::W) writer structure"]
impl crate::Writable for PA_DOUTTGL {}
#[doc = "Port Data Out Toggle Register"]
pub mod pa_douttgl;
#[doc = "Port Data In Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pa_din](pa_din) module"]
pub type PA_DIN = crate::Reg<u32, _PA_DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA_DIN;
#[doc = "`read()` method returns [pa_din::R](pa_din::R) reader structure"]
impl crate::Readable for PA_DIN {}
#[doc = "Port Data In Register"]
pub mod pa_din;
#[doc = "Port Unlocked Pins Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pa_pinlockn](pa_pinlockn) module"]
pub type PA_PINLOCKN = crate::Reg<u32, _PA_PINLOCKN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA_PINLOCKN;
#[doc = "`read()` method returns [pa_pinlockn::R](pa_pinlockn::R) reader structure"]
impl crate::Readable for PA_PINLOCKN {}
#[doc = "`write(|w| ..)` method takes [pa_pinlockn::W](pa_pinlockn::W) writer structure"]
impl crate::Writable for PA_PINLOCKN {}
#[doc = "Port Unlocked Pins Register"]
pub mod pa_pinlockn;
#[doc = "Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pb_ctrl](pb_ctrl) module"]
pub type PB_CTRL = crate::Reg<u32, _PB_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB_CTRL;
#[doc = "`read()` method returns [pb_ctrl::R](pb_ctrl::R) reader structure"]
impl crate::Readable for PB_CTRL {}
#[doc = "`write(|w| ..)` method takes [pb_ctrl::W](pb_ctrl::W) writer structure"]
impl crate::Writable for PB_CTRL {}
#[doc = "Port Control Register"]
pub mod pb_ctrl;
#[doc = "Port Pin Mode Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pb_model](pb_model) module"]
pub type PB_MODEL = crate::Reg<u32, _PB_MODEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB_MODEL;
#[doc = "`read()` method returns [pb_model::R](pb_model::R) reader structure"]
impl crate::Readable for PB_MODEL {}
#[doc = "`write(|w| ..)` method takes [pb_model::W](pb_model::W) writer structure"]
impl crate::Writable for PB_MODEL {}
#[doc = "Port Pin Mode Low Register"]
pub mod pb_model;
#[doc = "Port Pin Mode High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pb_modeh](pb_modeh) module"]
pub type PB_MODEH = crate::Reg<u32, _PB_MODEH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB_MODEH;
#[doc = "`read()` method returns [pb_modeh::R](pb_modeh::R) reader structure"]
impl crate::Readable for PB_MODEH {}
#[doc = "`write(|w| ..)` method takes [pb_modeh::W](pb_modeh::W) writer structure"]
impl crate::Writable for PB_MODEH {}
#[doc = "Port Pin Mode High Register"]
pub mod pb_modeh;
#[doc = "Port Data Out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pb_dout](pb_dout) module"]
pub type PB_DOUT = crate::Reg<u32, _PB_DOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB_DOUT;
#[doc = "`read()` method returns [pb_dout::R](pb_dout::R) reader structure"]
impl crate::Readable for PB_DOUT {}
#[doc = "`write(|w| ..)` method takes [pb_dout::W](pb_dout::W) writer structure"]
impl crate::Writable for PB_DOUT {}
#[doc = "Port Data Out Register"]
pub mod pb_dout;
#[doc = "Port Data Out Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pb_doutset](pb_doutset) module"]
pub type PB_DOUTSET = crate::Reg<u32, _PB_DOUTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB_DOUTSET;
#[doc = "`write(|w| ..)` method takes [pb_doutset::W](pb_doutset::W) writer structure"]
impl crate::Writable for PB_DOUTSET {}
#[doc = "Port Data Out Set Register"]
pub mod pb_doutset;
#[doc = "Port Data Out Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pb_doutclr](pb_doutclr) module"]
pub type PB_DOUTCLR = crate::Reg<u32, _PB_DOUTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB_DOUTCLR;
#[doc = "`write(|w| ..)` method takes [pb_doutclr::W](pb_doutclr::W) writer structure"]
impl crate::Writable for PB_DOUTCLR {}
#[doc = "Port Data Out Clear Register"]
pub mod pb_doutclr;
#[doc = "Port Data Out Toggle Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pb_douttgl](pb_douttgl) module"]
pub type PB_DOUTTGL = crate::Reg<u32, _PB_DOUTTGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB_DOUTTGL;
#[doc = "`write(|w| ..)` method takes [pb_douttgl::W](pb_douttgl::W) writer structure"]
impl crate::Writable for PB_DOUTTGL {}
#[doc = "Port Data Out Toggle Register"]
pub mod pb_douttgl;
#[doc = "Port Data In Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pb_din](pb_din) module"]
pub type PB_DIN = crate::Reg<u32, _PB_DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB_DIN;
#[doc = "`read()` method returns [pb_din::R](pb_din::R) reader structure"]
impl crate::Readable for PB_DIN {}
#[doc = "Port Data In Register"]
pub mod pb_din;
#[doc = "Port Unlocked Pins Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pb_pinlockn](pb_pinlockn) module"]
pub type PB_PINLOCKN = crate::Reg<u32, _PB_PINLOCKN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB_PINLOCKN;
#[doc = "`read()` method returns [pb_pinlockn::R](pb_pinlockn::R) reader structure"]
impl crate::Readable for PB_PINLOCKN {}
#[doc = "`write(|w| ..)` method takes [pb_pinlockn::W](pb_pinlockn::W) writer structure"]
impl crate::Writable for PB_PINLOCKN {}
#[doc = "Port Unlocked Pins Register"]
pub mod pb_pinlockn;
#[doc = "Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc_ctrl](pc_ctrl) module"]
pub type PC_CTRL = crate::Reg<u32, _PC_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC_CTRL;
#[doc = "`read()` method returns [pc_ctrl::R](pc_ctrl::R) reader structure"]
impl crate::Readable for PC_CTRL {}
#[doc = "`write(|w| ..)` method takes [pc_ctrl::W](pc_ctrl::W) writer structure"]
impl crate::Writable for PC_CTRL {}
#[doc = "Port Control Register"]
pub mod pc_ctrl;
#[doc = "Port Pin Mode Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc_model](pc_model) module"]
pub type PC_MODEL = crate::Reg<u32, _PC_MODEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC_MODEL;
#[doc = "`read()` method returns [pc_model::R](pc_model::R) reader structure"]
impl crate::Readable for PC_MODEL {}
#[doc = "`write(|w| ..)` method takes [pc_model::W](pc_model::W) writer structure"]
impl crate::Writable for PC_MODEL {}
#[doc = "Port Pin Mode Low Register"]
pub mod pc_model;
#[doc = "Port Pin Mode High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc_modeh](pc_modeh) module"]
pub type PC_MODEH = crate::Reg<u32, _PC_MODEH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC_MODEH;
#[doc = "`read()` method returns [pc_modeh::R](pc_modeh::R) reader structure"]
impl crate::Readable for PC_MODEH {}
#[doc = "`write(|w| ..)` method takes [pc_modeh::W](pc_modeh::W) writer structure"]
impl crate::Writable for PC_MODEH {}
#[doc = "Port Pin Mode High Register"]
pub mod pc_modeh;
#[doc = "Port Data Out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc_dout](pc_dout) module"]
pub type PC_DOUT = crate::Reg<u32, _PC_DOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC_DOUT;
#[doc = "`read()` method returns [pc_dout::R](pc_dout::R) reader structure"]
impl crate::Readable for PC_DOUT {}
#[doc = "`write(|w| ..)` method takes [pc_dout::W](pc_dout::W) writer structure"]
impl crate::Writable for PC_DOUT {}
#[doc = "Port Data Out Register"]
pub mod pc_dout;
#[doc = "Port Data Out Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc_doutset](pc_doutset) module"]
pub type PC_DOUTSET = crate::Reg<u32, _PC_DOUTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC_DOUTSET;
#[doc = "`write(|w| ..)` method takes [pc_doutset::W](pc_doutset::W) writer structure"]
impl crate::Writable for PC_DOUTSET {}
#[doc = "Port Data Out Set Register"]
pub mod pc_doutset;
#[doc = "Port Data Out Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc_doutclr](pc_doutclr) module"]
pub type PC_DOUTCLR = crate::Reg<u32, _PC_DOUTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC_DOUTCLR;
#[doc = "`write(|w| ..)` method takes [pc_doutclr::W](pc_doutclr::W) writer structure"]
impl crate::Writable for PC_DOUTCLR {}
#[doc = "Port Data Out Clear Register"]
pub mod pc_doutclr;
#[doc = "Port Data Out Toggle Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc_douttgl](pc_douttgl) module"]
pub type PC_DOUTTGL = crate::Reg<u32, _PC_DOUTTGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC_DOUTTGL;
#[doc = "`write(|w| ..)` method takes [pc_douttgl::W](pc_douttgl::W) writer structure"]
impl crate::Writable for PC_DOUTTGL {}
#[doc = "Port Data Out Toggle Register"]
pub mod pc_douttgl;
#[doc = "Port Data In Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc_din](pc_din) module"]
pub type PC_DIN = crate::Reg<u32, _PC_DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC_DIN;
#[doc = "`read()` method returns [pc_din::R](pc_din::R) reader structure"]
impl crate::Readable for PC_DIN {}
#[doc = "Port Data In Register"]
pub mod pc_din;
#[doc = "Port Unlocked Pins Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc_pinlockn](pc_pinlockn) module"]
pub type PC_PINLOCKN = crate::Reg<u32, _PC_PINLOCKN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC_PINLOCKN;
#[doc = "`read()` method returns [pc_pinlockn::R](pc_pinlockn::R) reader structure"]
impl crate::Readable for PC_PINLOCKN {}
#[doc = "`write(|w| ..)` method takes [pc_pinlockn::W](pc_pinlockn::W) writer structure"]
impl crate::Writable for PC_PINLOCKN {}
#[doc = "Port Unlocked Pins Register"]
pub mod pc_pinlockn;
#[doc = "Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd_ctrl](pd_ctrl) module"]
pub type PD_CTRL = crate::Reg<u32, _PD_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD_CTRL;
#[doc = "`read()` method returns [pd_ctrl::R](pd_ctrl::R) reader structure"]
impl crate::Readable for PD_CTRL {}
#[doc = "`write(|w| ..)` method takes [pd_ctrl::W](pd_ctrl::W) writer structure"]
impl crate::Writable for PD_CTRL {}
#[doc = "Port Control Register"]
pub mod pd_ctrl;
#[doc = "Port Pin Mode Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd_model](pd_model) module"]
pub type PD_MODEL = crate::Reg<u32, _PD_MODEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD_MODEL;
#[doc = "`read()` method returns [pd_model::R](pd_model::R) reader structure"]
impl crate::Readable for PD_MODEL {}
#[doc = "`write(|w| ..)` method takes [pd_model::W](pd_model::W) writer structure"]
impl crate::Writable for PD_MODEL {}
#[doc = "Port Pin Mode Low Register"]
pub mod pd_model;
#[doc = "Port Pin Mode High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd_modeh](pd_modeh) module"]
pub type PD_MODEH = crate::Reg<u32, _PD_MODEH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD_MODEH;
#[doc = "`read()` method returns [pd_modeh::R](pd_modeh::R) reader structure"]
impl crate::Readable for PD_MODEH {}
#[doc = "`write(|w| ..)` method takes [pd_modeh::W](pd_modeh::W) writer structure"]
impl crate::Writable for PD_MODEH {}
#[doc = "Port Pin Mode High Register"]
pub mod pd_modeh;
#[doc = "Port Data Out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd_dout](pd_dout) module"]
pub type PD_DOUT = crate::Reg<u32, _PD_DOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD_DOUT;
#[doc = "`read()` method returns [pd_dout::R](pd_dout::R) reader structure"]
impl crate::Readable for PD_DOUT {}
#[doc = "`write(|w| ..)` method takes [pd_dout::W](pd_dout::W) writer structure"]
impl crate::Writable for PD_DOUT {}
#[doc = "Port Data Out Register"]
pub mod pd_dout;
#[doc = "Port Data Out Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd_doutset](pd_doutset) module"]
pub type PD_DOUTSET = crate::Reg<u32, _PD_DOUTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD_DOUTSET;
#[doc = "`write(|w| ..)` method takes [pd_doutset::W](pd_doutset::W) writer structure"]
impl crate::Writable for PD_DOUTSET {}
#[doc = "Port Data Out Set Register"]
pub mod pd_doutset;
#[doc = "Port Data Out Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd_doutclr](pd_doutclr) module"]
pub type PD_DOUTCLR = crate::Reg<u32, _PD_DOUTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD_DOUTCLR;
#[doc = "`write(|w| ..)` method takes [pd_doutclr::W](pd_doutclr::W) writer structure"]
impl crate::Writable for PD_DOUTCLR {}
#[doc = "Port Data Out Clear Register"]
pub mod pd_doutclr;
#[doc = "Port Data Out Toggle Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd_douttgl](pd_douttgl) module"]
pub type PD_DOUTTGL = crate::Reg<u32, _PD_DOUTTGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD_DOUTTGL;
#[doc = "`write(|w| ..)` method takes [pd_douttgl::W](pd_douttgl::W) writer structure"]
impl crate::Writable for PD_DOUTTGL {}
#[doc = "Port Data Out Toggle Register"]
pub mod pd_douttgl;
#[doc = "Port Data In Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd_din](pd_din) module"]
pub type PD_DIN = crate::Reg<u32, _PD_DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD_DIN;
#[doc = "`read()` method returns [pd_din::R](pd_din::R) reader structure"]
impl crate::Readable for PD_DIN {}
#[doc = "Port Data In Register"]
pub mod pd_din;
#[doc = "Port Unlocked Pins Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd_pinlockn](pd_pinlockn) module"]
pub type PD_PINLOCKN = crate::Reg<u32, _PD_PINLOCKN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD_PINLOCKN;
#[doc = "`read()` method returns [pd_pinlockn::R](pd_pinlockn::R) reader structure"]
impl crate::Readable for PD_PINLOCKN {}
#[doc = "`write(|w| ..)` method takes [pd_pinlockn::W](pd_pinlockn::W) writer structure"]
impl crate::Writable for PD_PINLOCKN {}
#[doc = "Port Unlocked Pins Register"]
pub mod pd_pinlockn;
#[doc = "Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pe_ctrl](pe_ctrl) module"]
pub type PE_CTRL = crate::Reg<u32, _PE_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE_CTRL;
#[doc = "`read()` method returns [pe_ctrl::R](pe_ctrl::R) reader structure"]
impl crate::Readable for PE_CTRL {}
#[doc = "`write(|w| ..)` method takes [pe_ctrl::W](pe_ctrl::W) writer structure"]
impl crate::Writable for PE_CTRL {}
#[doc = "Port Control Register"]
pub mod pe_ctrl;
#[doc = "Port Pin Mode Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pe_model](pe_model) module"]
pub type PE_MODEL = crate::Reg<u32, _PE_MODEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE_MODEL;
#[doc = "`read()` method returns [pe_model::R](pe_model::R) reader structure"]
impl crate::Readable for PE_MODEL {}
#[doc = "`write(|w| ..)` method takes [pe_model::W](pe_model::W) writer structure"]
impl crate::Writable for PE_MODEL {}
#[doc = "Port Pin Mode Low Register"]
pub mod pe_model;
#[doc = "Port Pin Mode High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pe_modeh](pe_modeh) module"]
pub type PE_MODEH = crate::Reg<u32, _PE_MODEH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE_MODEH;
#[doc = "`read()` method returns [pe_modeh::R](pe_modeh::R) reader structure"]
impl crate::Readable for PE_MODEH {}
#[doc = "`write(|w| ..)` method takes [pe_modeh::W](pe_modeh::W) writer structure"]
impl crate::Writable for PE_MODEH {}
#[doc = "Port Pin Mode High Register"]
pub mod pe_modeh;
#[doc = "Port Data Out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pe_dout](pe_dout) module"]
pub type PE_DOUT = crate::Reg<u32, _PE_DOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE_DOUT;
#[doc = "`read()` method returns [pe_dout::R](pe_dout::R) reader structure"]
impl crate::Readable for PE_DOUT {}
#[doc = "`write(|w| ..)` method takes [pe_dout::W](pe_dout::W) writer structure"]
impl crate::Writable for PE_DOUT {}
#[doc = "Port Data Out Register"]
pub mod pe_dout;
#[doc = "Port Data Out Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pe_doutset](pe_doutset) module"]
pub type PE_DOUTSET = crate::Reg<u32, _PE_DOUTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE_DOUTSET;
#[doc = "`write(|w| ..)` method takes [pe_doutset::W](pe_doutset::W) writer structure"]
impl crate::Writable for PE_DOUTSET {}
#[doc = "Port Data Out Set Register"]
pub mod pe_doutset;
#[doc = "Port Data Out Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pe_doutclr](pe_doutclr) module"]
pub type PE_DOUTCLR = crate::Reg<u32, _PE_DOUTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE_DOUTCLR;
#[doc = "`write(|w| ..)` method takes [pe_doutclr::W](pe_doutclr::W) writer structure"]
impl crate::Writable for PE_DOUTCLR {}
#[doc = "Port Data Out Clear Register"]
pub mod pe_doutclr;
#[doc = "Port Data Out Toggle Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pe_douttgl](pe_douttgl) module"]
pub type PE_DOUTTGL = crate::Reg<u32, _PE_DOUTTGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE_DOUTTGL;
#[doc = "`write(|w| ..)` method takes [pe_douttgl::W](pe_douttgl::W) writer structure"]
impl crate::Writable for PE_DOUTTGL {}
#[doc = "Port Data Out Toggle Register"]
pub mod pe_douttgl;
#[doc = "Port Data In Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pe_din](pe_din) module"]
pub type PE_DIN = crate::Reg<u32, _PE_DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE_DIN;
#[doc = "`read()` method returns [pe_din::R](pe_din::R) reader structure"]
impl crate::Readable for PE_DIN {}
#[doc = "Port Data In Register"]
pub mod pe_din;
#[doc = "Port Unlocked Pins Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pe_pinlockn](pe_pinlockn) module"]
pub type PE_PINLOCKN = crate::Reg<u32, _PE_PINLOCKN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE_PINLOCKN;
#[doc = "`read()` method returns [pe_pinlockn::R](pe_pinlockn::R) reader structure"]
impl crate::Readable for PE_PINLOCKN {}
#[doc = "`write(|w| ..)` method takes [pe_pinlockn::W](pe_pinlockn::W) writer structure"]
impl crate::Writable for PE_PINLOCKN {}
#[doc = "Port Unlocked Pins Register"]
pub mod pe_pinlockn;
#[doc = "Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pf_ctrl](pf_ctrl) module"]
pub type PF_CTRL = crate::Reg<u32, _PF_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PF_CTRL;
#[doc = "`read()` method returns [pf_ctrl::R](pf_ctrl::R) reader structure"]
impl crate::Readable for PF_CTRL {}
#[doc = "`write(|w| ..)` method takes [pf_ctrl::W](pf_ctrl::W) writer structure"]
impl crate::Writable for PF_CTRL {}
#[doc = "Port Control Register"]
pub mod pf_ctrl;
#[doc = "Port Pin Mode Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pf_model](pf_model) module"]
pub type PF_MODEL = crate::Reg<u32, _PF_MODEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PF_MODEL;
#[doc = "`read()` method returns [pf_model::R](pf_model::R) reader structure"]
impl crate::Readable for PF_MODEL {}
#[doc = "`write(|w| ..)` method takes [pf_model::W](pf_model::W) writer structure"]
impl crate::Writable for PF_MODEL {}
#[doc = "Port Pin Mode Low Register"]
pub mod pf_model;
#[doc = "Port Pin Mode High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pf_modeh](pf_modeh) module"]
pub type PF_MODEH = crate::Reg<u32, _PF_MODEH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PF_MODEH;
#[doc = "`read()` method returns [pf_modeh::R](pf_modeh::R) reader structure"]
impl crate::Readable for PF_MODEH {}
#[doc = "`write(|w| ..)` method takes [pf_modeh::W](pf_modeh::W) writer structure"]
impl crate::Writable for PF_MODEH {}
#[doc = "Port Pin Mode High Register"]
pub mod pf_modeh;
#[doc = "Port Data Out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pf_dout](pf_dout) module"]
pub type PF_DOUT = crate::Reg<u32, _PF_DOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PF_DOUT;
#[doc = "`read()` method returns [pf_dout::R](pf_dout::R) reader structure"]
impl crate::Readable for PF_DOUT {}
#[doc = "`write(|w| ..)` method takes [pf_dout::W](pf_dout::W) writer structure"]
impl crate::Writable for PF_DOUT {}
#[doc = "Port Data Out Register"]
pub mod pf_dout;
#[doc = "Port Data Out Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pf_doutset](pf_doutset) module"]
pub type PF_DOUTSET = crate::Reg<u32, _PF_DOUTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PF_DOUTSET;
#[doc = "`write(|w| ..)` method takes [pf_doutset::W](pf_doutset::W) writer structure"]
impl crate::Writable for PF_DOUTSET {}
#[doc = "Port Data Out Set Register"]
pub mod pf_doutset;
#[doc = "Port Data Out Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pf_doutclr](pf_doutclr) module"]
pub type PF_DOUTCLR = crate::Reg<u32, _PF_DOUTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PF_DOUTCLR;
#[doc = "`write(|w| ..)` method takes [pf_doutclr::W](pf_doutclr::W) writer structure"]
impl crate::Writable for PF_DOUTCLR {}
#[doc = "Port Data Out Clear Register"]
pub mod pf_doutclr;
#[doc = "Port Data Out Toggle Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pf_douttgl](pf_douttgl) module"]
pub type PF_DOUTTGL = crate::Reg<u32, _PF_DOUTTGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PF_DOUTTGL;
#[doc = "`write(|w| ..)` method takes [pf_douttgl::W](pf_douttgl::W) writer structure"]
impl crate::Writable for PF_DOUTTGL {}
#[doc = "Port Data Out Toggle Register"]
pub mod pf_douttgl;
#[doc = "Port Data In Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pf_din](pf_din) module"]
pub type PF_DIN = crate::Reg<u32, _PF_DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PF_DIN;
#[doc = "`read()` method returns [pf_din::R](pf_din::R) reader structure"]
impl crate::Readable for PF_DIN {}
#[doc = "Port Data In Register"]
pub mod pf_din;
#[doc = "Port Unlocked Pins Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pf_pinlockn](pf_pinlockn) module"]
pub type PF_PINLOCKN = crate::Reg<u32, _PF_PINLOCKN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PF_PINLOCKN;
#[doc = "`read()` method returns [pf_pinlockn::R](pf_pinlockn::R) reader structure"]
impl crate::Readable for PF_PINLOCKN {}
#[doc = "`write(|w| ..)` method takes [pf_pinlockn::W](pf_pinlockn::W) writer structure"]
impl crate::Writable for PF_PINLOCKN {}
#[doc = "Port Unlocked Pins Register"]
pub mod pf_pinlockn;
#[doc = "External Interrupt Port Select Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [extipsell](extipsell) module"]
pub type EXTIPSELL = crate::Reg<u32, _EXTIPSELL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTIPSELL;
#[doc = "`read()` method returns [extipsell::R](extipsell::R) reader structure"]
impl crate::Readable for EXTIPSELL {}
#[doc = "`write(|w| ..)` method takes [extipsell::W](extipsell::W) writer structure"]
impl crate::Writable for EXTIPSELL {}
#[doc = "External Interrupt Port Select Low Register"]
pub mod extipsell;
#[doc = "External Interrupt Port Select High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [extipselh](extipselh) module"]
pub type EXTIPSELH = crate::Reg<u32, _EXTIPSELH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTIPSELH;
#[doc = "`read()` method returns [extipselh::R](extipselh::R) reader structure"]
impl crate::Readable for EXTIPSELH {}
#[doc = "`write(|w| ..)` method takes [extipselh::W](extipselh::W) writer structure"]
impl crate::Writable for EXTIPSELH {}
#[doc = "External Interrupt Port Select High Register"]
pub mod extipselh;
#[doc = "External Interrupt Rising Edge Trigger Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [extirise](extirise) module"]
pub type EXTIRISE = crate::Reg<u32, _EXTIRISE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTIRISE;
#[doc = "`read()` method returns [extirise::R](extirise::R) reader structure"]
impl crate::Readable for EXTIRISE {}
#[doc = "`write(|w| ..)` method takes [extirise::W](extirise::W) writer structure"]
impl crate::Writable for EXTIRISE {}
#[doc = "External Interrupt Rising Edge Trigger Register"]
pub mod extirise;
#[doc = "External Interrupt Falling Edge Trigger Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [extifall](extifall) module"]
pub type EXTIFALL = crate::Reg<u32, _EXTIFALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTIFALL;
#[doc = "`read()` method returns [extifall::R](extifall::R) reader structure"]
impl crate::Readable for EXTIFALL {}
#[doc = "`write(|w| ..)` method takes [extifall::W](extifall::W) writer structure"]
impl crate::Writable for EXTIFALL {}
#[doc = "External Interrupt Falling Edge Trigger Register"]
pub mod extifall;
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
#[doc = "Input Sense Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [insense](insense) module"]
pub type INSENSE = crate::Reg<u32, _INSENSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSENSE;
#[doc = "`read()` method returns [insense::R](insense::R) reader structure"]
impl crate::Readable for INSENSE {}
#[doc = "`write(|w| ..)` method takes [insense::W](insense::W) writer structure"]
impl crate::Writable for INSENSE {}
#[doc = "Input Sense Register"]
pub mod insense;
#[doc = "Configuration Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lock](lock) module"]
pub type LOCK = crate::Reg<u32, _LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCK;
#[doc = "`read()` method returns [lock::R](lock::R) reader structure"]
impl crate::Readable for LOCK {}
#[doc = "`write(|w| ..)` method takes [lock::W](lock::W) writer structure"]
impl crate::Writable for LOCK {}
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "GPIO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "GPIO Control Register"]
pub mod ctrl;
#[doc = "GPIO Command Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "GPIO Command Register"]
pub mod cmd;
#[doc = "EM4 Wake-up Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [em4wuen](em4wuen) module"]
pub type EM4WUEN = crate::Reg<u32, _EM4WUEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EM4WUEN;
#[doc = "`read()` method returns [em4wuen::R](em4wuen::R) reader structure"]
impl crate::Readable for EM4WUEN {}
#[doc = "`write(|w| ..)` method takes [em4wuen::W](em4wuen::W) writer structure"]
impl crate::Writable for EM4WUEN {}
#[doc = "EM4 Wake-up Enable Register"]
pub mod em4wuen;
#[doc = "EM4 Wake-up Polarity Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [em4wupol](em4wupol) module"]
pub type EM4WUPOL = crate::Reg<u32, _EM4WUPOL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EM4WUPOL;
#[doc = "`read()` method returns [em4wupol::R](em4wupol::R) reader structure"]
impl crate::Readable for EM4WUPOL {}
#[doc = "`write(|w| ..)` method takes [em4wupol::W](em4wupol::W) writer structure"]
impl crate::Writable for EM4WUPOL {}
#[doc = "EM4 Wake-up Polarity Register"]
pub mod em4wupol;
#[doc = "EM4 Wake-up Cause Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [em4wucause](em4wucause) module"]
pub type EM4WUCAUSE = crate::Reg<u32, _EM4WUCAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EM4WUCAUSE;
#[doc = "`read()` method returns [em4wucause::R](em4wucause::R) reader structure"]
impl crate::Readable for EM4WUCAUSE {}
#[doc = "EM4 Wake-up Cause Register"]
pub mod em4wucause;
