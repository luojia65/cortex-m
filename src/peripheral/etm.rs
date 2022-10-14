//! Embedded Trace Macrocell

use volatile_register::{RO, RW, WO};

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// Main Control Register
    pub cr: RW<u32>,
    /// Configuration Code Register
    pub ccr: RO<u32>,
    /// Trigger Event
    pub trigger: WO<u32>,
    /// ASIC Control Register
    pub asiccr: WO<u32>,
    /// ETM Status Register
    pub sr: RW<u32>,
    /// System Configuration Register
    pub scr: RO<u32>,
    /// TraceEnable Start/Stop Control Register
    pub tsscr: WO<u32>,
    /// TraceEnable Control 2
    pub tecr2: WO<u32>,
    /// TraceEnable Event Register
    pub teevr: WO<u32>,
    /// TraceEnable Control 1
    pub tecr1: WO<u32>,
    /// FIFOFULL Region Register
    pub ffrr: WO<u32>,
    /// FIFOFULL Level Register
    pub fflr: RW<u32>,
    /// ViewData Event Register
    pub vdevr: WO<u32>,
    /// ViewData Control 1
    pub vdcr1: WO<u32>,
    /// ViewData Control 2
    pub vdcr2: WO<u32>,
    /// ViewData Control 3
    pub vdcr3: WO<u32>,
    /// Address Comparator Value Registers
    pub acvr: [WO<u32>; 16],
    /// Address Comparator Access Type Registers
    pub actr: [WO<u32>; 16],
    /// Data Comparator Value Registers
    pub dcvr: [WO<u32>; 16],
    /// Data Comparator Mask Registers
    pub dcmr: [WO<u32>; 16],
    /// Counter Reload Value Registers
    pub cntrldvr: [WO<u32>; 4],
    /// Counter Enable Registers
    pub cntenr: [WO<u32>; 4],
    /// Counter Reload Event Registers
    pub cntrldevr: [WO<u32>; 4],
    /// Counter Value Registers
    pub cntvr: [RW<u32>; 4],
    /// Sequencer State Transition Event Registers
    pub sqabevr: [WO<u32>; 6],
    _reserved0: u32,
    /// Current Sequencer State Register
    pub sqr: RW<u32>,
    /// External Output Event Registers
    pub extoutevr: [WO<u32>; 4],
    /// Context ID Comparator Value Registers
    pub cidcvr: [WO<u32>; 3],
    /// Context ID Comparator Mask Register
    pub cidcmr: WO<u32>,
    /// Implementation specific registers
    pub impl_: [WO<u32>; 8],
    /// Synchronization Frequency Register
    pub syncfr: WO<u32>,
    /// ETM ID Register
    pub idr: RO<u32>,
    /// Configuration Code Extension Register
    pub ccer: WO<u32>,
    /// Extended External Input Selection Register
    pub extinselr: WO<u32>,
    /// TraceEnable Start/Stop EmbeddedICE Control Register
    pub tesseicr: WO<u32>,
    /// EmbeddedICE Behavior Control Register
    pub eibcr: WO<u32>,
    /// Timestamp Event Register, ETMv3.5
    pub tsevr: RW<u32>,
    /// Auxiliary Control Register, ETMv3.5
    pub auxcr: RW<u32>,
    /// CoreSight Trace ID Register
    pub traceidr: RW<u32>,
    _reserved1: u32,
    /// ETM ID Register 2
    pub idr2: RO<u32>,
    _reserved2: [u32; 13],
    /// VMID Comparator Value Register, ETMv3.5
    pub vmidcvr: RW<u32>,
    _reserved3: [u32; 47],
    /// OS Lock Access Register
    pub oslar: WO<u32>,
    /// OS Lock Status Register
    pub oslsr: RO<u32>,
    /// OS Save and Restore Register
    pub ossrr: RW<u32>,
    _reserved4: u32,
    /// Power Down Control Register, ETMv3.5
    pub pdcr: RW<u32>,
    /// Device Power-Down Status Register
    pub pdsr: RW<u32>,
    _reserved5: [u32; 762],
    /// Integration Mode Control Register
    pub itctrl: RW<u32>,
    _reserved6: [u32; 39],
    /// Claim Tag Set Register
    pub claimset: RW<u32>,
    /// Claim Tag Clear Register
    pub claimclr: RW<u32>,
    _reserved7: [u32; 2],
    /// Lock Access Register
    pub lar: WO<u32>,
    /// Lock Status Register
    pub lsr: RO<u32>,
    /// Authentication Status Register
    pub authstatus: RO<u32>,
    reserved8: [u32; 3],
    /// CoreSight Device Configuration Register
    pub devid: RO<u32>,
    /// CoreSight Device Type Register
    pub devtype: RO<u32>,
    /// Peripheral ID4
    pub pidr4: RO<u32>,
    /// Peripheral ID5
    pub pidr5: RO<u32>,
    /// Peripheral ID6
    pub pidr6: RO<u32>,
    /// Peripheral ID7
    pub pidr7: RO<u32>,
    /// Peripheral ID0
    pub pidr0: RO<u32>,
    /// Peripheral ID1
    pub pidr1: RO<u32>,
    /// Peripheral ID2
    pub pidr2: RO<u32>,
    /// Peripheral ID3
    pub pidr3: RO<u32>,
    /// Component ID0
    pub cidr0: RO<u32>,
    /// Component ID1
    pub cidr1: RO<u32>,
    /// Component ID2
    pub cidr2: RO<u32>,
    /// Component ID3
    pub cidr3: RO<u32>,
}
