
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Dac",
        extends: None,
        description: Some("Digital-to-analog converter"),
        items: &[
            BlockItem {
                name: "cr",
                description: Some("control register"),
                array: None,
                byte_offset: 0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr"),
                }),
            },
            BlockItem {
                name: "swtrigr",
                description: Some("software trigger register"),
                array: None,
                byte_offset: 4,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Swtrigr"),
                }),
            },
            BlockItem {
                name: "dhr12r",
                description: Some("channel 12-bit right-aligned data holding register"),
                array: Some(Array::Regular(RegularArray { len: 2, stride: 12 })),
                byte_offset: 8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Dhr12r"),
                }),
            },
            BlockItem {
                name: "dhr12l",
                description: Some("channel 12-bit left-aligned data holding register"),
                array: Some(Array::Regular(RegularArray { len: 2, stride: 12 })),
                byte_offset: 12,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Dhr12l"),
                }),
            },
            BlockItem {
                name: "dhr8r",
                description: Some("channel 8-bit right-aligned data holding register"),
                array: Some(Array::Regular(RegularArray { len: 2, stride: 12 })),
                byte_offset: 16,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Dhr8r"),
                }),
            },
            BlockItem {
                name: "dhr12rd",
                description: Some("Dual DAC 12-bit right-aligned data holding register"),
                array: None,
                byte_offset: 32,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Dhr12rd"),
                }),
            },
            BlockItem {
                name: "dhr12ld",
                description: Some("DUAL DAC 12-bit left aligned data holding register"),
                array: None,
                byte_offset: 36,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Dhr12ld"),
                }),
            },
            BlockItem {
                name: "dhr8rd",
                description: Some("DUAL DAC 8-bit right aligned data holding register"),
                array: None,
                byte_offset: 40,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Dhr8rd"),
                }),
            },
            BlockItem {
                name: "dor",
                description: Some("channel data output register"),
                array: Some(Array::Regular(RegularArray { len: 2, stride: 4 })),
                byte_offset: 44,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Dor"),
                }),
            },
            BlockItem {
                name: "sr",
                description: Some("status register"),
                array: None,
                byte_offset: 52,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Sr"),
                }),
            },
            BlockItem {
                name: "ccr",
                description: Some("calibration control register"),
                array: None,
                byte_offset: 56,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ccr"),
                }),
            },
            BlockItem {
                name: "mcr",
                description: Some("mode control register"),
                array: None,
                byte_offset: 60,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Mcr"),
                }),
            },
            BlockItem {
                name: "shsr1",
                description: Some("Sample and Hold sample time register"),
                array: Some(Array::Regular(RegularArray { len: 2, stride: 4 })),
                byte_offset: 64,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Shsr"),
                }),
            },
            BlockItem {
                name: "shhr",
                description: Some("Sample and Hold hold time register"),
                array: None,
                byte_offset: 72,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Shhr"),
                }),
            },
            BlockItem {
                name: "shrr",
                description: Some("Sample and Hold refresh time register"),
                array: None,
                byte_offset: 76,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Shrr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Ccr",
            extends: None,
            description: Some("calibration control register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "otrim1",
                    description: Some("DAC Channel 1 offset trimming value"),
                    bit_offset: 0,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "otrim2",
                    description: Some("DAC Channel 2 offset trimming value"),
                    bit_offset: 16,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some("control register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some("DAC channel enable"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 2, stride: 16 })),
                    enumm: None,
                },
                Field {
                    name: "ten",
                    description: Some("DAC channel trigger enable"),
                    bit_offset: 1,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 2, stride: 16 })),
                    enumm: None,
                },
                Field {
                    name: "tsel1",
                    description: Some("DAC channel 1 trigger selection"),
                    bit_offset: 2,
                    bit_size: 4,
                    array: None,
                    enumm: Some("Tsel1"),
                },
                Field {
                    name: "wave",
                    description: Some("DAC channel noise/triangle wave generation enable"),
                    bit_offset: 6,
                    bit_size: 2,
                    array: Some(Array::Regular(RegularArray { len: 2, stride: 16 })),
                    enumm: Some("Wave"),
                },
                Field {
                    name: "mamp",
                    description: Some("DAC channel mask/amplitude selector"),
                    bit_offset: 8,
                    bit_size: 4,
                    array: Some(Array::Regular(RegularArray { len: 2, stride: 16 })),
                    enumm: None,
                },
                Field {
                    name: "dmaen",
                    description: Some("DAC channel DMA enable"),
                    bit_offset: 12,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 2, stride: 16 })),
                    enumm: None,
                },
                Field {
                    name: "dmaudrie",
                    description: Some("DAC channel DMA Underrun Interrupt enable"),
                    bit_offset: 13,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 2, stride: 16 })),
                    enumm: None,
                },
                Field {
                    name: "cen",
                    description: Some("DAC channel calibration enable"),
                    bit_offset: 14,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 2, stride: 16 })),
                    enumm: None,
                },
                Field {
                    name: "tsel2",
                    description: Some("DAC channel 2 trigger selection"),
                    bit_offset: 18,
                    bit_size: 4,
                    array: None,
                    enumm: Some("Tsel2"),
                },
            ],
        },
        FieldSet {
            name: "Dhr12l",
            extends: None,
            description: Some("channel 12-bit left-aligned data holding register"),
            bit_size: 32,
            fields: &[Field {
                name: "dhr",
                description: Some("DAC channel 12-bit left-aligned data"),
                bit_offset: 4,
                bit_size: 12,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Dhr12ld",
            extends: None,
            description: Some("DUAL DAC 12-bit left aligned data holding register"),
            bit_size: 32,
            fields: &[Field {
                name: "dhr",
                description: Some("DAC channel 12-bit left-aligned data"),
                bit_offset: 4,
                bit_size: 12,
                array: Some(Array::Regular(RegularArray { len: 2, stride: 16 })),
                enumm: None,
            }],
        },
        FieldSet {
            name: "Dhr12r",
            extends: None,
            description: Some("channel 12-bit right-aligned data holding register"),
            bit_size: 32,
            fields: &[Field {
                name: "dhr",
                description: Some("DAC channel 12-bit right-aligned data"),
                bit_offset: 0,
                bit_size: 12,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Dhr12rd",
            extends: None,
            description: Some("Dual DAC 12-bit right-aligned data holding register"),
            bit_size: 32,
            fields: &[Field {
                name: "dhr",
                description: Some("DAC channel 12-bit right-aligned data"),
                bit_offset: 0,
                bit_size: 12,
                array: Some(Array::Regular(RegularArray { len: 2, stride: 16 })),
                enumm: None,
            }],
        },
        FieldSet {
            name: "Dhr8r",
            extends: None,
            description: Some("channel 8-bit right-aligned data holding register"),
            bit_size: 32,
            fields: &[Field {
                name: "dhr",
                description: Some("DAC channel 8-bit right-aligned data"),
                bit_offset: 0,
                bit_size: 8,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Dhr8rd",
            extends: None,
            description: Some("DUAL DAC 8-bit right aligned data holding register"),
            bit_size: 32,
            fields: &[Field {
                name: "dhr",
                description: Some("DAC channel 8-bit right-aligned data"),
                bit_offset: 0,
                bit_size: 8,
                array: Some(Array::Regular(RegularArray { len: 2, stride: 8 })),
                enumm: None,
            }],
        },
        FieldSet {
            name: "Dor",
            extends: None,
            description: Some("channel data output register"),
            bit_size: 32,
            fields: &[Field {
                name: "dor",
                description: Some("DAC channel data output"),
                bit_offset: 0,
                bit_size: 12,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Mcr",
            extends: None,
            description: Some("mode control register"),
            bit_size: 32,
            fields: &[Field {
                name: "mode",
                description: Some("DAC channel mode"),
                bit_offset: 0,
                bit_size: 3,
                array: Some(Array::Regular(RegularArray { len: 2, stride: 16 })),
                enumm: None,
            }],
        },
        FieldSet {
            name: "Shhr",
            extends: None,
            description: Some("Sample and Hold hold time register"),
            bit_size: 32,
            fields: &[Field {
                name: "thold",
                description: Some("DAC channel hold Time"),
                bit_offset: 0,
                bit_size: 10,
                array: Some(Array::Regular(RegularArray { len: 2, stride: 16 })),
                enumm: None,
            }],
        },
        FieldSet {
            name: "Shrr",
            extends: None,
            description: Some("Sample and Hold refresh time register"),
            bit_size: 32,
            fields: &[Field {
                name: "trefresh",
                description: Some("DAC channel refresh Time"),
                bit_offset: 0,
                bit_size: 8,
                array: Some(Array::Regular(RegularArray { len: 2, stride: 16 })),
                enumm: None,
            }],
        },
        FieldSet {
            name: "Shsr",
            extends: None,
            description: Some("Sample and Hold sample time register"),
            bit_size: 32,
            fields: &[Field {
                name: "tsample",
                description: Some("DAC channel sample Time"),
                bit_offset: 0,
                bit_size: 10,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some("status register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmaudr",
                    description: Some("DAC channel DMA underrun flag"),
                    bit_offset: 13,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 2, stride: 16 })),
                    enumm: None,
                },
                Field {
                    name: "cal_flag",
                    description: Some("DAC channel calibration offset status"),
                    bit_offset: 14,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 2, stride: 16 })),
                    enumm: None,
                },
                Field {
                    name: "bwst",
                    description: Some("DAC channel busy writing sample time flag"),
                    bit_offset: 15,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 2, stride: 16 })),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Swtrigr",
            extends: None,
            description: Some("software trigger register"),
            bit_size: 32,
            fields: &[Field {
                name: "swtrig",
                description: Some("DAC channel software trigger"),
                bit_offset: 0,
                bit_size: 1,
                array: Some(Array::Regular(RegularArray { len: 2, stride: 1 })),
                enumm: None,
            }],
        },
    ],
    enums: &[
        Enum {
            name: "Tsel1",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "SOFTWARE",
                    description: Some("Software trigger"),
                    value: 0,
                },
                EnumVariant {
                    name: "TIM1_TRGO",
                    description: Some("Timer 1 TRGO event"),
                    value: 1,
                },
                EnumVariant {
                    name: "TIM2_TRGO",
                    description: Some("Timer 2 TRGO event"),
                    value: 2,
                },
                EnumVariant {
                    name: "TIM4_TRGO",
                    description: Some("Timer 4 TRGO event"),
                    value: 3,
                },
                EnumVariant {
                    name: "TIM5_TRGO",
                    description: Some("Timer 5 TRGO event"),
                    value: 4,
                },
                EnumVariant {
                    name: "TIM6_TRGO",
                    description: Some("Timer 6 TRGO event"),
                    value: 5,
                },
                EnumVariant {
                    name: "TIM7_TRGO",
                    description: Some("Timer 7 TRGO event"),
                    value: 6,
                },
                EnumVariant {
                    name: "TIM8_TRGO",
                    description: Some("Timer 8 TRGO event"),
                    value: 7,
                },
                EnumVariant {
                    name: "TIM15_TRGO",
                    description: Some("Timer 15 TRGO event"),
                    value: 8,
                },
                EnumVariant {
                    name: "HRTIM1_DACTRG1",
                    description: Some("High resolution timer 1 DACTRG1 event"),
                    value: 9,
                },
                EnumVariant {
                    name: "HRTIM1_DACTRG2",
                    description: Some("High resolution timer 1 DACTRG2 event"),
                    value: 10,
                },
                EnumVariant {
                    name: "LPTIM1_OUT",
                    description: Some("Low-power timer 1 OUT event"),
                    value: 11,
                },
                EnumVariant {
                    name: "LPTIM2_OUT",
                    description: Some("Low-power timer 2 OUT event"),
                    value: 12,
                },
                EnumVariant {
                    name: "EXTI9",
                    description: Some("EXTI line9"),
                    value: 13,
                },
                EnumVariant {
                    name: "LPTIM3_OUT",
                    description: Some("Low-power timer 3 OUT event"),
                    value: 14,
                },
            ],
        },
        Enum {
            name: "Tsel2",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "SOFTWARE",
                    description: Some("Software trigger"),
                    value: 0,
                },
                EnumVariant {
                    name: "TIM1_TRGO",
                    description: Some("Timer 1 TRGO event"),
                    value: 1,
                },
                EnumVariant {
                    name: "TIM2_TRGO",
                    description: Some("Timer 2 TRGO event"),
                    value: 2,
                },
                EnumVariant {
                    name: "TIM4_TRGO",
                    description: Some("Timer 4 TRGO event"),
                    value: 3,
                },
                EnumVariant {
                    name: "TIM5_TRGO",
                    description: Some("Timer 5 TRGO event"),
                    value: 4,
                },
                EnumVariant {
                    name: "TIM6_TRGO",
                    description: Some("Timer 6 TRGO event"),
                    value: 5,
                },
                EnumVariant {
                    name: "TIM7_TRGO",
                    description: Some("Timer 7 TRGO event"),
                    value: 6,
                },
                EnumVariant {
                    name: "TIM8_TRGO",
                    description: Some("Timer 8 TRGO event"),
                    value: 7,
                },
                EnumVariant {
                    name: "TIM15_TRGO",
                    description: Some("Timer 15 TRGO event"),
                    value: 8,
                },
                EnumVariant {
                    name: "HRTIM1_DACTRG1",
                    description: Some("High resolution timer 1 DACTRG1 event"),
                    value: 9,
                },
                EnumVariant {
                    name: "HRTIM1_DACTRG2",
                    description: Some("High resolution timer 1 DACTRG2 event"),
                    value: 10,
                },
                EnumVariant {
                    name: "LPTIM1_OUT",
                    description: Some("Low-power timer 1 OUT event"),
                    value: 11,
                },
                EnumVariant {
                    name: "LPTIM2_OUT",
                    description: Some("Low-power timer 2 OUT event"),
                    value: 12,
                },
                EnumVariant {
                    name: "EXTI9",
                    description: Some("EXTI line9"),
                    value: 13,
                },
                EnumVariant {
                    name: "LPTIM3_OUT",
                    description: Some("Low-power timer 3 OUT event"),
                    value: 14,
                },
            ],
        },
        Enum {
            name: "Wave",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some("Wave generation disabled"),
                    value: 0,
                },
                EnumVariant {
                    name: "NOISE",
                    description: Some("Noise wave generation enabled"),
                    value: 1,
                },
                EnumVariant {
                    name: "TRIANGLE",
                    description: Some("Triangle wave generation enabled"),
                    value: 2,
                },
            ],
        },
    ],
};
