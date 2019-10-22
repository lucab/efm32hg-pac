#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Calibration temperature and checksum"]
    pub cal: CAL,
    _reserved1: [u8; 28usize],
    #[doc = "0x20 - ADC0 Calibration register 0"]
    pub adc0cal0: ADC0CAL0,
    _reserved2: [u8; 28usize],
    #[doc = "0x40 - ADC0 Calibration register 1"]
    pub adc0cal1: ADC0CAL1,
    _reserved3: [u8; 28usize],
    #[doc = "0x60 - ADC0 Calibration register 2"]
    pub adc0cal2: ADC0CAL2,
    _reserved4: [u8; 92usize],
    #[doc = "0xc0 - IDAC0 calibration register"]
    pub idac0cal0: IDAC0CAL0,
    _reserved5: [u8; 28usize],
    #[doc = "0xe0 - USHFRCO calibration register"]
    pub ushfrcocal0: USHFRCOCAL0,
    _reserved6: [u8; 60usize],
    #[doc = "0x120 - AUXHFRCO calibration register 0"]
    pub auxhfrcocal0: AUXHFRCOCAL0,
    _reserved7: [u8; 28usize],
    #[doc = "0x140 - AUXHFRCO calibration register 1"]
    pub auxhfrcocal1: AUXHFRCOCAL1,
    _reserved8: [u8; 28usize],
    #[doc = "0x160 - HFRCO calibration register 0"]
    pub hfrcocal0: HFRCOCAL0,
    _reserved9: [u8; 28usize],
    #[doc = "0x180 - HFRCO calibration register 1"]
    pub hfrcocal1: HFRCOCAL1,
    _reserved10: [u8; 28usize],
    #[doc = "0x1a0 - Memory information"]
    pub meminfo: MEMINFO,
    _reserved11: [u8; 60usize],
    #[doc = "0x1e0 - Reserved"]
    pub _reserved: _RESERVED,
    _reserved12: [u8; 28usize],
    #[doc = "0x200 - Low 32 bits of device unique number"]
    pub uniquel: UNIQUEL,
    _reserved13: [u8; 28usize],
    #[doc = "0x220 - High 32 bits of device unique number"]
    pub uniqueh: UNIQUEH,
    _reserved14: [u8; 28usize],
    #[doc = "0x240 - Flash and SRAM Memory size in KiloBytes"]
    pub msize: MSIZE,
    _reserved15: [u8; 28usize],
    #[doc = "0x260 - Part description"]
    pub part: PART,
}
#[doc = "Calibration temperature and checksum\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cal](cal) module"]
pub type CAL = crate::Reg<u32, _CAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAL;
#[doc = "`read()` method returns [cal::R](cal::R) reader structure"]
impl crate::Readable for CAL {}
#[doc = "Calibration temperature and checksum"]
pub mod cal;
#[doc = "ADC0 Calibration register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adc0cal0](adc0cal0) module"]
pub type ADC0CAL0 = crate::Reg<u32, _ADC0CAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC0CAL0;
#[doc = "`read()` method returns [adc0cal0::R](adc0cal0::R) reader structure"]
impl crate::Readable for ADC0CAL0 {}
#[doc = "ADC0 Calibration register 0"]
pub mod adc0cal0;
#[doc = "ADC0 Calibration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adc0cal1](adc0cal1) module"]
pub type ADC0CAL1 = crate::Reg<u32, _ADC0CAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC0CAL1;
#[doc = "`read()` method returns [adc0cal1::R](adc0cal1::R) reader structure"]
impl crate::Readable for ADC0CAL1 {}
#[doc = "ADC0 Calibration register 1"]
pub mod adc0cal1;
#[doc = "ADC0 Calibration register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adc0cal2](adc0cal2) module"]
pub type ADC0CAL2 = crate::Reg<u32, _ADC0CAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC0CAL2;
#[doc = "`read()` method returns [adc0cal2::R](adc0cal2::R) reader structure"]
impl crate::Readable for ADC0CAL2 {}
#[doc = "ADC0 Calibration register 2"]
pub mod adc0cal2;
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_reserved](_reserved) module"]
pub type _RESERVED = crate::Reg<u32, __RESERVED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __RESERVED;
#[doc = "`read()` method returns [_reserved::R](_reserved::R) reader structure"]
impl crate::Readable for _RESERVED {}
#[doc = "Reserved"]
pub mod _reserved;
#[doc = "IDAC0 calibration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [idac0cal0](idac0cal0) module"]
pub type IDAC0CAL0 = crate::Reg<u32, _IDAC0CAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDAC0CAL0;
#[doc = "`read()` method returns [idac0cal0::R](idac0cal0::R) reader structure"]
impl crate::Readable for IDAC0CAL0 {}
#[doc = "IDAC0 calibration register"]
pub mod idac0cal0;
#[doc = "USHFRCO calibration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ushfrcocal0](ushfrcocal0) module"]
pub type USHFRCOCAL0 = crate::Reg<u32, _USHFRCOCAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USHFRCOCAL0;
#[doc = "`read()` method returns [ushfrcocal0::R](ushfrcocal0::R) reader structure"]
impl crate::Readable for USHFRCOCAL0 {}
#[doc = "USHFRCO calibration register"]
pub mod ushfrcocal0;
#[doc = "AUXHFRCO calibration register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [auxhfrcocal0](auxhfrcocal0) module"]
pub type AUXHFRCOCAL0 = crate::Reg<u32, _AUXHFRCOCAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUXHFRCOCAL0;
#[doc = "`read()` method returns [auxhfrcocal0::R](auxhfrcocal0::R) reader structure"]
impl crate::Readable for AUXHFRCOCAL0 {}
#[doc = "AUXHFRCO calibration register 0"]
pub mod auxhfrcocal0;
#[doc = "AUXHFRCO calibration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [auxhfrcocal1](auxhfrcocal1) module"]
pub type AUXHFRCOCAL1 = crate::Reg<u32, _AUXHFRCOCAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUXHFRCOCAL1;
#[doc = "`read()` method returns [auxhfrcocal1::R](auxhfrcocal1::R) reader structure"]
impl crate::Readable for AUXHFRCOCAL1 {}
#[doc = "AUXHFRCO calibration register 1"]
pub mod auxhfrcocal1;
#[doc = "HFRCO calibration register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfrcocal0](hfrcocal0) module"]
pub type HFRCOCAL0 = crate::Reg<u32, _HFRCOCAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFRCOCAL0;
#[doc = "`read()` method returns [hfrcocal0::R](hfrcocal0::R) reader structure"]
impl crate::Readable for HFRCOCAL0 {}
#[doc = "HFRCO calibration register 0"]
pub mod hfrcocal0;
#[doc = "HFRCO calibration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfrcocal1](hfrcocal1) module"]
pub type HFRCOCAL1 = crate::Reg<u32, _HFRCOCAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFRCOCAL1;
#[doc = "`read()` method returns [hfrcocal1::R](hfrcocal1::R) reader structure"]
impl crate::Readable for HFRCOCAL1 {}
#[doc = "HFRCO calibration register 1"]
pub mod hfrcocal1;
#[doc = "Memory information\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [meminfo](meminfo) module"]
pub type MEMINFO = crate::Reg<u32, _MEMINFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEMINFO;
#[doc = "`read()` method returns [meminfo::R](meminfo::R) reader structure"]
impl crate::Readable for MEMINFO {}
#[doc = "Memory information"]
pub mod meminfo;
#[doc = "Low 32 bits of device unique number\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uniquel](uniquel) module"]
pub type UNIQUEL = crate::Reg<u32, _UNIQUEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UNIQUEL;
#[doc = "`read()` method returns [uniquel::R](uniquel::R) reader structure"]
impl crate::Readable for UNIQUEL {}
#[doc = "Low 32 bits of device unique number"]
pub mod uniquel;
#[doc = "High 32 bits of device unique number\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uniqueh](uniqueh) module"]
pub type UNIQUEH = crate::Reg<u32, _UNIQUEH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UNIQUEH;
#[doc = "`read()` method returns [uniqueh::R](uniqueh::R) reader structure"]
impl crate::Readable for UNIQUEH {}
#[doc = "High 32 bits of device unique number"]
pub mod uniqueh;
#[doc = "Flash and SRAM Memory size in KiloBytes\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [msize](msize) module"]
pub type MSIZE = crate::Reg<u32, _MSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSIZE;
#[doc = "`read()` method returns [msize::R](msize::R) reader structure"]
impl crate::Readable for MSIZE {}
#[doc = "Flash and SRAM Memory size in KiloBytes"]
pub mod msize;
#[doc = "Part description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [part](part) module"]
pub type PART = crate::Reg<u32, _PART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PART;
#[doc = "`read()` method returns [part::R](part::R) reader structure"]
impl crate::Readable for PART {}
#[doc = "Part description"]
pub mod part;
