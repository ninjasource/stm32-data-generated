
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rcc",
            extends: None,
            description: Some(
                "Reset and clock control",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "Clock control register",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "icscr",
                    description: Some(
                        "Internal clock sources calibration register",
                    ),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Icscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr",
                    description: Some(
                        "Clock configuration register",
                    ),
                    array: None,
                    byte_offset: 8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pllcfgr",
                    description: Some(
                        "PLL configuration register",
                    ),
                    array: None,
                    byte_offset: 12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pllcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cier",
                    description: Some(
                        "Clock interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cifr",
                    description: Some(
                        "Clock interrupt flag register",
                    ),
                    array: None,
                    byte_offset: 28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Cifr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cicr",
                    description: Some(
                        "Clock interrupt clear register",
                    ),
                    array: None,
                    byte_offset: 32,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Cicr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1rstr",
                    description: Some(
                        "AHB1 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb1rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb2rstr",
                    description: Some(
                        "AHB2 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb2rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb3rstr",
                    description: Some(
                        "AHB3 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb3rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1rstr1",
                    description: Some(
                        "APB1 peripheral reset register 1",
                    ),
                    array: None,
                    byte_offset: 56,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1rstr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1rstr2",
                    description: Some(
                        "APB1 peripheral reset register 2",
                    ),
                    array: None,
                    byte_offset: 60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1rstr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2rstr",
                    description: Some(
                        "APB2 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb3rstr",
                    description: Some(
                        "APB3 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb3rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1enr",
                    description: Some(
                        "AHB1 peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 72,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb1enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb2enr",
                    description: Some(
                        "AHB2 peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 76,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb2enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb3enr",
                    description: Some(
                        "AHB3 peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb3enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1enr1",
                    description: Some(
                        "APB1 peripheral clock enable register 1",
                    ),
                    array: None,
                    byte_offset: 88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1enr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1enr2",
                    description: Some(
                        "APB1 peripheral clock enable register 2",
                    ),
                    array: None,
                    byte_offset: 92,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1enr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2enr",
                    description: Some(
                        "APB2 peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 96,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb3enr",
                    description: Some(
                        "APB3 peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb3enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1smenr",
                    description: Some(
                        "AHB1 peripheral clocks enable in Sleep modes register",
                    ),
                    array: None,
                    byte_offset: 104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb1smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb2smenr",
                    description: Some(
                        "AHB2 peripheral clocks enable in Sleep modes register",
                    ),
                    array: None,
                    byte_offset: 108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb2smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb3smenr",
                    description: Some(
                        "AHB3 peripheral clocks enable in Sleep and Stop modes register",
                    ),
                    array: None,
                    byte_offset: 112,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb3smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1smenr1",
                    description: Some(
                        "APB1 peripheral clocks enable in Sleep mode register 1",
                    ),
                    array: None,
                    byte_offset: 120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1smenr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1smenr2",
                    description: Some(
                        "APB1 peripheral clocks enable in Sleep mode register 2",
                    ),
                    array: None,
                    byte_offset: 124,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1smenr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2smenr",
                    description: Some(
                        "APB2 peripheral clocks enable in Sleep mode register",
                    ),
                    array: None,
                    byte_offset: 128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb3smenr",
                    description: Some(
                        "APB3 peripheral clock enable in Sleep mode register",
                    ),
                    array: None,
                    byte_offset: 132,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb3smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccipr",
                    description: Some(
                        "Peripherals independent clock configuration register",
                    ),
                    array: None,
                    byte_offset: 136,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccipr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdcr",
                    description: Some(
                        "Backup domain control register",
                    ),
                    array: None,
                    byte_offset: 144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bdcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "csr",
                    description: Some(
                        "Control/status register",
                    ),
                    array: None,
                    byte_offset: 148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Csr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "extcfgr",
                    description: Some(
                        "Extended clock recovery register",
                    ),
                    array: None,
                    byte_offset: 264,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Extcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "c2ahb1enr",
                    description: Some(
                        "CPU2 AHB1 peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 328,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "C2ahb1enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "c2ahb2enr",
                    description: Some(
                        "CPU2 AHB2 peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 332,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "C2ahb2enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "c2ahb3enr",
                    description: Some(
                        "CPU2 AHB3 peripheral clock enable register [dual core device only]",
                    ),
                    array: None,
                    byte_offset: 336,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "C2ahb3enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "c2apb1enr1",
                    description: Some(
                        "CPU2 APB1 peripheral clock enable register 1 [dual core device only]",
                    ),
                    array: None,
                    byte_offset: 344,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "C2apb1enr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "c2apb1enr2",
                    description: Some(
                        "CPU2 APB1 peripheral clock enable register 2 [dual core device only]",
                    ),
                    array: None,
                    byte_offset: 348,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "C2apb1enr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "c2apb2enr",
                    description: Some(
                        "CPU2 APB2 peripheral clock enable register [dual core device only]",
                    ),
                    array: None,
                    byte_offset: 352,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "C2apb2enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "c2apb3enr",
                    description: Some(
                        "CPU2 APB3 peripheral clock enable register [dual core device only]",
                    ),
                    array: None,
                    byte_offset: 356,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "C2apb3enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "c2ahb1smenr",
                    description: Some(
                        "CPU2 AHB1 peripheral clocks enable in Sleep modes register [dual core device only]",
                    ),
                    array: None,
                    byte_offset: 360,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "C2ahb1smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "c2ahb2smenr",
                    description: Some(
                        "CPU2 AHB2 peripheral clocks enable in Sleep modes register [dual core device only]",
                    ),
                    array: None,
                    byte_offset: 364,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "C2ahb2smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "c2ahb3smenr",
                    description: Some(
                        "CPU2 AHB3 peripheral clocks enable in Sleep mode register [dual core device only]",
                    ),
                    array: None,
                    byte_offset: 368,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "C2ahb3smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "c2apb1smenr1",
                    description: Some(
                        "CPU2 APB1 peripheral clocks enable in Sleep mode register 1 [dual core device only]",
                    ),
                    array: None,
                    byte_offset: 376,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "C2apb1smenr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "c2apb1smenr2",
                    description: Some(
                        "CPU2 APB1 peripheral clocks enable in Sleep mode register 2 [dual core device only]",
                    ),
                    array: None,
                    byte_offset: 380,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "C2apb1smenr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "c2apb2smenr",
                    description: Some(
                        "CPU2 APB2 peripheral clocks enable in Sleep mode register [dual core device only]",
                    ),
                    array: None,
                    byte_offset: 384,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "C2apb2smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "c2apb3smenr",
                    description: Some(
                        "CPU2 APB3 peripheral clock enable in Sleep mode register [dual core device only]",
                    ),
                    array: None,
                    byte_offset: 388,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "C2apb3smenr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "C2ahb3enr",
            extends: None,
            description: Some(
                "CPU2 AHB3 peripheral clock enable register [dual core device only]",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pkaen",
                    description: Some(
                        "CPU2 PKA accelerator clock enable",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aesen",
                    description: Some(
                        "CPU2 AES accelerator clock enable",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rngen",
                    description: Some(
                        "CPU2 True RNG clocks enable",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsemen",
                    description: Some(
                        "CPU2 HSEM clock enable",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ipccen",
                    description: Some(
                        "CPU2 IPCC interface clock enable",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flashen",
                    description: Some(
                        "CPU2 Flash interface clock enable",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Extcfgr",
            extends: None,
            description: Some(
                "Extended clock recovery register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "shdhpre",
                    description: Some(
                        "HCLK3 shared prescaler (AHB3, Flash, and SRAM2)",
                    ),
                    bit_offset: 0,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "c2hpre",
                    description: Some(
                        "[dual core device only] HCLK2 prescaler (CPU2)",
                    ),
                    bit_offset: 4,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "shdhpref",
                    description: Some(
                        "HCLK3 shared prescaler flag (AHB3, Flash, and SRAM2)",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "c2hpref",
                    description: Some(
                        "CLK2 prescaler flag (CPU2)",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb2rstr",
            extends: None,
            description: Some(
                "APB2 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adcrst",
                    description: Some(
                        "ADC reset",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim1rst",
                    description: Some(
                        "TIM1 timer reset",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi1rst",
                    description: Some(
                        "SPI1 reset",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart1rst",
                    description: Some(
                        "USART1 reset",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim16rst",
                    description: Some(
                        "TIM16 timer reset",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim17rst",
                    description: Some(
                        "TIM17 timer reset",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "C2apb3smenr",
            extends: None,
            description: Some(
                "CPU2 APB3 peripheral clock enable in Sleep mode register [dual core device only]",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "subghzspismen",
                    description: Some(
                        "sub-GHz radio SPI clock enable during CPU2 CSleep and CStop modes",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "C2ahb1enr",
            extends: None,
            description: Some(
                "CPU2 AHB1 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma1en",
                    description: Some(
                        "CPU2 DMA1 clock enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dma2en",
                    description: Some(
                        "CPU2 DMA2 clock enable",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmamux1en",
                    description: Some(
                        "CPU2 DMAMUX1 clock enable",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crcen",
                    description: Some(
                        "CPU2 CRC clock enable",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb2smenr",
            extends: None,
            description: Some(
                "AHB2 peripheral clocks enable in Sleep modes register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioasmen",
                    description: Some(
                        "IO port A clock enable during CPU1 CSleep mode.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiobsmen",
                    description: Some(
                        "IO port B clock enable during CPU1 CSleep mode.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiocsmen",
                    description: Some(
                        "IO port C clock enable during CPU1 CSleep mode.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiohsmen",
                    description: Some(
                        "IO port H clock enable during CPU1 CSleep mode.",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1smenr1",
            extends: None,
            description: Some(
                "APB1 peripheral clocks enable in Sleep mode register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2smen",
                    description: Some(
                        "TIM2 timer clock enable during CPU1 CSleep mode.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtcapbsmen",
                    description: Some(
                        "RTC bus clock enable during CPU1 CSleep mode.",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wwdgsmen",
                    description: Some(
                        "Window watchdog clocks enable during CPU1 CSleep mode.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi2smen",
                    description: Some(
                        "SPI2 clock enable during CPU1 CSleep mode.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart2smen",
                    description: Some(
                        "USART2 clock enable during CPU1 CSleep mode.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1smen",
                    description: Some(
                        "I2C1 clock enable during CPU1 Csleep and CStop modes",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2smen",
                    description: Some(
                        "I2C2 clock enable during CPU1 Csleep and CStop modes",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c3smen",
                    description: Some(
                        "I2C3 clock enable during CPU1 Csleep and CStop modes",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dacsmen",
                    description: Some(
                        "DAC1 clock enable during CPU1 CSleep mode.",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim1smen",
                    description: Some(
                        "Low power timer 1 clock enable during CPU1 Csleep and CStop mode",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cifr",
            extends: None,
            description: Some(
                "Clock interrupt flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyf",
                    description: Some(
                        "LSI ready interrupt flag",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lserdyf",
                    description: Some(
                        "LSE ready interrupt flag",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msirdyf",
                    description: Some(
                        "MSI ready interrupt flag",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsirdyf",
                    description: Some(
                        "HSI16 ready interrupt flag",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hserdyf",
                    description: Some(
                        "HSE32 ready interrupt flag",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllrdyf",
                    description: Some(
                        "PLL ready interrupt flag",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cssf",
                    description: Some(
                        "HSE32 Clock security system interrupt flag",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsecssf",
                    description: Some(
                        "LSE Clock security system interrupt flag",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb2enr",
            extends: None,
            description: Some(
                "APB2 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adcen",
                    description: Some(
                        "CPU1 ADC clocks enable",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim1en",
                    description: Some(
                        "CPU1 TIM1 timer clock enable",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi1en",
                    description: Some(
                        "CPU1 SPI1 clock enable",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart1en",
                    description: Some(
                        "CPU1 USART1clocks enable",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim16en",
                    description: Some(
                        "CPU1 TIM16 timer clock enable",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim17en",
                    description: Some(
                        "CPU1 TIM17 timer clock enable",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1rstr1",
            extends: None,
            description: Some(
                "APB1 peripheral reset register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2rst",
                    description: Some(
                        "TIM2 timer reset",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi2rst",
                    description: Some(
                        "SPI2 reset",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart2rst",
                    description: Some(
                        "USART2 reset",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1rst",
                    description: Some(
                        "I2C1 reset",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2rst",
                    description: Some(
                        "I2C2 reset",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c3rst",
                    description: Some(
                        "I2C3 reset",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dacrst",
                    description: Some(
                        "DAC1 reset",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim1rst",
                    description: Some(
                        "Low Power Timer 1 reset",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cicr",
            extends: None,
            description: Some(
                "Clock interrupt clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyc",
                    description: Some(
                        "LSI ready interrupt clear",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lserdyc",
                    description: Some(
                        "LSE ready interrupt clear",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msirdyc",
                    description: Some(
                        "MSI ready interrupt clear",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsirdyc",
                    description: Some(
                        "HSI16 ready interrupt clear",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hserdyc",
                    description: Some(
                        "HSE32 ready interrupt clear",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllrdyc",
                    description: Some(
                        "PLL ready interrupt clear",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cssc",
                    description: Some(
                        "HSE32 Clock security system interrupt clear",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsecssc",
                    description: Some(
                        "LSE Clock security system interrupt clear",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some(
                "Control/status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsion",
                    description: Some(
                        "LSI oscillator enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsirdy",
                    description: Some(
                        "LSI oscillator ready",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsipre",
                    description: Some(
                        "LSI frequency prescaler",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msisrange",
                    description: Some(
                        "MSI clock ranges",
                    ),
                    bit_offset: 8,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rfrstf",
                    description: Some(
                        "Radio in reset status flag",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rfrst",
                    description: Some(
                        "Radio reset",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rmvf",
                    description: Some(
                        "Remove reset flag",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rfilarstf",
                    description: Some(
                        "Radio illegal access flag",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "oblrstf",
                    description: Some(
                        "Option byte loader reset flag",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pinrstf",
                    description: Some(
                        "Pin reset flag",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "borrstf",
                    description: Some(
                        "BOR flag",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sftrstf",
                    description: Some(
                        "Software reset flag",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iwdgrstf",
                    description: Some(
                        "Independent window watchdog reset flag",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wwdgrstf",
                    description: Some(
                        "Window watchdog reset flag",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpwrrstf",
                    description: Some(
                        "Low-power reset flag",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "C2apb1enr1",
            extends: None,
            description: Some(
                "CPU2 APB1 peripheral clock enable register 1 [dual core device only]",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2en",
                    description: Some(
                        "CPU2 TIM2 timer clock enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtcapben",
                    description: Some(
                        "CPU2 RTC APB clock enable",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi2en",
                    description: Some(
                        "CPU2 SPI2 clock enable",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart2en",
                    description: Some(
                        "CPU2 USART2 clock enable",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1en",
                    description: Some(
                        "CPU2 I2C1 clocks enable",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2en",
                    description: Some(
                        "CPU2 I2C2 clocks enable",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c3en",
                    description: Some(
                        "CPU2 I2C3 clocks enable",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dac1en",
                    description: Some(
                        "CPU2 DAC1 clock enable",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim1en",
                    description: Some(
                        "CPU2 Low power timer 1 clocks enable",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "C2apb1smenr2",
            extends: None,
            description: Some(
                "CPU2 APB1 peripheral clocks enable in Sleep mode register 2 [dual core device only]",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpuart1smen",
                    description: Some(
                        "Low power UART 1 clock enable during CPU2 CSleep and CStop mode",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim2smen",
                    description: Some(
                        "Low power timer 2 clocks enable during CPU2 CSleep and CStop modes.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim3smen",
                    description: Some(
                        "Low power timer 3 clocks enable during CPU2 CSleep and CStop modes.",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "C2apb3enr",
            extends: None,
            description: Some(
                "CPU2 APB3 peripheral clock enable register [dual core device only]",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "subghzspien",
                    description: Some(
                        "CPU2 sub-GHz radio SPI clock enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "C2ahb2smenr",
            extends: None,
            description: Some(
                "CPU2 AHB2 peripheral clocks enable in Sleep modes register [dual core device only]",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioasmen",
                    description: Some(
                        "IO port A clock enable during CPU2 CSleep mode.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiobsmen",
                    description: Some(
                        "IO port B clock enable during CPU2 CSleep mode.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiocsmen",
                    description: Some(
                        "IO port C clock enable during CPU2 CSleep mode.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiohsmen",
                    description: Some(
                        "IO port H clock enable during CPU2 CSleep mode.",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cier",
            extends: None,
            description: Some(
                "Clock interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyie",
                    description: Some(
                        "LSI ready interrupt enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lserdyie",
                    description: Some(
                        "LSE ready interrupt enable",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msirdyie",
                    description: Some(
                        "MSI ready interrupt enable",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsirdyie",
                    description: Some(
                        "HSI16 ready interrupt enable",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hserdyie",
                    description: Some(
                        "HSE32 ready interrupt enable",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllrdyie",
                    description: Some(
                        "PLL ready interrupt enable",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsecssie",
                    description: Some(
                        "LSE clock security system interrupt enable",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1enr2",
            extends: None,
            description: Some(
                "APB1 peripheral clock enable register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpuart1en",
                    description: Some(
                        "CPU1 Low power UART 1 clocks enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim2en",
                    description: Some(
                        "CPU1 Low power timer 2 clocks enable",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim3en",
                    description: Some(
                        "CPU1 Low power timer 3 clocks enable",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1enr1",
            extends: None,
            description: Some(
                "APB1 peripheral clock enable register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2en",
                    description: Some(
                        "CPU1 TIM2 timer clock enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtcapben",
                    description: Some(
                        "CPU1 RTC APB clock enable",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wwdgen",
                    description: Some(
                        "CPU1 Window watchdog clock enable",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi2en",
                    description: Some(
                        "CPU1 SPI2 clock enable",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart2en",
                    description: Some(
                        "CPU1 USART2 clock enable",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1en",
                    description: Some(
                        "CPU1 I2C1 clocks enable",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2en",
                    description: Some(
                        "CPU1 I2C2 clocks enable",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c3en",
                    description: Some(
                        "CPU1 I2C3 clocks enable",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dac1en",
                    description: Some(
                        "CPU1 DAC1 clock enable",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim1en",
                    description: Some(
                        "CPU1 Low power timer 1 clocks enable",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "C2ahb2enr",
            extends: None,
            description: Some(
                "CPU2 AHB2 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioaen",
                    description: Some(
                        "CPU2 IO port A clock enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioben",
                    description: Some(
                        "CPU2 IO port B clock enable",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiocen",
                    description: Some(
                        "CPU2 IO port C clock enable",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiohen",
                    description: Some(
                        "CPU2 IO port H clock enable",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb3rstr",
            extends: None,
            description: Some(
                "AHB3 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pkarst",
                    description: Some(
                        "PKARST",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aesrst",
                    description: Some(
                        "AESRST",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rngrst",
                    description: Some(
                        "RNGRST",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsemrst",
                    description: Some(
                        "HSEMRST",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ipccrst",
                    description: Some(
                        "IPCCRST",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flashrst",
                    description: Some(
                        "Flash interface reset",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb1smenr",
            extends: None,
            description: Some(
                "AHB1 peripheral clocks enable in Sleep modes register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma1smen",
                    description: Some(
                        "DMA1 clock enable during CPU1 CSleep mode.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dma2smen",
                    description: Some(
                        "DMA2 clock enable during CPU1 CSleep mode",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmamux1smen",
                    description: Some(
                        "DMAMUX1 clock enable during CPU1 CSleep mode.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crcsmen",
                    description: Some(
                        "CRC clock enable during CPU1 CSleep mode.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bdcr",
            extends: None,
            description: Some(
                "Backup domain control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lseon",
                    description: Some(
                        "LSE oscillator enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lserdy",
                    description: Some(
                        "LSE oscillator ready",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsebyp",
                    description: Some(
                        "LSE oscillator bypass",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsedrv",
                    description: Some(
                        "LSE oscillator drive capability",
                    ),
                    bit_offset: 3,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lsedrv",
                    ),
                },
                Field {
                    name: "lsecsson",
                    description: Some(
                        "CSS on LSE enable",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsecssd",
                    description: Some(
                        "CSS on LSE failure Detection",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsesysen",
                    description: Some(
                        "LSE system clock enable",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtcsel",
                    description: Some(
                        "RTC clock source selection",
                    ),
                    bit_offset: 8,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Rtcsel",
                    ),
                },
                Field {
                    name: "lsesysrdy",
                    description: Some(
                        "LSE system clock ready",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtcen",
                    description: Some(
                        "RTC clock enable",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bdrst",
                    description: Some(
                        "Backup domain software reset",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lscoen",
                    description: Some(
                        "Low speed clock output enable",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lscosel",
                    description: Some(
                        "Low speed clock output selection",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "C2ahb1smenr",
            extends: None,
            description: Some(
                "CPU2 AHB1 peripheral clocks enable in Sleep modes register [dual core device only]",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma1smen",
                    description: Some(
                        "DMA1 clock enable during CPU2 CSleep mode.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dma2smen",
                    description: Some(
                        "DMA2 clock enable during CPU2 CSleep mode.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmamux1smen",
                    description: Some(
                        "DMAMUX1 clock enable during CPU2 CSleep mode.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crcsmen",
                    description: Some(
                        "CRC clock enable during CPU2 CSleep mode.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb1enr",
            extends: None,
            description: Some(
                "AHB1 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma1en",
                    description: Some(
                        "CPU1 DMA1 clock enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dma2en",
                    description: Some(
                        "CPU1 DMA2 clock enable",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmamux1en",
                    description: Some(
                        "CPU1 DMAMUX1 clock enable",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crcen",
                    description: Some(
                        "CPU1 CRC clock enable",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "C2apb2enr",
            extends: None,
            description: Some(
                "CPU2 APB2 peripheral clock enable register [dual core device only]",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adcen",
                    description: Some(
                        "ADC clocks enable",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim1en",
                    description: Some(
                        "CPU2 TIM1 timer clock enable",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi1en",
                    description: Some(
                        "CPU2 SPI1 clock enable",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart1en",
                    description: Some(
                        "CPU2 USART1clocks enable",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim16en",
                    description: Some(
                        "CPU2 TIM16 timer clock enable",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim17en",
                    description: Some(
                        "CPU2 TIM17 timer clock enable",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb3enr",
            extends: None,
            description: Some(
                "APB3 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "subghzspien",
                    description: Some(
                        "sub-GHz radio SPI clock enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb2rstr",
            extends: None,
            description: Some(
                "AHB2 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioarst",
                    description: Some(
                        "IO port A reset",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiobrst",
                    description: Some(
                        "IO port B reset",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiocrst",
                    description: Some(
                        "IO port C reset",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiohrst",
                    description: Some(
                        "IO port H reset",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "C2ahb3smenr",
            extends: None,
            description: Some(
                "CPU2 AHB3 peripheral clocks enable in Sleep mode register [dual core device only]",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pkasmen",
                    description: Some(
                        "PKA accelerator clock enable during CPU2 CSleep mode.",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aessmen",
                    description: Some(
                        "AES accelerator clock enable during CPU2 CSleep mode.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rngsmen",
                    description: Some(
                        "True RNG clock enable during CPU2 CSleep and CStop mode.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram1smen",
                    description: Some(
                        "SRAM1 interface clock enable during CPU2 CSleep mode.",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram2smen",
                    description: Some(
                        "SRAM2 memory interface clock enable during CPU2 CSleep mode.",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flashsmen",
                    description: Some(
                        "Flash interface clock enable during CPU2 CSleep mode.",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb2enr",
            extends: None,
            description: Some(
                "AHB2 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioaen",
                    description: Some(
                        "CPU1 IO port A clock enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioben",
                    description: Some(
                        "CPU1 IO port B clock enable",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiocen",
                    description: Some(
                        "CPU1 IO port C clock enable",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiohen",
                    description: Some(
                        "CPU1 IO port H clock enable",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1rstr2",
            extends: None,
            description: Some(
                "APB1 peripheral reset register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpuart1rst",
                    description: Some(
                        "Low-power UART 1 reset",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim2rst",
                    description: Some(
                        "Low-power timer 2 reset",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim3rst",
                    description: Some(
                        "Low-power timer 3 reset",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb3smenr",
            extends: None,
            description: Some(
                "APB3 peripheral clock enable in Sleep mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "subghzspismen",
                    description: Some(
                        "Sub-GHz radio SPI clock enable during Sleep and Stop modes",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "Clock control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "msion",
                    description: Some(
                        "MSI clock enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msirdy",
                    description: Some(
                        "MSI clock ready flag (After reset this bit will be read 1 once the MSI is ready)",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msipllen",
                    description: Some(
                        "MSI clock PLL enable",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msirgsel",
                    description: Some(
                        "MSI range control selection",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msirange",
                    description: Some(
                        "MSI clock ranges",
                    ),
                    bit_offset: 4,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsion",
                    description: Some(
                        "HSI16 clock enable",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsikeron",
                    description: Some(
                        "HSI16 always enable for peripheral kernel clocks.",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsirdy",
                    description: Some(
                        "HSI16 clock ready flag. (After wakeup from Stop this bit will be read 1 once the HSI16 is ready)",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsiasfs",
                    description: Some(
                        "HSI16 automatic start from Stop",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsikerdy",
                    description: Some(
                        "HSI16 kernel clock ready flag for peripherals requests.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hseon",
                    description: Some(
                        "HSE32 clock enable",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hserdy",
                    description: Some(
                        "HSE32 clock ready flag",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csson",
                    description: Some(
                        "HSE32 Clock security system enable",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsepre",
                    description: Some(
                        "HSE32 sysclk prescaler",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsebyppwr",
                    description: Some(
                        "Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO.",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllon",
                    description: Some(
                        "Main PLL enable",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllrdy",
                    description: Some(
                        "Main PLL clock ready flag",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Icscr",
            extends: None,
            description: Some(
                "Internal clock sources calibration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "msical",
                    description: Some(
                        "MSI clock calibration",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msitrim",
                    description: Some(
                        "MSI clock trimming",
                    ),
                    bit_offset: 8,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsical",
                    description: Some(
                        "HSI16 clock calibration",
                    ),
                    bit_offset: 16,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsitrim",
                    description: Some(
                        "HSI16 clock trimming",
                    ),
                    bit_offset: 24,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb1rstr",
            extends: None,
            description: Some(
                "AHB1 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma1rst",
                    description: Some(
                        "DMA1 reset",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dma2rst",
                    description: Some(
                        "DMA2 reset",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmamux1rst",
                    description: Some(
                        "DMAMUX1 reset",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crcrst",
                    description: Some(
                        "CRC reset",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb3smenr",
            extends: None,
            description: Some(
                "AHB3 peripheral clocks enable in Sleep and Stop modes register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pkasmen",
                    description: Some(
                        "PKA accelerator clock enable during CPU1 CSleep mode.",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aessmen",
                    description: Some(
                        "AES accelerator clock enable during CPU1 CSleep mode.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rngsmen",
                    description: Some(
                        "True RNG clocks enable during CPU1 Csleep and CStop modes",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram1smen",
                    description: Some(
                        "SRAM1 interface clock enable during CPU1 CSleep mode.",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram2smen",
                    description: Some(
                        "SRAM2 memory interface clock enable during CPU1 CSleep mode",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flashsmen",
                    description: Some(
                        "Flash interface clock enable during CPU1 CSleep mode.",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb2smenr",
            extends: None,
            description: Some(
                "APB2 peripheral clocks enable in Sleep mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adcsmen",
                    description: Some(
                        "ADC clocks enable during CPU1 Csleep and CStop modes",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim1smen",
                    description: Some(
                        "TIM1 timer clock enable during CPU1 CSleep mode.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi1smen",
                    description: Some(
                        "SPI1 clock enable during CPU1 CSleep mode.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart1smen",
                    description: Some(
                        "USART1 clock enable during CPU1 Csleep and CStop modes.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim16smen",
                    description: Some(
                        "TIM16 timer clock enable during CPU1 CSleep mode.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim17smen",
                    description: Some(
                        "TIM17 timer clock enable during CPU1 CSleep mode.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb3rstr",
            extends: None,
            description: Some(
                "APB3 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "subghzspirst",
                    description: Some(
                        "Sub-GHz radio SPI reset",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ccipr",
            extends: None,
            description: Some(
                "Peripherals independent clock configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart1sel",
                    description: Some(
                        "USART1 clock source selection",
                    ),
                    bit_offset: 0,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart2sel",
                    description: Some(
                        "USART2 clock source selection",
                    ),
                    bit_offset: 2,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi2sel",
                    description: Some(
                        "SPI2 I2S clock source selection",
                    ),
                    bit_offset: 8,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpuart1sel",
                    description: Some(
                        "LPUART1 clock source selection",
                    ),
                    bit_offset: 10,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1sel",
                    description: Some(
                        "I2C1 clock source selection",
                    ),
                    bit_offset: 12,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2sel",
                    description: Some(
                        "I2C2 clock source selection",
                    ),
                    bit_offset: 14,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c3sel",
                    description: Some(
                        "I2C3 clock source selection",
                    ),
                    bit_offset: 16,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim1sel",
                    description: Some(
                        "Low power timer 1 clock source selection",
                    ),
                    bit_offset: 18,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim2sel",
                    description: Some(
                        "Low power timer 2 clock source selection",
                    ),
                    bit_offset: 20,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim3sel",
                    description: Some(
                        "Low power timer 3 clock source selection",
                    ),
                    bit_offset: 22,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adcsel",
                    description: Some(
                        "ADC clock source selection",
                    ),
                    bit_offset: 28,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Adcsel",
                    ),
                },
                Field {
                    name: "rngsel",
                    description: Some(
                        "RNG clock source selection",
                    ),
                    bit_offset: 30,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb3enr",
            extends: None,
            description: Some(
                "AHB3 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pkaen",
                    description: Some(
                        "PKAEN",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aesen",
                    description: Some(
                        "AESEN",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rngen",
                    description: Some(
                        "RNGEN",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsemen",
                    description: Some(
                        "HSEMEN",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ipccen",
                    description: Some(
                        "IPCCEN",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flashen",
                    description: Some(
                        "CPU1 Flash interface clock enable",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "C2apb2smenr",
            extends: None,
            description: Some(
                "CPU2 APB2 peripheral clocks enable in Sleep mode register [dual core device only]",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adcsmen",
                    description: Some(
                        "ADC clocks enable during CPU2 Csleep and CStop modes",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim1smen",
                    description: Some(
                        "TIM1 timer clock enable during CPU2 CSleep mode",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi1smen",
                    description: Some(
                        "SPI1 clock enable during CPU2 CSleep mode",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart1smen",
                    description: Some(
                        "USART1clock enable during CPU2 CSleep and CStop mode",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim16smen",
                    description: Some(
                        "TIM16 timer clock enable during CPU2 CSleep mode",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim17smen",
                    description: Some(
                        "TIM17 timer clock enable during CPU2 CSleep mode",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "C2apb1smenr1",
            extends: None,
            description: Some(
                "CPU2 APB1 peripheral clocks enable in Sleep mode register 1 [dual core device only]",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2smen",
                    description: Some(
                        "TIM2 timer clock enable during CPU2 CSleep mode.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtcapbsmen",
                    description: Some(
                        "RTC bus clock enable during CPU2 CSleep mode.",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi2smen",
                    description: Some(
                        "SPI2 clock enable during CPU2 CSleep mode.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart2smen",
                    description: Some(
                        "USART2 clock enable during CPU2 CSleep mode.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1smen",
                    description: Some(
                        "I2C1 clock enable during CPU2 CSleep and CStop modes",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2smen",
                    description: Some(
                        "I2C2 clock enable during CPU2 CSleep and CStop modes",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c3smen",
                    description: Some(
                        "I2C3 clock enable during CPU2 CSleep and CStop modes",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dac1smen",
                    description: Some(
                        "DAC1 clock enable during CPU2 CSleep mode.",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim1smen",
                    description: Some(
                        "Low power timer 1 clock enable during CPU2 CSleep and CStop mode",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfgr",
            extends: None,
            description: Some(
                "Clock configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sw",
                    description: Some(
                        "System clock switch",
                    ),
                    bit_offset: 0,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sws",
                    description: Some(
                        "System clock switch status",
                    ),
                    bit_offset: 2,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hpre",
                    description: Some(
                        "HCLK1 prescaler (CPU1, AHB1, AHB2, and SRAM1.)",
                    ),
                    bit_offset: 4,
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Hpre",
                    ),
                },
                Field {
                    name: "ppre1",
                    description: Some(
                        "PCLK1 low-speed prescaler (APB1)",
                    ),
                    bit_offset: 8,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ppre",
                    ),
                },
                Field {
                    name: "ppre2",
                    description: Some(
                        "PCLK2 high-speed prescaler (APB2)",
                    ),
                    bit_offset: 11,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ppre",
                    ),
                },
                Field {
                    name: "stopwuck",
                    description: Some(
                        "Wakeup from Stop and CSS backup clock selection",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hpref",
                    description: Some(
                        "HCLK1 prescaler flag (CPU1, AHB1, AHB2, and SRAM1)",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ppre1f",
                    description: Some(
                        "PCLK1 prescaler flag (APB1)",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ppre2f",
                    description: Some(
                        "PCLK2 prescaler flag (APB2)",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mcosel",
                    description: Some(
                        "Microcontroller clock output",
                    ),
                    bit_offset: 24,
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Mcosel",
                    ),
                },
                Field {
                    name: "mcopre",
                    description: Some(
                        "Microcontroller clock output prescaler",
                    ),
                    bit_offset: 28,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mcopre",
                    ),
                },
            ],
        },
        FieldSet {
            name: "C2apb1enr2",
            extends: None,
            description: Some(
                "CPU2 APB1 peripheral clock enable register 2 [dual core device only]",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpuart1en",
                    description: Some(
                        "CPU2 Low power UART 1 clocks enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim2en",
                    description: Some(
                        "CPU2 Low power timer 2 clocks enable",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim3en",
                    description: Some(
                        "CPU2 Low power timer 3 clocks enable",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pllcfgr",
            extends: None,
            description: Some(
                "PLL configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllsrc",
                    description: Some(
                        "Main PLL entry clock source",
                    ),
                    bit_offset: 0,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllm",
                    description: Some(
                        "Division factor for the main PLL input clock",
                    ),
                    bit_offset: 4,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "plln",
                    description: Some(
                        "Main PLL multiplication factor for VCO",
                    ),
                    bit_offset: 8,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllpen",
                    description: Some(
                        "Main PLL PLLPCLK output enable",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllp",
                    description: Some(
                        "Main PLL division factor for PLLPCLK.",
                    ),
                    bit_offset: 17,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllqen",
                    description: Some(
                        "Main PLL PLLQCLK output enable",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllq",
                    description: Some(
                        "Main PLL division factor for PLLQCLK",
                    ),
                    bit_offset: 25,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllren",
                    description: Some(
                        "Main PLL PLLRCLK output enable",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllr",
                    description: Some(
                        "Main PLL division factor for PLLRCLK",
                    ),
                    bit_offset: 29,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1smenr2",
            extends: None,
            description: Some(
                "APB1 peripheral clocks enable in Sleep mode register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpuart1smen",
                    description: Some(
                        "Low power UART 1 clock enable during CPU1 Csleep and CStop modes.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim2smen",
                    description: Some(
                        "Low power timer 2 clock enable during CPU1 Csleep and CStop modes",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim3smen",
                    description: Some(
                        "Low power timer 3 clock enable during CPU1 Csleep and CStop modes",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Adcsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HSI16",
                    description: Some(
                        "HSI16 used as ADC clock source",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLLPCLK",
                    description: Some(
                        "PLLPCLK used as ADC clock source",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "SYSCLK",
                    description: Some(
                        "SYSCLK used as ADC clock source",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Hpre",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "DCLK not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV3",
                    description: Some(
                        "hclk = SYSCLK divided by 3",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV5",
                    description: Some(
                        "hclk = SYSCLK divided by 5",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV6",
                    description: Some(
                        "hclk = SYSCLK divided by 6",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV10",
                    description: Some(
                        "hclk = SYSCLK divided by 8",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV32",
                    description: Some(
                        "hclk = SYSCLK divided by 32",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "hclk = SYSCLK divided by 2",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "hclk = SYSCLK divided by 4",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "hclk = SYSCLK divided by 8",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "hclk = SYSCLK divided by 16",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "DIV64",
                    description: Some(
                        "hclk = SYSCLK divided by 64",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "DIV128",
                    description: Some(
                        "hclk = SYSCLK divided by 128",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "DIV256",
                    description: Some(
                        "hclk = SYSCLK divided by 256",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "DIV512",
                    description: Some(
                        "hclk = SYSCLK divided by 256",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Ppre",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "HCLK not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "HCLK divided by 2",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "HCLK divided by 4",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "HCLK divided by 8",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "HCLK divided by 16",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Rtcsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NOCLOCK",
                    description: Some(
                        "No clock selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE oscillator clock selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI oscillator clock selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE oscillator clock divided by 32 selected",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Lsedrv",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "LOW",
                    description: Some(
                        "Low driving capability",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MEDIUMLOW",
                    description: Some(
                        "Medium low driving capability",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MEDIUMHIGH",
                    description: Some(
                        "Medium high driving capability",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HIGH",
                    description: Some(
                        "High driving capability",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Mcosel",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "NOCLOCK",
                    description: Some(
                        "No clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYSCLK",
                    description: Some(
                        "SYSCLK clock selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MSI",
                    description: Some(
                        "MSI oscillator clock selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSI16",
                    description: Some(
                        "HSI oscillator clock selected",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "HSE32",
                    description: Some(
                        "HSE32 oscillator clock selected",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "PLLRCLK",
                    description: Some(
                        "Main PLLRCLK clock selected",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI oscillator clock selected",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE oscillator clock selected",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "PLLPCLK",
                    description: Some(
                        "Main PLLCLK oscillator clock selected",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "PLLQCLK",
                    description: Some(
                        "Main PLLQCLK oscillator clock selected",
                    ),
                    value: 14,
                },
            ],
        },
        Enum {
            name: "Mcopre",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "No division",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "Division by 2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "Division by 4",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "Division by 8",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "Division by 16",
                    ),
                    value: 4,
                },
            ],
        },
    ],
};