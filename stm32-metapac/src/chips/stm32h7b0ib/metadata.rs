include!("../metadata_0464.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32H7B0IB",
    family: "STM32H7",
    line: "STM32H7B0 Value line",
    memory: &[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 0x8000000,
            size: 131072,
            settings: Some(FlashSettings {
                erase_size: 8192,
                write_size: 32,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "OTP",
            kind: MemoryRegionKind::Flash,
            address: 0x8fff000,
            size: 1024,
            settings: Some(FlashSettings {
                erase_size: 1024,
                write_size: 32,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x24000000,
            size: 1048576,
            settings: None,
        },
    ],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
