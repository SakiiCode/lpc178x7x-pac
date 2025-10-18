#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    p0_0: P0_0,
    p0_1: P0_1,
    p0_2: P0_2,
    p0_3: P0_3,
    p0_4: P0_4,
    p0_5: P0_5,
    p0_6: P0_6,
    p0_7: P0_7,
    p0_8: P0_8,
    p0_9: P0_9,
    p0_10: P0_10,
    p0_11: P0_11,
    p0_12: P0_12,
    p0_13: P0_13,
    p0_14: P0_14,
    p0_15: P0_15,
    p0_16: P0_16,
    p0_17: P0_17,
    p0_18: P0_18,
    p0_19: P0_19,
    p0_20: P0_20,
    p0_21: P0_21,
    p0_22: P0_22,
    p0_23: P0_23,
    p0_24: P0_24,
    p0_25: P0_25,
    p0_26: P0_26,
    p0_27: P0_27,
    p0_28: P0_28,
    p0_29: P0_29,
    p0_30: P0_30,
    p0_31: P0_31,
    p1_0: P1_0,
    p1_1: P1_1,
    p1_2: P1_2,
    p1_3: P1_3,
    p1_4: P1_4,
    p1_5: P1_5,
    p1_6: P1_6,
    p1_7: P1_7,
    p1_8: P1_8,
    p1_9: P1_9,
    p1_10: P1_10,
    p1_11: P1_11,
    p1_12: P1_12,
    p1_13: P1_13,
    p1_14: P1_14,
    p1_15: P1_15,
    p1_16: P1_16,
    p1_17: P1_17,
    p1_18: P1_18,
    p1_19: P1_19,
    p1_20: P1_20,
    p1_21: P1_21,
    p1_22: P1_22,
    p1_23: P1_23,
    p1_24: P1_24,
    p1_25: P1_25,
    p1_26: P1_26,
    p1_27: P1_27,
    p1_28: P1_28,
    p1_29: P1_29,
    p1_30: P1_30,
    p1_31: P1_31,
    p2_0: P2_0,
    p2_1: P2_1,
    p2_2: P2_2,
    p2_3: P2_3,
    p2_4: P2_4,
    p2_5: P2_5,
    p2_6: P2_6,
    p2_7: P2_7,
    p2_8: P2_8,
    p2_9: P2_9,
    p2_10: P2_10,
    p2_11: P2_11,
    p2_12: P2_12,
    p2_13: P2_13,
    p2_14: P2_14,
    p2_15: P2_15,
    p2_16: P2_16,
    p2_17: P2_17,
    p2_18: P2_18,
    p2_19: P2_19,
    p2_20: P2_20,
    p2_21: P2_21,
    p2_22: P2_22,
    p2_23: P2_23,
    p2_24: P2_24,
    p2_25: P2_25,
    p2_26: P2_26,
    p2_27: P2_27,
    p2_28: P2_28,
    p2_29: P2_29,
    p2_30: P2_30,
    p2_31: P2_31,
    p3_0: P3_0,
    p3_1: P3_1,
    p3_2: P3_2,
    p3_3: P3_3,
    p3_4: P3_4,
    p3_5: P3_5,
    p3_6: P3_6,
    p3_7: P3_7,
    p3_8: P3_8,
    p3_9: P3_9,
    p3_10: P3_10,
    p3_11: P3_11,
    p3_12: P3_12,
    p3_13: P3_13,
    p3_14: P3_14,
    p3_15: P3_15,
    p3_16: P3_16,
    p3_17: P3_17,
    p3_18: P3_18,
    p3_19: P3_19,
    p3_20: P3_20,
    p3_21: P3_21,
    p3_22: P3_22,
    p3_23: P3_23,
    p3_24: P3_24,
    p3_25: P3_25,
    p3_26: P3_26,
    p3_27: P3_27,
    p3_28: P3_28,
    p3_29: P3_29,
    p3_30: P3_30,
    p3_31: P3_31,
    p4_0: P4_0,
    p4_1: P4_1,
    p4_2: P4_2,
    p4_3: P4_3,
    p4_4: P4_4,
    p4_5: P4_5,
    p4_6: P4_6,
    p4_7: P4_7,
    p4_8: P4_8,
    p4_9: P4_9,
    p4_10: P4_10,
    p4_11: P4_11,
    p4_12: P4_12,
    p4_13: P4_13,
    p4_14: P4_14,
    p4_15: P4_15,
    p4_16: P4_16,
    p4_17: P4_17,
    p4_18: P4_18,
    p4_19: P4_19,
    p4_20: P4_20,
    p4_21: P4_21,
    p4_22: P4_22,
    p4_23: P4_23,
    p4_24: P4_24,
    p4_25: P4_25,
    p4_26: P4_26,
    p4_27: P4_27,
    p4_28: P4_28,
    p4_29: P4_29,
    p4_30: P4_30,
    p4_31: P4_31,
    p5_0: P5_0,
    p5_1: P5_1,
    p5_2: P5_2,
    p5_3: P5_3,
    p5_4: P5_4,
}
impl RegisterBlock {
    #[doc = "0x00 - I/O configuration register for pin P0\\[0\\]"]
    #[inline(always)]
    pub const fn p0_0(&self) -> &P0_0 {
        &self.p0_0
    }
    #[doc = "0x04 - I/O configuration register for pin P0\\[1\\]"]
    #[inline(always)]
    pub const fn p0_1(&self) -> &P0_1 {
        &self.p0_1
    }
    #[doc = "0x08 - I/O configuration register for pin P0\\[2\\]"]
    #[inline(always)]
    pub const fn p0_2(&self) -> &P0_2 {
        &self.p0_2
    }
    #[doc = "0x0c - I/O configuration register for pin P0\\[3\\]"]
    #[inline(always)]
    pub const fn p0_3(&self) -> &P0_3 {
        &self.p0_3
    }
    #[doc = "0x10 - I/O configuration register for pin P0\\[4\\]"]
    #[inline(always)]
    pub const fn p0_4(&self) -> &P0_4 {
        &self.p0_4
    }
    #[doc = "0x14 - I/O configuration register for pin P0\\[5\\]"]
    #[inline(always)]
    pub const fn p0_5(&self) -> &P0_5 {
        &self.p0_5
    }
    #[doc = "0x18 - I/O configuration register for pin P0\\[6\\]"]
    #[inline(always)]
    pub const fn p0_6(&self) -> &P0_6 {
        &self.p0_6
    }
    #[doc = "0x1c - I/O configuration register for pin P0\\[7\\]"]
    #[inline(always)]
    pub const fn p0_7(&self) -> &P0_7 {
        &self.p0_7
    }
    #[doc = "0x20 - I/O configuration register for pin P0\\[8\\]"]
    #[inline(always)]
    pub const fn p0_8(&self) -> &P0_8 {
        &self.p0_8
    }
    #[doc = "0x24 - I/O configuration register for pin P0\\[9\\]"]
    #[inline(always)]
    pub const fn p0_9(&self) -> &P0_9 {
        &self.p0_9
    }
    #[doc = "0x28 - I/O configuration register for pin P0\\[10\\]"]
    #[inline(always)]
    pub const fn p0_10(&self) -> &P0_10 {
        &self.p0_10
    }
    #[doc = "0x2c - I/O configuration register for pin P0\\[11\\]"]
    #[inline(always)]
    pub const fn p0_11(&self) -> &P0_11 {
        &self.p0_11
    }
    #[doc = "0x30 - I/O configuration register for pin P0\\[12\\]"]
    #[inline(always)]
    pub const fn p0_12(&self) -> &P0_12 {
        &self.p0_12
    }
    #[doc = "0x34 - I/O configuration register for pin P0\\[13\\]"]
    #[inline(always)]
    pub const fn p0_13(&self) -> &P0_13 {
        &self.p0_13
    }
    #[doc = "0x38 - I/O configuration register for pin P0\\[14\\]"]
    #[inline(always)]
    pub const fn p0_14(&self) -> &P0_14 {
        &self.p0_14
    }
    #[doc = "0x3c - I/O configuration register for pin P0\\[15\\]"]
    #[inline(always)]
    pub const fn p0_15(&self) -> &P0_15 {
        &self.p0_15
    }
    #[doc = "0x40 - I/O configuration register for pin P0\\[16\\]"]
    #[inline(always)]
    pub const fn p0_16(&self) -> &P0_16 {
        &self.p0_16
    }
    #[doc = "0x44 - I/O configuration register for pin P0\\[17\\]"]
    #[inline(always)]
    pub const fn p0_17(&self) -> &P0_17 {
        &self.p0_17
    }
    #[doc = "0x48 - I/O configuration register for pin P0\\[18\\]"]
    #[inline(always)]
    pub const fn p0_18(&self) -> &P0_18 {
        &self.p0_18
    }
    #[doc = "0x4c - I/O configuration register for pin P0\\[19\\]"]
    #[inline(always)]
    pub const fn p0_19(&self) -> &P0_19 {
        &self.p0_19
    }
    #[doc = "0x50 - I/O configuration register for pin P0\\[20\\]"]
    #[inline(always)]
    pub const fn p0_20(&self) -> &P0_20 {
        &self.p0_20
    }
    #[doc = "0x54 - I/O configuration register for pin P0\\[21\\]"]
    #[inline(always)]
    pub const fn p0_21(&self) -> &P0_21 {
        &self.p0_21
    }
    #[doc = "0x58 - I/O configuration register for pin P0\\[22\\]"]
    #[inline(always)]
    pub const fn p0_22(&self) -> &P0_22 {
        &self.p0_22
    }
    #[doc = "0x5c - I/O configuration register for pin P0\\[23\\]"]
    #[inline(always)]
    pub const fn p0_23(&self) -> &P0_23 {
        &self.p0_23
    }
    #[doc = "0x60 - I/O configuration register for pin P0\\[24\\]"]
    #[inline(always)]
    pub const fn p0_24(&self) -> &P0_24 {
        &self.p0_24
    }
    #[doc = "0x64 - I/O configuration register for pin P0\\[25\\]"]
    #[inline(always)]
    pub const fn p0_25(&self) -> &P0_25 {
        &self.p0_25
    }
    #[doc = "0x68 - I/O configuration register for pin P0\\[26\\]"]
    #[inline(always)]
    pub const fn p0_26(&self) -> &P0_26 {
        &self.p0_26
    }
    #[doc = "0x6c - I/O configuration register for pin P0\\[27\\]"]
    #[inline(always)]
    pub const fn p0_27(&self) -> &P0_27 {
        &self.p0_27
    }
    #[doc = "0x70 - I/O configuration register for pin P0\\[28\\]"]
    #[inline(always)]
    pub const fn p0_28(&self) -> &P0_28 {
        &self.p0_28
    }
    #[doc = "0x74 - I/O configuration register for pin P0\\[29\\]"]
    #[inline(always)]
    pub const fn p0_29(&self) -> &P0_29 {
        &self.p0_29
    }
    #[doc = "0x78 - I/O configuration register for pin P0\\[30\\]"]
    #[inline(always)]
    pub const fn p0_30(&self) -> &P0_30 {
        &self.p0_30
    }
    #[doc = "0x7c - I/O configuration register for pin P0\\[31\\]"]
    #[inline(always)]
    pub const fn p0_31(&self) -> &P0_31 {
        &self.p0_31
    }
    #[doc = "0x80 - I/O configuration register for pin P1\\[0\\]"]
    #[inline(always)]
    pub const fn p1_0(&self) -> &P1_0 {
        &self.p1_0
    }
    #[doc = "0x84 - I/O configuration register for pin P1\\[1\\]"]
    #[inline(always)]
    pub const fn p1_1(&self) -> &P1_1 {
        &self.p1_1
    }
    #[doc = "0x88 - I/O configuration register for pin P1\\[2\\]"]
    #[inline(always)]
    pub const fn p1_2(&self) -> &P1_2 {
        &self.p1_2
    }
    #[doc = "0x8c - I/O configuration register for pin P1\\[3\\]"]
    #[inline(always)]
    pub const fn p1_3(&self) -> &P1_3 {
        &self.p1_3
    }
    #[doc = "0x90 - I/O configuration register for pin P1\\[4\\]"]
    #[inline(always)]
    pub const fn p1_4(&self) -> &P1_4 {
        &self.p1_4
    }
    #[doc = "0x94 - I/O configuration register for pin P1\\[5\\]"]
    #[inline(always)]
    pub const fn p1_5(&self) -> &P1_5 {
        &self.p1_5
    }
    #[doc = "0x98 - I/O configuration register for pin P1\\[6\\]"]
    #[inline(always)]
    pub const fn p1_6(&self) -> &P1_6 {
        &self.p1_6
    }
    #[doc = "0x9c - I/O configuration register for pin P1\\[7\\]"]
    #[inline(always)]
    pub const fn p1_7(&self) -> &P1_7 {
        &self.p1_7
    }
    #[doc = "0xa0 - I/O configuration register for pin P1\\[8\\]"]
    #[inline(always)]
    pub const fn p1_8(&self) -> &P1_8 {
        &self.p1_8
    }
    #[doc = "0xa4 - I/O configuration register for pin P1\\[9\\]"]
    #[inline(always)]
    pub const fn p1_9(&self) -> &P1_9 {
        &self.p1_9
    }
    #[doc = "0xa8 - I/O configuration register for pin P1\\[10\\]"]
    #[inline(always)]
    pub const fn p1_10(&self) -> &P1_10 {
        &self.p1_10
    }
    #[doc = "0xac - I/O configuration register for pin P1\\[11\\]"]
    #[inline(always)]
    pub const fn p1_11(&self) -> &P1_11 {
        &self.p1_11
    }
    #[doc = "0xb0 - I/O configuration register for pin P1\\[12\\]"]
    #[inline(always)]
    pub const fn p1_12(&self) -> &P1_12 {
        &self.p1_12
    }
    #[doc = "0xb4 - I/O configuration register for pin P1\\[13\\]"]
    #[inline(always)]
    pub const fn p1_13(&self) -> &P1_13 {
        &self.p1_13
    }
    #[doc = "0xb8 - I/O configuration register for pin P1\\[14\\]"]
    #[inline(always)]
    pub const fn p1_14(&self) -> &P1_14 {
        &self.p1_14
    }
    #[doc = "0xbc - I/O configuration register for pin P1\\[15\\]"]
    #[inline(always)]
    pub const fn p1_15(&self) -> &P1_15 {
        &self.p1_15
    }
    #[doc = "0xc0 - I/O configuration register for pin P1\\[16\\]"]
    #[inline(always)]
    pub const fn p1_16(&self) -> &P1_16 {
        &self.p1_16
    }
    #[doc = "0xc4 - I/O configuration register for pin P1\\[17\\]"]
    #[inline(always)]
    pub const fn p1_17(&self) -> &P1_17 {
        &self.p1_17
    }
    #[doc = "0xc8 - I/O configuration register for pin P1\\[18\\]"]
    #[inline(always)]
    pub const fn p1_18(&self) -> &P1_18 {
        &self.p1_18
    }
    #[doc = "0xcc - I/O configuration register for pin P1\\[19\\]"]
    #[inline(always)]
    pub const fn p1_19(&self) -> &P1_19 {
        &self.p1_19
    }
    #[doc = "0xd0 - I/O configuration register for pin P1\\[20\\]"]
    #[inline(always)]
    pub const fn p1_20(&self) -> &P1_20 {
        &self.p1_20
    }
    #[doc = "0xd4 - I/O configuration register for pin P1\\[21\\]"]
    #[inline(always)]
    pub const fn p1_21(&self) -> &P1_21 {
        &self.p1_21
    }
    #[doc = "0xd8 - I/O configuration register for pin P1\\[22\\]"]
    #[inline(always)]
    pub const fn p1_22(&self) -> &P1_22 {
        &self.p1_22
    }
    #[doc = "0xdc - I/O configuration register for pin P1\\[23\\]"]
    #[inline(always)]
    pub const fn p1_23(&self) -> &P1_23 {
        &self.p1_23
    }
    #[doc = "0xe0 - I/O configuration register for pin P1\\[24\\]"]
    #[inline(always)]
    pub const fn p1_24(&self) -> &P1_24 {
        &self.p1_24
    }
    #[doc = "0xe4 - I/O configuration register for pin P1\\[25\\]"]
    #[inline(always)]
    pub const fn p1_25(&self) -> &P1_25 {
        &self.p1_25
    }
    #[doc = "0xe8 - I/O configuration register for pin P1\\[26\\]"]
    #[inline(always)]
    pub const fn p1_26(&self) -> &P1_26 {
        &self.p1_26
    }
    #[doc = "0xec - I/O configuration register for pin P1\\[27\\]"]
    #[inline(always)]
    pub const fn p1_27(&self) -> &P1_27 {
        &self.p1_27
    }
    #[doc = "0xf0 - I/O configuration register for pin P1\\[28\\]"]
    #[inline(always)]
    pub const fn p1_28(&self) -> &P1_28 {
        &self.p1_28
    }
    #[doc = "0xf4 - I/O configuration register for pin P1\\[29\\]"]
    #[inline(always)]
    pub const fn p1_29(&self) -> &P1_29 {
        &self.p1_29
    }
    #[doc = "0xf8 - I/O configuration register for pin P1\\[30\\]"]
    #[inline(always)]
    pub const fn p1_30(&self) -> &P1_30 {
        &self.p1_30
    }
    #[doc = "0xfc - I/O configuration register for pin P1\\[31\\]"]
    #[inline(always)]
    pub const fn p1_31(&self) -> &P1_31 {
        &self.p1_31
    }
    #[doc = "0x100 - I/O configuration register for pin P2\\[0\\]"]
    #[inline(always)]
    pub const fn p2_0(&self) -> &P2_0 {
        &self.p2_0
    }
    #[doc = "0x104 - I/O configuration register for pin P2\\[1\\]"]
    #[inline(always)]
    pub const fn p2_1(&self) -> &P2_1 {
        &self.p2_1
    }
    #[doc = "0x108 - I/O configuration register for pin P2\\[2\\]"]
    #[inline(always)]
    pub const fn p2_2(&self) -> &P2_2 {
        &self.p2_2
    }
    #[doc = "0x10c - I/O configuration register for pin P2\\[3\\]"]
    #[inline(always)]
    pub const fn p2_3(&self) -> &P2_3 {
        &self.p2_3
    }
    #[doc = "0x110 - I/O configuration register for pin P2\\[4\\]"]
    #[inline(always)]
    pub const fn p2_4(&self) -> &P2_4 {
        &self.p2_4
    }
    #[doc = "0x114 - I/O configuration register for pin P2\\[5\\]"]
    #[inline(always)]
    pub const fn p2_5(&self) -> &P2_5 {
        &self.p2_5
    }
    #[doc = "0x118 - I/O configuration register for pin P2\\[6\\]"]
    #[inline(always)]
    pub const fn p2_6(&self) -> &P2_6 {
        &self.p2_6
    }
    #[doc = "0x11c - I/O configuration register for pin P2\\[7\\]"]
    #[inline(always)]
    pub const fn p2_7(&self) -> &P2_7 {
        &self.p2_7
    }
    #[doc = "0x120 - I/O configuration register for pin P2\\[8\\]"]
    #[inline(always)]
    pub const fn p2_8(&self) -> &P2_8 {
        &self.p2_8
    }
    #[doc = "0x124 - I/O configuration register for pin P2\\[9\\]"]
    #[inline(always)]
    pub const fn p2_9(&self) -> &P2_9 {
        &self.p2_9
    }
    #[doc = "0x128 - I/O configuration register for pin P2\\[10\\]"]
    #[inline(always)]
    pub const fn p2_10(&self) -> &P2_10 {
        &self.p2_10
    }
    #[doc = "0x12c - I/O configuration register for pin P2\\[11\\]"]
    #[inline(always)]
    pub const fn p2_11(&self) -> &P2_11 {
        &self.p2_11
    }
    #[doc = "0x130 - I/O configuration register for pin P2\\[12\\]"]
    #[inline(always)]
    pub const fn p2_12(&self) -> &P2_12 {
        &self.p2_12
    }
    #[doc = "0x134 - I/O configuration register for pin P2\\[13\\]"]
    #[inline(always)]
    pub const fn p2_13(&self) -> &P2_13 {
        &self.p2_13
    }
    #[doc = "0x138 - I/O configuration register for pin P2\\[14\\]"]
    #[inline(always)]
    pub const fn p2_14(&self) -> &P2_14 {
        &self.p2_14
    }
    #[doc = "0x13c - I/O configuration register for pin P2\\[15\\]"]
    #[inline(always)]
    pub const fn p2_15(&self) -> &P2_15 {
        &self.p2_15
    }
    #[doc = "0x140 - I/O configuration register for pin P2\\[16\\]"]
    #[inline(always)]
    pub const fn p2_16(&self) -> &P2_16 {
        &self.p2_16
    }
    #[doc = "0x144 - I/O configuration register for pin P2\\[17\\]"]
    #[inline(always)]
    pub const fn p2_17(&self) -> &P2_17 {
        &self.p2_17
    }
    #[doc = "0x148 - I/O configuration register for pin P2\\[18\\]"]
    #[inline(always)]
    pub const fn p2_18(&self) -> &P2_18 {
        &self.p2_18
    }
    #[doc = "0x14c - I/O configuration register for pin P2\\[19\\]"]
    #[inline(always)]
    pub const fn p2_19(&self) -> &P2_19 {
        &self.p2_19
    }
    #[doc = "0x150 - I/O configuration register for pin P2\\[20\\]"]
    #[inline(always)]
    pub const fn p2_20(&self) -> &P2_20 {
        &self.p2_20
    }
    #[doc = "0x154 - I/O configuration register for pin P2\\[21\\]"]
    #[inline(always)]
    pub const fn p2_21(&self) -> &P2_21 {
        &self.p2_21
    }
    #[doc = "0x158 - I/O configuration register for pin P2\\[22\\]"]
    #[inline(always)]
    pub const fn p2_22(&self) -> &P2_22 {
        &self.p2_22
    }
    #[doc = "0x15c - I/O configuration register for pin P2\\[23\\]"]
    #[inline(always)]
    pub const fn p2_23(&self) -> &P2_23 {
        &self.p2_23
    }
    #[doc = "0x160 - I/O configuration register for pin P2\\[24\\]"]
    #[inline(always)]
    pub const fn p2_24(&self) -> &P2_24 {
        &self.p2_24
    }
    #[doc = "0x164 - I/O configuration register for pin P2\\[25\\]"]
    #[inline(always)]
    pub const fn p2_25(&self) -> &P2_25 {
        &self.p2_25
    }
    #[doc = "0x168 - I/O configuration register for pin P2\\[26\\]"]
    #[inline(always)]
    pub const fn p2_26(&self) -> &P2_26 {
        &self.p2_26
    }
    #[doc = "0x16c - I/O configuration register for pin P2\\[27\\]"]
    #[inline(always)]
    pub const fn p2_27(&self) -> &P2_27 {
        &self.p2_27
    }
    #[doc = "0x170 - I/O configuration register for pin P2\\[28\\]"]
    #[inline(always)]
    pub const fn p2_28(&self) -> &P2_28 {
        &self.p2_28
    }
    #[doc = "0x174 - I/O configuration register for pin P2\\[29\\]"]
    #[inline(always)]
    pub const fn p2_29(&self) -> &P2_29 {
        &self.p2_29
    }
    #[doc = "0x178 - I/O configuration register for pin P2\\[30\\]"]
    #[inline(always)]
    pub const fn p2_30(&self) -> &P2_30 {
        &self.p2_30
    }
    #[doc = "0x17c - I/O configuration register for pin P2\\[31\\]"]
    #[inline(always)]
    pub const fn p2_31(&self) -> &P2_31 {
        &self.p2_31
    }
    #[doc = "0x180 - I/O configuration register for pin P3\\[0\\]"]
    #[inline(always)]
    pub const fn p3_0(&self) -> &P3_0 {
        &self.p3_0
    }
    #[doc = "0x184 - I/O configuration register for pin P3\\[1\\]"]
    #[inline(always)]
    pub const fn p3_1(&self) -> &P3_1 {
        &self.p3_1
    }
    #[doc = "0x188 - I/O configuration register for pin P3\\[2\\]"]
    #[inline(always)]
    pub const fn p3_2(&self) -> &P3_2 {
        &self.p3_2
    }
    #[doc = "0x18c - I/O configuration register for pin P3\\[3\\]"]
    #[inline(always)]
    pub const fn p3_3(&self) -> &P3_3 {
        &self.p3_3
    }
    #[doc = "0x190 - I/O configuration register for pin P3\\[4\\]"]
    #[inline(always)]
    pub const fn p3_4(&self) -> &P3_4 {
        &self.p3_4
    }
    #[doc = "0x194 - I/O configuration register for pin P3\\[5\\]"]
    #[inline(always)]
    pub const fn p3_5(&self) -> &P3_5 {
        &self.p3_5
    }
    #[doc = "0x198 - I/O configuration register for pin P3\\[6\\]"]
    #[inline(always)]
    pub const fn p3_6(&self) -> &P3_6 {
        &self.p3_6
    }
    #[doc = "0x19c - I/O configuration register for pin P3\\[7\\]"]
    #[inline(always)]
    pub const fn p3_7(&self) -> &P3_7 {
        &self.p3_7
    }
    #[doc = "0x1a0 - I/O configuration register for pin P3\\[8\\]"]
    #[inline(always)]
    pub const fn p3_8(&self) -> &P3_8 {
        &self.p3_8
    }
    #[doc = "0x1a4 - I/O configuration register for pin P3\\[9\\]"]
    #[inline(always)]
    pub const fn p3_9(&self) -> &P3_9 {
        &self.p3_9
    }
    #[doc = "0x1a8 - I/O configuration register for pin P3\\[10\\]"]
    #[inline(always)]
    pub const fn p3_10(&self) -> &P3_10 {
        &self.p3_10
    }
    #[doc = "0x1ac - I/O configuration register for pin P3\\[11\\]"]
    #[inline(always)]
    pub const fn p3_11(&self) -> &P3_11 {
        &self.p3_11
    }
    #[doc = "0x1b0 - I/O configuration register for pin P3\\[12\\]"]
    #[inline(always)]
    pub const fn p3_12(&self) -> &P3_12 {
        &self.p3_12
    }
    #[doc = "0x1b4 - I/O configuration register for pin P3\\[13\\]"]
    #[inline(always)]
    pub const fn p3_13(&self) -> &P3_13 {
        &self.p3_13
    }
    #[doc = "0x1b8 - I/O configuration register for pin P3\\[14\\]"]
    #[inline(always)]
    pub const fn p3_14(&self) -> &P3_14 {
        &self.p3_14
    }
    #[doc = "0x1bc - I/O configuration register for pin P3\\[15\\]"]
    #[inline(always)]
    pub const fn p3_15(&self) -> &P3_15 {
        &self.p3_15
    }
    #[doc = "0x1c0 - I/O configuration register for pin P3\\[16\\]"]
    #[inline(always)]
    pub const fn p3_16(&self) -> &P3_16 {
        &self.p3_16
    }
    #[doc = "0x1c4 - I/O configuration register for pin P3\\[17\\]"]
    #[inline(always)]
    pub const fn p3_17(&self) -> &P3_17 {
        &self.p3_17
    }
    #[doc = "0x1c8 - I/O configuration register for pin P3\\[18\\]"]
    #[inline(always)]
    pub const fn p3_18(&self) -> &P3_18 {
        &self.p3_18
    }
    #[doc = "0x1cc - I/O configuration register for pin P3\\[19\\]"]
    #[inline(always)]
    pub const fn p3_19(&self) -> &P3_19 {
        &self.p3_19
    }
    #[doc = "0x1d0 - I/O configuration register for pin P3\\[20\\]"]
    #[inline(always)]
    pub const fn p3_20(&self) -> &P3_20 {
        &self.p3_20
    }
    #[doc = "0x1d4 - I/O configuration register for pin P3\\[21\\]"]
    #[inline(always)]
    pub const fn p3_21(&self) -> &P3_21 {
        &self.p3_21
    }
    #[doc = "0x1d8 - I/O configuration register for pin P3\\[22\\]"]
    #[inline(always)]
    pub const fn p3_22(&self) -> &P3_22 {
        &self.p3_22
    }
    #[doc = "0x1dc - I/O configuration register for pin P3\\[23\\]"]
    #[inline(always)]
    pub const fn p3_23(&self) -> &P3_23 {
        &self.p3_23
    }
    #[doc = "0x1e0 - I/O configuration register for pin P3\\[24\\]"]
    #[inline(always)]
    pub const fn p3_24(&self) -> &P3_24 {
        &self.p3_24
    }
    #[doc = "0x1e4 - I/O configuration register for pin P3\\[25\\]"]
    #[inline(always)]
    pub const fn p3_25(&self) -> &P3_25 {
        &self.p3_25
    }
    #[doc = "0x1e8 - I/O configuration register for pin P3\\[26\\]"]
    #[inline(always)]
    pub const fn p3_26(&self) -> &P3_26 {
        &self.p3_26
    }
    #[doc = "0x1ec - I/O configuration register for pin P3\\[27\\]"]
    #[inline(always)]
    pub const fn p3_27(&self) -> &P3_27 {
        &self.p3_27
    }
    #[doc = "0x1f0 - I/O configuration register for pin P3\\[28\\]"]
    #[inline(always)]
    pub const fn p3_28(&self) -> &P3_28 {
        &self.p3_28
    }
    #[doc = "0x1f4 - I/O configuration register for pin P3\\[29\\]"]
    #[inline(always)]
    pub const fn p3_29(&self) -> &P3_29 {
        &self.p3_29
    }
    #[doc = "0x1f8 - I/O configuration register for pin P3\\[30\\]"]
    #[inline(always)]
    pub const fn p3_30(&self) -> &P3_30 {
        &self.p3_30
    }
    #[doc = "0x1fc - I/O configuration register for pin P3\\[31\\]"]
    #[inline(always)]
    pub const fn p3_31(&self) -> &P3_31 {
        &self.p3_31
    }
    #[doc = "0x200 - I/O configuration register for pin P4\\[0\\]"]
    #[inline(always)]
    pub const fn p4_0(&self) -> &P4_0 {
        &self.p4_0
    }
    #[doc = "0x204 - I/O configuration register for pin P4\\[1\\]"]
    #[inline(always)]
    pub const fn p4_1(&self) -> &P4_1 {
        &self.p4_1
    }
    #[doc = "0x208 - I/O configuration register for pin P4\\[2\\]"]
    #[inline(always)]
    pub const fn p4_2(&self) -> &P4_2 {
        &self.p4_2
    }
    #[doc = "0x20c - I/O configuration register for pin P4\\[3\\]"]
    #[inline(always)]
    pub const fn p4_3(&self) -> &P4_3 {
        &self.p4_3
    }
    #[doc = "0x210 - I/O configuration register for pin P4\\[4\\]"]
    #[inline(always)]
    pub const fn p4_4(&self) -> &P4_4 {
        &self.p4_4
    }
    #[doc = "0x214 - I/O configuration register for pin P4\\[5\\]"]
    #[inline(always)]
    pub const fn p4_5(&self) -> &P4_5 {
        &self.p4_5
    }
    #[doc = "0x218 - I/O configuration register for pin P4\\[6\\]"]
    #[inline(always)]
    pub const fn p4_6(&self) -> &P4_6 {
        &self.p4_6
    }
    #[doc = "0x21c - I/O configuration register for pin P4\\[7\\]"]
    #[inline(always)]
    pub const fn p4_7(&self) -> &P4_7 {
        &self.p4_7
    }
    #[doc = "0x220 - I/O configuration register for pin P4\\[8\\]"]
    #[inline(always)]
    pub const fn p4_8(&self) -> &P4_8 {
        &self.p4_8
    }
    #[doc = "0x224 - I/O configuration register for pin P4\\[9\\]"]
    #[inline(always)]
    pub const fn p4_9(&self) -> &P4_9 {
        &self.p4_9
    }
    #[doc = "0x228 - I/O configuration register for pin P4\\[10\\]"]
    #[inline(always)]
    pub const fn p4_10(&self) -> &P4_10 {
        &self.p4_10
    }
    #[doc = "0x22c - I/O configuration register for pin P4\\[11\\]"]
    #[inline(always)]
    pub const fn p4_11(&self) -> &P4_11 {
        &self.p4_11
    }
    #[doc = "0x230 - I/O configuration register for pin P4\\[12\\]"]
    #[inline(always)]
    pub const fn p4_12(&self) -> &P4_12 {
        &self.p4_12
    }
    #[doc = "0x234 - I/O configuration register for pin P4\\[13\\]"]
    #[inline(always)]
    pub const fn p4_13(&self) -> &P4_13 {
        &self.p4_13
    }
    #[doc = "0x238 - I/O configuration register for pin P4\\[14\\]"]
    #[inline(always)]
    pub const fn p4_14(&self) -> &P4_14 {
        &self.p4_14
    }
    #[doc = "0x23c - I/O configuration register for pin P4\\[15\\]"]
    #[inline(always)]
    pub const fn p4_15(&self) -> &P4_15 {
        &self.p4_15
    }
    #[doc = "0x240 - I/O configuration register for pin P4\\[16\\]"]
    #[inline(always)]
    pub const fn p4_16(&self) -> &P4_16 {
        &self.p4_16
    }
    #[doc = "0x244 - I/O configuration register for pin P4\\[17\\]"]
    #[inline(always)]
    pub const fn p4_17(&self) -> &P4_17 {
        &self.p4_17
    }
    #[doc = "0x248 - I/O configuration register for pin P4\\[18\\]"]
    #[inline(always)]
    pub const fn p4_18(&self) -> &P4_18 {
        &self.p4_18
    }
    #[doc = "0x24c - I/O configuration register for pin P4\\[19\\]"]
    #[inline(always)]
    pub const fn p4_19(&self) -> &P4_19 {
        &self.p4_19
    }
    #[doc = "0x250 - I/O configuration register for pin P4\\[20\\]"]
    #[inline(always)]
    pub const fn p4_20(&self) -> &P4_20 {
        &self.p4_20
    }
    #[doc = "0x254 - I/O configuration register for pin P4\\[21\\]"]
    #[inline(always)]
    pub const fn p4_21(&self) -> &P4_21 {
        &self.p4_21
    }
    #[doc = "0x258 - I/O configuration register for pin P4\\[22\\]"]
    #[inline(always)]
    pub const fn p4_22(&self) -> &P4_22 {
        &self.p4_22
    }
    #[doc = "0x25c - I/O configuration register for pin P4\\[23\\]"]
    #[inline(always)]
    pub const fn p4_23(&self) -> &P4_23 {
        &self.p4_23
    }
    #[doc = "0x260 - I/O configuration register for pin P4\\[24\\]"]
    #[inline(always)]
    pub const fn p4_24(&self) -> &P4_24 {
        &self.p4_24
    }
    #[doc = "0x264 - I/O configuration register for pin P4\\[25\\]"]
    #[inline(always)]
    pub const fn p4_25(&self) -> &P4_25 {
        &self.p4_25
    }
    #[doc = "0x268 - I/O configuration register for pin P4\\[26\\]"]
    #[inline(always)]
    pub const fn p4_26(&self) -> &P4_26 {
        &self.p4_26
    }
    #[doc = "0x26c - I/O configuration register for pin P4\\[27\\]"]
    #[inline(always)]
    pub const fn p4_27(&self) -> &P4_27 {
        &self.p4_27
    }
    #[doc = "0x270 - I/O configuration register for pin P4\\[28\\]"]
    #[inline(always)]
    pub const fn p4_28(&self) -> &P4_28 {
        &self.p4_28
    }
    #[doc = "0x274 - I/O configuration register for pin P4\\[29\\]"]
    #[inline(always)]
    pub const fn p4_29(&self) -> &P4_29 {
        &self.p4_29
    }
    #[doc = "0x278 - I/O configuration register for pin P4\\[30\\]"]
    #[inline(always)]
    pub const fn p4_30(&self) -> &P4_30 {
        &self.p4_30
    }
    #[doc = "0x27c - I/O configuration register for pin P4\\[31\\]"]
    #[inline(always)]
    pub const fn p4_31(&self) -> &P4_31 {
        &self.p4_31
    }
    #[doc = "0x280 - I/O configuration register for pin P5\\[0\\]"]
    #[inline(always)]
    pub const fn p5_0(&self) -> &P5_0 {
        &self.p5_0
    }
    #[doc = "0x284 - I/O configuration register for pin P5\\[1\\]"]
    #[inline(always)]
    pub const fn p5_1(&self) -> &P5_1 {
        &self.p5_1
    }
    #[doc = "0x288 - I/O configuration register for pin P5\\[2\\]"]
    #[inline(always)]
    pub const fn p5_2(&self) -> &P5_2 {
        &self.p5_2
    }
    #[doc = "0x28c - I/O configuration register for pin P5\\[3\\]"]
    #[inline(always)]
    pub const fn p5_3(&self) -> &P5_3 {
        &self.p5_3
    }
    #[doc = "0x290 - I/O configuration register for pin P5\\[4\\]"]
    #[inline(always)]
    pub const fn p5_4(&self) -> &P5_4 {
        &self.p5_4
    }
}
#[doc = "P0_0 (rw) register accessor: I/O configuration register for pin P0\\[0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_0`] module"]
pub type P0_0 = crate::Reg<p0_0::P0_0Spec>;
#[doc = "I/O configuration register for pin P0\\[0\\]"]
pub mod p0_0;
#[doc = "P0_1 (rw) register accessor: I/O configuration register for pin P0\\[1\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_1`] module"]
pub type P0_1 = crate::Reg<p0_1::P0_1Spec>;
#[doc = "I/O configuration register for pin P0\\[1\\]"]
pub mod p0_1;
#[doc = "P0_2 (rw) register accessor: I/O configuration register for pin P0\\[2\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_2`] module"]
pub type P0_2 = crate::Reg<p0_2::P0_2Spec>;
#[doc = "I/O configuration register for pin P0\\[2\\]"]
pub mod p0_2;
#[doc = "P0_3 (rw) register accessor: I/O configuration register for pin P0\\[3\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_3`] module"]
pub type P0_3 = crate::Reg<p0_3::P0_3Spec>;
#[doc = "I/O configuration register for pin P0\\[3\\]"]
pub mod p0_3;
#[doc = "P0_4 (rw) register accessor: I/O configuration register for pin P0\\[4\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_4`] module"]
pub type P0_4 = crate::Reg<p0_4::P0_4Spec>;
#[doc = "I/O configuration register for pin P0\\[4\\]"]
pub mod p0_4;
#[doc = "P0_5 (rw) register accessor: I/O configuration register for pin P0\\[5\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_5`] module"]
pub type P0_5 = crate::Reg<p0_5::P0_5Spec>;
#[doc = "I/O configuration register for pin P0\\[5\\]"]
pub mod p0_5;
#[doc = "P0_6 (rw) register accessor: I/O configuration register for pin P0\\[6\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_6`] module"]
pub type P0_6 = crate::Reg<p0_6::P0_6Spec>;
#[doc = "I/O configuration register for pin P0\\[6\\]"]
pub mod p0_6;
#[doc = "P0_7 (rw) register accessor: I/O configuration register for pin P0\\[7\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_7`] module"]
pub type P0_7 = crate::Reg<p0_7::P0_7Spec>;
#[doc = "I/O configuration register for pin P0\\[7\\]"]
pub mod p0_7;
#[doc = "P0_8 (rw) register accessor: I/O configuration register for pin P0\\[8\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_8`] module"]
pub type P0_8 = crate::Reg<p0_8::P0_8Spec>;
#[doc = "I/O configuration register for pin P0\\[8\\]"]
pub mod p0_8;
#[doc = "P0_9 (rw) register accessor: I/O configuration register for pin P0\\[9\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_9`] module"]
pub type P0_9 = crate::Reg<p0_9::P0_9Spec>;
#[doc = "I/O configuration register for pin P0\\[9\\]"]
pub mod p0_9;
#[doc = "P0_10 (rw) register accessor: I/O configuration register for pin P0\\[10\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_10`] module"]
pub type P0_10 = crate::Reg<p0_10::P0_10Spec>;
#[doc = "I/O configuration register for pin P0\\[10\\]"]
pub mod p0_10;
#[doc = "P0_11 (rw) register accessor: I/O configuration register for pin P0\\[11\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_11`] module"]
pub type P0_11 = crate::Reg<p0_11::P0_11Spec>;
#[doc = "I/O configuration register for pin P0\\[11\\]"]
pub mod p0_11;
#[doc = "P0_12 (rw) register accessor: I/O configuration register for pin P0\\[12\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_12::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_12`] module"]
pub type P0_12 = crate::Reg<p0_12::P0_12Spec>;
#[doc = "I/O configuration register for pin P0\\[12\\]"]
pub mod p0_12;
#[doc = "P0_13 (rw) register accessor: I/O configuration register for pin P0\\[13\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_13::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_13`] module"]
pub type P0_13 = crate::Reg<p0_13::P0_13Spec>;
#[doc = "I/O configuration register for pin P0\\[13\\]"]
pub mod p0_13;
#[doc = "P0_14 (rw) register accessor: I/O configuration register for pin P0\\[14\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_14`] module"]
pub type P0_14 = crate::Reg<p0_14::P0_14Spec>;
#[doc = "I/O configuration register for pin P0\\[14\\]"]
pub mod p0_14;
#[doc = "P0_15 (rw) register accessor: I/O configuration register for pin P0\\[15\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_15`] module"]
pub type P0_15 = crate::Reg<p0_15::P0_15Spec>;
#[doc = "I/O configuration register for pin P0\\[15\\]"]
pub mod p0_15;
#[doc = "P0_16 (rw) register accessor: I/O configuration register for pin P0\\[16\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_16`] module"]
pub type P0_16 = crate::Reg<p0_16::P0_16Spec>;
#[doc = "I/O configuration register for pin P0\\[16\\]"]
pub mod p0_16;
#[doc = "P0_17 (rw) register accessor: I/O configuration register for pin P0\\[17\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_17`] module"]
pub type P0_17 = crate::Reg<p0_17::P0_17Spec>;
#[doc = "I/O configuration register for pin P0\\[17\\]"]
pub mod p0_17;
#[doc = "P0_18 (rw) register accessor: I/O configuration register for pin P0\\[18\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_18`] module"]
pub type P0_18 = crate::Reg<p0_18::P0_18Spec>;
#[doc = "I/O configuration register for pin P0\\[18\\]"]
pub mod p0_18;
#[doc = "P0_19 (rw) register accessor: I/O configuration register for pin P0\\[19\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_19`] module"]
pub type P0_19 = crate::Reg<p0_19::P0_19Spec>;
#[doc = "I/O configuration register for pin P0\\[19\\]"]
pub mod p0_19;
#[doc = "P0_20 (rw) register accessor: I/O configuration register for pin P0\\[20\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_20`] module"]
pub type P0_20 = crate::Reg<p0_20::P0_20Spec>;
#[doc = "I/O configuration register for pin P0\\[20\\]"]
pub mod p0_20;
#[doc = "P0_21 (rw) register accessor: I/O configuration register for pin P0\\[21\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_21`] module"]
pub type P0_21 = crate::Reg<p0_21::P0_21Spec>;
#[doc = "I/O configuration register for pin P0\\[21\\]"]
pub mod p0_21;
#[doc = "P0_22 (rw) register accessor: I/O configuration register for pin P0\\[22\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_22`] module"]
pub type P0_22 = crate::Reg<p0_22::P0_22Spec>;
#[doc = "I/O configuration register for pin P0\\[22\\]"]
pub mod p0_22;
#[doc = "P0_23 (rw) register accessor: I/O configuration register for pin P0\\[23\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_23::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_23`] module"]
pub type P0_23 = crate::Reg<p0_23::P0_23Spec>;
#[doc = "I/O configuration register for pin P0\\[23\\]"]
pub mod p0_23;
#[doc = "P0_24 (rw) register accessor: I/O configuration register for pin P0\\[24\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_24::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_24`] module"]
pub type P0_24 = crate::Reg<p0_24::P0_24Spec>;
#[doc = "I/O configuration register for pin P0\\[24\\]"]
pub mod p0_24;
#[doc = "P0_25 (rw) register accessor: I/O configuration register for pin P0\\[25\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_25::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_25`] module"]
pub type P0_25 = crate::Reg<p0_25::P0_25Spec>;
#[doc = "I/O configuration register for pin P0\\[25\\]"]
pub mod p0_25;
#[doc = "P0_26 (rw) register accessor: I/O configuration register for pin P0\\[26\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_26::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_26`] module"]
pub type P0_26 = crate::Reg<p0_26::P0_26Spec>;
#[doc = "I/O configuration register for pin P0\\[26\\]"]
pub mod p0_26;
#[doc = "P0_27 (rw) register accessor: I/O configuration register for pin P0\\[27\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_27`] module"]
pub type P0_27 = crate::Reg<p0_27::P0_27Spec>;
#[doc = "I/O configuration register for pin P0\\[27\\]"]
pub mod p0_27;
#[doc = "P0_28 (rw) register accessor: I/O configuration register for pin P0\\[28\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_28`] module"]
pub type P0_28 = crate::Reg<p0_28::P0_28Spec>;
#[doc = "I/O configuration register for pin P0\\[28\\]"]
pub mod p0_28;
#[doc = "P0_29 (rw) register accessor: I/O configuration register for pin P0\\[29\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_29`] module"]
pub type P0_29 = crate::Reg<p0_29::P0_29Spec>;
#[doc = "I/O configuration register for pin P0\\[29\\]"]
pub mod p0_29;
#[doc = "P0_30 (rw) register accessor: I/O configuration register for pin P0\\[30\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_30`] module"]
pub type P0_30 = crate::Reg<p0_30::P0_30Spec>;
#[doc = "I/O configuration register for pin P0\\[30\\]"]
pub mod p0_30;
#[doc = "P0_31 (rw) register accessor: I/O configuration register for pin P0\\[31\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0_31`] module"]
pub type P0_31 = crate::Reg<p0_31::P0_31Spec>;
#[doc = "I/O configuration register for pin P0\\[31\\]"]
pub mod p0_31;
#[doc = "P1_0 (rw) register accessor: I/O configuration register for pin P1\\[0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_0`] module"]
pub type P1_0 = crate::Reg<p1_0::P1_0Spec>;
#[doc = "I/O configuration register for pin P1\\[0\\]"]
pub mod p1_0;
#[doc = "P1_1 (rw) register accessor: I/O configuration register for pin P1\\[1\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_1`] module"]
pub type P1_1 = crate::Reg<p1_1::P1_1Spec>;
#[doc = "I/O configuration register for pin P1\\[1\\]"]
pub mod p1_1;
#[doc = "P1_2 (rw) register accessor: I/O configuration register for pin P1\\[2\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_2`] module"]
pub type P1_2 = crate::Reg<p1_2::P1_2Spec>;
#[doc = "I/O configuration register for pin P1\\[2\\]"]
pub mod p1_2;
#[doc = "P1_3 (rw) register accessor: I/O configuration register for pin P1\\[3\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_3`] module"]
pub type P1_3 = crate::Reg<p1_3::P1_3Spec>;
#[doc = "I/O configuration register for pin P1\\[3\\]"]
pub mod p1_3;
#[doc = "P1_4 (rw) register accessor: I/O configuration register for pin P1\\[4\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_4`] module"]
pub type P1_4 = crate::Reg<p1_4::P1_4Spec>;
#[doc = "I/O configuration register for pin P1\\[4\\]"]
pub mod p1_4;
#[doc = "P1_5 (rw) register accessor: I/O configuration register for pin P1\\[5\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_5`] module"]
pub type P1_5 = crate::Reg<p1_5::P1_5Spec>;
#[doc = "I/O configuration register for pin P1\\[5\\]"]
pub mod p1_5;
#[doc = "P1_6 (rw) register accessor: I/O configuration register for pin P1\\[6\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_6`] module"]
pub type P1_6 = crate::Reg<p1_6::P1_6Spec>;
#[doc = "I/O configuration register for pin P1\\[6\\]"]
pub mod p1_6;
#[doc = "P1_7 (rw) register accessor: I/O configuration register for pin P1\\[7\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_7`] module"]
pub type P1_7 = crate::Reg<p1_7::P1_7Spec>;
#[doc = "I/O configuration register for pin P1\\[7\\]"]
pub mod p1_7;
#[doc = "P1_8 (rw) register accessor: I/O configuration register for pin P1\\[8\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_8`] module"]
pub type P1_8 = crate::Reg<p1_8::P1_8Spec>;
#[doc = "I/O configuration register for pin P1\\[8\\]"]
pub mod p1_8;
#[doc = "P1_9 (rw) register accessor: I/O configuration register for pin P1\\[9\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_9`] module"]
pub type P1_9 = crate::Reg<p1_9::P1_9Spec>;
#[doc = "I/O configuration register for pin P1\\[9\\]"]
pub mod p1_9;
#[doc = "P1_10 (rw) register accessor: I/O configuration register for pin P1\\[10\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_10`] module"]
pub type P1_10 = crate::Reg<p1_10::P1_10Spec>;
#[doc = "I/O configuration register for pin P1\\[10\\]"]
pub mod p1_10;
#[doc = "P1_11 (rw) register accessor: I/O configuration register for pin P1\\[11\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_11`] module"]
pub type P1_11 = crate::Reg<p1_11::P1_11Spec>;
#[doc = "I/O configuration register for pin P1\\[11\\]"]
pub mod p1_11;
#[doc = "P1_12 (rw) register accessor: I/O configuration register for pin P1\\[12\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_12`] module"]
pub type P1_12 = crate::Reg<p1_12::P1_12Spec>;
#[doc = "I/O configuration register for pin P1\\[12\\]"]
pub mod p1_12;
#[doc = "P1_13 (rw) register accessor: I/O configuration register for pin P1\\[13\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_13`] module"]
pub type P1_13 = crate::Reg<p1_13::P1_13Spec>;
#[doc = "I/O configuration register for pin P1\\[13\\]"]
pub mod p1_13;
#[doc = "P1_14 (rw) register accessor: I/O configuration register for pin P1\\[14\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_14`] module"]
pub type P1_14 = crate::Reg<p1_14::P1_14Spec>;
#[doc = "I/O configuration register for pin P1\\[14\\]"]
pub mod p1_14;
#[doc = "P1_15 (rw) register accessor: I/O configuration register for pin P1\\[15\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_15`] module"]
pub type P1_15 = crate::Reg<p1_15::P1_15Spec>;
#[doc = "I/O configuration register for pin P1\\[15\\]"]
pub mod p1_15;
#[doc = "P1_16 (rw) register accessor: I/O configuration register for pin P1\\[16\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_16`] module"]
pub type P1_16 = crate::Reg<p1_16::P1_16Spec>;
#[doc = "I/O configuration register for pin P1\\[16\\]"]
pub mod p1_16;
#[doc = "P1_17 (rw) register accessor: I/O configuration register for pin P1\\[17\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_17`] module"]
pub type P1_17 = crate::Reg<p1_17::P1_17Spec>;
#[doc = "I/O configuration register for pin P1\\[17\\]"]
pub mod p1_17;
#[doc = "P1_18 (rw) register accessor: I/O configuration register for pin P1\\[18\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_18`] module"]
pub type P1_18 = crate::Reg<p1_18::P1_18Spec>;
#[doc = "I/O configuration register for pin P1\\[18\\]"]
pub mod p1_18;
#[doc = "P1_19 (rw) register accessor: I/O configuration register for pin P1\\[19\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_19`] module"]
pub type P1_19 = crate::Reg<p1_19::P1_19Spec>;
#[doc = "I/O configuration register for pin P1\\[19\\]"]
pub mod p1_19;
#[doc = "P1_20 (rw) register accessor: I/O configuration register for pin P1\\[20\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_20`] module"]
pub type P1_20 = crate::Reg<p1_20::P1_20Spec>;
#[doc = "I/O configuration register for pin P1\\[20\\]"]
pub mod p1_20;
#[doc = "P1_21 (rw) register accessor: I/O configuration register for pin P1\\[21\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_21`] module"]
pub type P1_21 = crate::Reg<p1_21::P1_21Spec>;
#[doc = "I/O configuration register for pin P1\\[21\\]"]
pub mod p1_21;
#[doc = "P1_22 (rw) register accessor: I/O configuration register for pin P1\\[22\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_22`] module"]
pub type P1_22 = crate::Reg<p1_22::P1_22Spec>;
#[doc = "I/O configuration register for pin P1\\[22\\]"]
pub mod p1_22;
#[doc = "P1_23 (rw) register accessor: I/O configuration register for pin P1\\[23\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_23`] module"]
pub type P1_23 = crate::Reg<p1_23::P1_23Spec>;
#[doc = "I/O configuration register for pin P1\\[23\\]"]
pub mod p1_23;
#[doc = "P1_24 (rw) register accessor: I/O configuration register for pin P1\\[24\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_24`] module"]
pub type P1_24 = crate::Reg<p1_24::P1_24Spec>;
#[doc = "I/O configuration register for pin P1\\[24\\]"]
pub mod p1_24;
#[doc = "P1_25 (rw) register accessor: I/O configuration register for pin P1\\[25\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_25`] module"]
pub type P1_25 = crate::Reg<p1_25::P1_25Spec>;
#[doc = "I/O configuration register for pin P1\\[25\\]"]
pub mod p1_25;
#[doc = "P1_26 (rw) register accessor: I/O configuration register for pin P1\\[26\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_26`] module"]
pub type P1_26 = crate::Reg<p1_26::P1_26Spec>;
#[doc = "I/O configuration register for pin P1\\[26\\]"]
pub mod p1_26;
#[doc = "P1_27 (rw) register accessor: I/O configuration register for pin P1\\[27\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_27`] module"]
pub type P1_27 = crate::Reg<p1_27::P1_27Spec>;
#[doc = "I/O configuration register for pin P1\\[27\\]"]
pub mod p1_27;
#[doc = "P1_28 (rw) register accessor: I/O configuration register for pin P1\\[28\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_28`] module"]
pub type P1_28 = crate::Reg<p1_28::P1_28Spec>;
#[doc = "I/O configuration register for pin P1\\[28\\]"]
pub mod p1_28;
#[doc = "P1_29 (rw) register accessor: I/O configuration register for pin P1\\[29\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_29`] module"]
pub type P1_29 = crate::Reg<p1_29::P1_29Spec>;
#[doc = "I/O configuration register for pin P1\\[29\\]"]
pub mod p1_29;
#[doc = "P1_30 (rw) register accessor: I/O configuration register for pin P1\\[30\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_30::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_30`] module"]
pub type P1_30 = crate::Reg<p1_30::P1_30Spec>;
#[doc = "I/O configuration register for pin P1\\[30\\]"]
pub mod p1_30;
#[doc = "P1_31 (rw) register accessor: I/O configuration register for pin P1\\[31\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_31::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1_31`] module"]
pub type P1_31 = crate::Reg<p1_31::P1_31Spec>;
#[doc = "I/O configuration register for pin P1\\[31\\]"]
pub mod p1_31;
#[doc = "P2_0 (rw) register accessor: I/O configuration register for pin P2\\[0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_0`] module"]
pub type P2_0 = crate::Reg<p2_0::P2_0Spec>;
#[doc = "I/O configuration register for pin P2\\[0\\]"]
pub mod p2_0;
#[doc = "P2_1 (rw) register accessor: I/O configuration register for pin P2\\[1\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_1`] module"]
pub type P2_1 = crate::Reg<p2_1::P2_1Spec>;
#[doc = "I/O configuration register for pin P2\\[1\\]"]
pub mod p2_1;
#[doc = "P2_2 (rw) register accessor: I/O configuration register for pin P2\\[2\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_2`] module"]
pub type P2_2 = crate::Reg<p2_2::P2_2Spec>;
#[doc = "I/O configuration register for pin P2\\[2\\]"]
pub mod p2_2;
#[doc = "P2_3 (rw) register accessor: I/O configuration register for pin P2\\[3\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_3`] module"]
pub type P2_3 = crate::Reg<p2_3::P2_3Spec>;
#[doc = "I/O configuration register for pin P2\\[3\\]"]
pub mod p2_3;
#[doc = "P2_4 (rw) register accessor: I/O configuration register for pin P2\\[4\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_4`] module"]
pub type P2_4 = crate::Reg<p2_4::P2_4Spec>;
#[doc = "I/O configuration register for pin P2\\[4\\]"]
pub mod p2_4;
#[doc = "P2_5 (rw) register accessor: I/O configuration register for pin P2\\[5\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_5`] module"]
pub type P2_5 = crate::Reg<p2_5::P2_5Spec>;
#[doc = "I/O configuration register for pin P2\\[5\\]"]
pub mod p2_5;
#[doc = "P2_6 (rw) register accessor: I/O configuration register for pin P2\\[6\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_6`] module"]
pub type P2_6 = crate::Reg<p2_6::P2_6Spec>;
#[doc = "I/O configuration register for pin P2\\[6\\]"]
pub mod p2_6;
#[doc = "P2_7 (rw) register accessor: I/O configuration register for pin P2\\[7\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_7`] module"]
pub type P2_7 = crate::Reg<p2_7::P2_7Spec>;
#[doc = "I/O configuration register for pin P2\\[7\\]"]
pub mod p2_7;
#[doc = "P2_8 (rw) register accessor: I/O configuration register for pin P2\\[8\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_8`] module"]
pub type P2_8 = crate::Reg<p2_8::P2_8Spec>;
#[doc = "I/O configuration register for pin P2\\[8\\]"]
pub mod p2_8;
#[doc = "P2_9 (rw) register accessor: I/O configuration register for pin P2\\[9\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_9`] module"]
pub type P2_9 = crate::Reg<p2_9::P2_9Spec>;
#[doc = "I/O configuration register for pin P2\\[9\\]"]
pub mod p2_9;
#[doc = "P2_10 (rw) register accessor: I/O configuration register for pin P2\\[10\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_10::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_10`] module"]
pub type P2_10 = crate::Reg<p2_10::P2_10Spec>;
#[doc = "I/O configuration register for pin P2\\[10\\]"]
pub mod p2_10;
#[doc = "P2_11 (rw) register accessor: I/O configuration register for pin P2\\[11\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_11::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_11`] module"]
pub type P2_11 = crate::Reg<p2_11::P2_11Spec>;
#[doc = "I/O configuration register for pin P2\\[11\\]"]
pub mod p2_11;
#[doc = "P2_12 (rw) register accessor: I/O configuration register for pin P2\\[12\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_12::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_12`] module"]
pub type P2_12 = crate::Reg<p2_12::P2_12Spec>;
#[doc = "I/O configuration register for pin P2\\[12\\]"]
pub mod p2_12;
#[doc = "P2_13 (rw) register accessor: I/O configuration register for pin P2\\[13\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_13::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_13`] module"]
pub type P2_13 = crate::Reg<p2_13::P2_13Spec>;
#[doc = "I/O configuration register for pin P2\\[13\\]"]
pub mod p2_13;
#[doc = "P2_14 (rw) register accessor: I/O configuration register for pin P2\\[14\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_14`] module"]
pub type P2_14 = crate::Reg<p2_14::P2_14Spec>;
#[doc = "I/O configuration register for pin P2\\[14\\]"]
pub mod p2_14;
#[doc = "P2_15 (rw) register accessor: I/O configuration register for pin P2\\[15\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_15`] module"]
pub type P2_15 = crate::Reg<p2_15::P2_15Spec>;
#[doc = "I/O configuration register for pin P2\\[15\\]"]
pub mod p2_15;
#[doc = "P2_16 (rw) register accessor: I/O configuration register for pin P2\\[16\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_16`] module"]
pub type P2_16 = crate::Reg<p2_16::P2_16Spec>;
#[doc = "I/O configuration register for pin P2\\[16\\]"]
pub mod p2_16;
#[doc = "P2_17 (rw) register accessor: I/O configuration register for pin P2\\[17\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_17`] module"]
pub type P2_17 = crate::Reg<p2_17::P2_17Spec>;
#[doc = "I/O configuration register for pin P2\\[17\\]"]
pub mod p2_17;
#[doc = "P2_18 (rw) register accessor: I/O configuration register for pin P2\\[18\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_18`] module"]
pub type P2_18 = crate::Reg<p2_18::P2_18Spec>;
#[doc = "I/O configuration register for pin P2\\[18\\]"]
pub mod p2_18;
#[doc = "P2_19 (rw) register accessor: I/O configuration register for pin P2\\[19\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_19`] module"]
pub type P2_19 = crate::Reg<p2_19::P2_19Spec>;
#[doc = "I/O configuration register for pin P2\\[19\\]"]
pub mod p2_19;
#[doc = "P2_20 (rw) register accessor: I/O configuration register for pin P2\\[20\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_20`] module"]
pub type P2_20 = crate::Reg<p2_20::P2_20Spec>;
#[doc = "I/O configuration register for pin P2\\[20\\]"]
pub mod p2_20;
#[doc = "P2_21 (rw) register accessor: I/O configuration register for pin P2\\[21\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_21`] module"]
pub type P2_21 = crate::Reg<p2_21::P2_21Spec>;
#[doc = "I/O configuration register for pin P2\\[21\\]"]
pub mod p2_21;
#[doc = "P2_22 (rw) register accessor: I/O configuration register for pin P2\\[22\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_22`] module"]
pub type P2_22 = crate::Reg<p2_22::P2_22Spec>;
#[doc = "I/O configuration register for pin P2\\[22\\]"]
pub mod p2_22;
#[doc = "P2_23 (rw) register accessor: I/O configuration register for pin P2\\[23\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_23`] module"]
pub type P2_23 = crate::Reg<p2_23::P2_23Spec>;
#[doc = "I/O configuration register for pin P2\\[23\\]"]
pub mod p2_23;
#[doc = "P2_24 (rw) register accessor: I/O configuration register for pin P2\\[24\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_24`] module"]
pub type P2_24 = crate::Reg<p2_24::P2_24Spec>;
#[doc = "I/O configuration register for pin P2\\[24\\]"]
pub mod p2_24;
#[doc = "P2_25 (rw) register accessor: I/O configuration register for pin P2\\[25\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_25`] module"]
pub type P2_25 = crate::Reg<p2_25::P2_25Spec>;
#[doc = "I/O configuration register for pin P2\\[25\\]"]
pub mod p2_25;
#[doc = "P2_26 (rw) register accessor: I/O configuration register for pin P2\\[26\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_26`] module"]
pub type P2_26 = crate::Reg<p2_26::P2_26Spec>;
#[doc = "I/O configuration register for pin P2\\[26\\]"]
pub mod p2_26;
#[doc = "P2_27 (rw) register accessor: I/O configuration register for pin P2\\[27\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_27`] module"]
pub type P2_27 = crate::Reg<p2_27::P2_27Spec>;
#[doc = "I/O configuration register for pin P2\\[27\\]"]
pub mod p2_27;
#[doc = "P2_28 (rw) register accessor: I/O configuration register for pin P2\\[28\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_28`] module"]
pub type P2_28 = crate::Reg<p2_28::P2_28Spec>;
#[doc = "I/O configuration register for pin P2\\[28\\]"]
pub mod p2_28;
#[doc = "P2_29 (rw) register accessor: I/O configuration register for pin P2\\[29\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_29`] module"]
pub type P2_29 = crate::Reg<p2_29::P2_29Spec>;
#[doc = "I/O configuration register for pin P2\\[29\\]"]
pub mod p2_29;
#[doc = "P2_30 (rw) register accessor: I/O configuration register for pin P2\\[30\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_30`] module"]
pub type P2_30 = crate::Reg<p2_30::P2_30Spec>;
#[doc = "I/O configuration register for pin P2\\[30\\]"]
pub mod p2_30;
#[doc = "P2_31 (rw) register accessor: I/O configuration register for pin P2\\[31\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2_31`] module"]
pub type P2_31 = crate::Reg<p2_31::P2_31Spec>;
#[doc = "I/O configuration register for pin P2\\[31\\]"]
pub mod p2_31;
#[doc = "P3_0 (rw) register accessor: I/O configuration register for pin P3\\[0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_0`] module"]
pub type P3_0 = crate::Reg<p3_0::P3_0Spec>;
#[doc = "I/O configuration register for pin P3\\[0\\]"]
pub mod p3_0;
#[doc = "P3_1 (rw) register accessor: I/O configuration register for pin P3\\[1\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_1`] module"]
pub type P3_1 = crate::Reg<p3_1::P3_1Spec>;
#[doc = "I/O configuration register for pin P3\\[1\\]"]
pub mod p3_1;
#[doc = "P3_2 (rw) register accessor: I/O configuration register for pin P3\\[2\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_2`] module"]
pub type P3_2 = crate::Reg<p3_2::P3_2Spec>;
#[doc = "I/O configuration register for pin P3\\[2\\]"]
pub mod p3_2;
#[doc = "P3_3 (rw) register accessor: I/O configuration register for pin P3\\[3\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_3`] module"]
pub type P3_3 = crate::Reg<p3_3::P3_3Spec>;
#[doc = "I/O configuration register for pin P3\\[3\\]"]
pub mod p3_3;
#[doc = "P3_4 (rw) register accessor: I/O configuration register for pin P3\\[4\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_4`] module"]
pub type P3_4 = crate::Reg<p3_4::P3_4Spec>;
#[doc = "I/O configuration register for pin P3\\[4\\]"]
pub mod p3_4;
#[doc = "P3_5 (rw) register accessor: I/O configuration register for pin P3\\[5\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_5`] module"]
pub type P3_5 = crate::Reg<p3_5::P3_5Spec>;
#[doc = "I/O configuration register for pin P3\\[5\\]"]
pub mod p3_5;
#[doc = "P3_6 (rw) register accessor: I/O configuration register for pin P3\\[6\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_6`] module"]
pub type P3_6 = crate::Reg<p3_6::P3_6Spec>;
#[doc = "I/O configuration register for pin P3\\[6\\]"]
pub mod p3_6;
#[doc = "P3_7 (rw) register accessor: I/O configuration register for pin P3\\[7\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_7`] module"]
pub type P3_7 = crate::Reg<p3_7::P3_7Spec>;
#[doc = "I/O configuration register for pin P3\\[7\\]"]
pub mod p3_7;
#[doc = "P3_8 (rw) register accessor: I/O configuration register for pin P3\\[8\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_8`] module"]
pub type P3_8 = crate::Reg<p3_8::P3_8Spec>;
#[doc = "I/O configuration register for pin P3\\[8\\]"]
pub mod p3_8;
#[doc = "P3_9 (rw) register accessor: I/O configuration register for pin P3\\[9\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_9`] module"]
pub type P3_9 = crate::Reg<p3_9::P3_9Spec>;
#[doc = "I/O configuration register for pin P3\\[9\\]"]
pub mod p3_9;
#[doc = "P3_10 (rw) register accessor: I/O configuration register for pin P3\\[10\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_10`] module"]
pub type P3_10 = crate::Reg<p3_10::P3_10Spec>;
#[doc = "I/O configuration register for pin P3\\[10\\]"]
pub mod p3_10;
#[doc = "P3_11 (rw) register accessor: I/O configuration register for pin P3\\[11\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_11`] module"]
pub type P3_11 = crate::Reg<p3_11::P3_11Spec>;
#[doc = "I/O configuration register for pin P3\\[11\\]"]
pub mod p3_11;
#[doc = "P3_12 (rw) register accessor: I/O configuration register for pin P3\\[12\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_12`] module"]
pub type P3_12 = crate::Reg<p3_12::P3_12Spec>;
#[doc = "I/O configuration register for pin P3\\[12\\]"]
pub mod p3_12;
#[doc = "P3_13 (rw) register accessor: I/O configuration register for pin P3\\[13\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_13`] module"]
pub type P3_13 = crate::Reg<p3_13::P3_13Spec>;
#[doc = "I/O configuration register for pin P3\\[13\\]"]
pub mod p3_13;
#[doc = "P3_14 (rw) register accessor: I/O configuration register for pin P3\\[14\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_14`] module"]
pub type P3_14 = crate::Reg<p3_14::P3_14Spec>;
#[doc = "I/O configuration register for pin P3\\[14\\]"]
pub mod p3_14;
#[doc = "P3_15 (rw) register accessor: I/O configuration register for pin P3\\[15\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_15`] module"]
pub type P3_15 = crate::Reg<p3_15::P3_15Spec>;
#[doc = "I/O configuration register for pin P3\\[15\\]"]
pub mod p3_15;
#[doc = "P3_16 (rw) register accessor: I/O configuration register for pin P3\\[16\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_16`] module"]
pub type P3_16 = crate::Reg<p3_16::P3_16Spec>;
#[doc = "I/O configuration register for pin P3\\[16\\]"]
pub mod p3_16;
#[doc = "P3_17 (rw) register accessor: I/O configuration register for pin P3\\[17\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_17`] module"]
pub type P3_17 = crate::Reg<p3_17::P3_17Spec>;
#[doc = "I/O configuration register for pin P3\\[17\\]"]
pub mod p3_17;
#[doc = "P3_18 (rw) register accessor: I/O configuration register for pin P3\\[18\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_18`] module"]
pub type P3_18 = crate::Reg<p3_18::P3_18Spec>;
#[doc = "I/O configuration register for pin P3\\[18\\]"]
pub mod p3_18;
#[doc = "P3_19 (rw) register accessor: I/O configuration register for pin P3\\[19\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_19`] module"]
pub type P3_19 = crate::Reg<p3_19::P3_19Spec>;
#[doc = "I/O configuration register for pin P3\\[19\\]"]
pub mod p3_19;
#[doc = "P3_20 (rw) register accessor: I/O configuration register for pin P3\\[20\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_20`] module"]
pub type P3_20 = crate::Reg<p3_20::P3_20Spec>;
#[doc = "I/O configuration register for pin P3\\[20\\]"]
pub mod p3_20;
#[doc = "P3_21 (rw) register accessor: I/O configuration register for pin P3\\[21\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_21`] module"]
pub type P3_21 = crate::Reg<p3_21::P3_21Spec>;
#[doc = "I/O configuration register for pin P3\\[21\\]"]
pub mod p3_21;
#[doc = "P3_22 (rw) register accessor: I/O configuration register for pin P3\\[22\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_22`] module"]
pub type P3_22 = crate::Reg<p3_22::P3_22Spec>;
#[doc = "I/O configuration register for pin P3\\[22\\]"]
pub mod p3_22;
#[doc = "P3_23 (rw) register accessor: I/O configuration register for pin P3\\[23\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_23`] module"]
pub type P3_23 = crate::Reg<p3_23::P3_23Spec>;
#[doc = "I/O configuration register for pin P3\\[23\\]"]
pub mod p3_23;
#[doc = "P3_24 (rw) register accessor: I/O configuration register for pin P3\\[24\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_24`] module"]
pub type P3_24 = crate::Reg<p3_24::P3_24Spec>;
#[doc = "I/O configuration register for pin P3\\[24\\]"]
pub mod p3_24;
#[doc = "P3_25 (rw) register accessor: I/O configuration register for pin P3\\[25\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_25`] module"]
pub type P3_25 = crate::Reg<p3_25::P3_25Spec>;
#[doc = "I/O configuration register for pin P3\\[25\\]"]
pub mod p3_25;
#[doc = "P3_26 (rw) register accessor: I/O configuration register for pin P3\\[26\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_26`] module"]
pub type P3_26 = crate::Reg<p3_26::P3_26Spec>;
#[doc = "I/O configuration register for pin P3\\[26\\]"]
pub mod p3_26;
#[doc = "P3_27 (rw) register accessor: I/O configuration register for pin P3\\[27\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_27`] module"]
pub type P3_27 = crate::Reg<p3_27::P3_27Spec>;
#[doc = "I/O configuration register for pin P3\\[27\\]"]
pub mod p3_27;
#[doc = "P3_28 (rw) register accessor: I/O configuration register for pin P3\\[28\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_28`] module"]
pub type P3_28 = crate::Reg<p3_28::P3_28Spec>;
#[doc = "I/O configuration register for pin P3\\[28\\]"]
pub mod p3_28;
#[doc = "P3_29 (rw) register accessor: I/O configuration register for pin P3\\[29\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_29`] module"]
pub type P3_29 = crate::Reg<p3_29::P3_29Spec>;
#[doc = "I/O configuration register for pin P3\\[29\\]"]
pub mod p3_29;
#[doc = "P3_30 (rw) register accessor: I/O configuration register for pin P3\\[30\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_30`] module"]
pub type P3_30 = crate::Reg<p3_30::P3_30Spec>;
#[doc = "I/O configuration register for pin P3\\[30\\]"]
pub mod p3_30;
#[doc = "P3_31 (rw) register accessor: I/O configuration register for pin P3\\[31\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p3_31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3_31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3_31`] module"]
pub type P3_31 = crate::Reg<p3_31::P3_31Spec>;
#[doc = "I/O configuration register for pin P3\\[31\\]"]
pub mod p3_31;
#[doc = "P4_0 (rw) register accessor: I/O configuration register for pin P4\\[0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_0`] module"]
pub type P4_0 = crate::Reg<p4_0::P4_0Spec>;
#[doc = "I/O configuration register for pin P4\\[0\\]"]
pub mod p4_0;
#[doc = "P4_1 (rw) register accessor: I/O configuration register for pin P4\\[1\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_1`] module"]
pub type P4_1 = crate::Reg<p4_1::P4_1Spec>;
#[doc = "I/O configuration register for pin P4\\[1\\]"]
pub mod p4_1;
#[doc = "P4_2 (rw) register accessor: I/O configuration register for pin P4\\[2\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_2`] module"]
pub type P4_2 = crate::Reg<p4_2::P4_2Spec>;
#[doc = "I/O configuration register for pin P4\\[2\\]"]
pub mod p4_2;
#[doc = "P4_3 (rw) register accessor: I/O configuration register for pin P4\\[3\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_3`] module"]
pub type P4_3 = crate::Reg<p4_3::P4_3Spec>;
#[doc = "I/O configuration register for pin P4\\[3\\]"]
pub mod p4_3;
#[doc = "P4_4 (rw) register accessor: I/O configuration register for pin P4\\[4\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_4`] module"]
pub type P4_4 = crate::Reg<p4_4::P4_4Spec>;
#[doc = "I/O configuration register for pin P4\\[4\\]"]
pub mod p4_4;
#[doc = "P4_5 (rw) register accessor: I/O configuration register for pin P4\\[5\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_5`] module"]
pub type P4_5 = crate::Reg<p4_5::P4_5Spec>;
#[doc = "I/O configuration register for pin P4\\[5\\]"]
pub mod p4_5;
#[doc = "P4_6 (rw) register accessor: I/O configuration register for pin P4\\[6\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_6`] module"]
pub type P4_6 = crate::Reg<p4_6::P4_6Spec>;
#[doc = "I/O configuration register for pin P4\\[6\\]"]
pub mod p4_6;
#[doc = "P4_7 (rw) register accessor: I/O configuration register for pin P4\\[7\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_7`] module"]
pub type P4_7 = crate::Reg<p4_7::P4_7Spec>;
#[doc = "I/O configuration register for pin P4\\[7\\]"]
pub mod p4_7;
#[doc = "P4_8 (rw) register accessor: I/O configuration register for pin P4\\[8\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_8`] module"]
pub type P4_8 = crate::Reg<p4_8::P4_8Spec>;
#[doc = "I/O configuration register for pin P4\\[8\\]"]
pub mod p4_8;
#[doc = "P4_9 (rw) register accessor: I/O configuration register for pin P4\\[9\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_9`] module"]
pub type P4_9 = crate::Reg<p4_9::P4_9Spec>;
#[doc = "I/O configuration register for pin P4\\[9\\]"]
pub mod p4_9;
#[doc = "P4_10 (rw) register accessor: I/O configuration register for pin P4\\[10\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_10`] module"]
pub type P4_10 = crate::Reg<p4_10::P4_10Spec>;
#[doc = "I/O configuration register for pin P4\\[10\\]"]
pub mod p4_10;
#[doc = "P4_11 (rw) register accessor: I/O configuration register for pin P4\\[11\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_11`] module"]
pub type P4_11 = crate::Reg<p4_11::P4_11Spec>;
#[doc = "I/O configuration register for pin P4\\[11\\]"]
pub mod p4_11;
#[doc = "P4_12 (rw) register accessor: I/O configuration register for pin P4\\[12\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_12`] module"]
pub type P4_12 = crate::Reg<p4_12::P4_12Spec>;
#[doc = "I/O configuration register for pin P4\\[12\\]"]
pub mod p4_12;
#[doc = "P4_13 (rw) register accessor: I/O configuration register for pin P4\\[13\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_13`] module"]
pub type P4_13 = crate::Reg<p4_13::P4_13Spec>;
#[doc = "I/O configuration register for pin P4\\[13\\]"]
pub mod p4_13;
#[doc = "P4_14 (rw) register accessor: I/O configuration register for pin P4\\[14\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_14`] module"]
pub type P4_14 = crate::Reg<p4_14::P4_14Spec>;
#[doc = "I/O configuration register for pin P4\\[14\\]"]
pub mod p4_14;
#[doc = "P4_15 (rw) register accessor: I/O configuration register for pin P4\\[15\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_15`] module"]
pub type P4_15 = crate::Reg<p4_15::P4_15Spec>;
#[doc = "I/O configuration register for pin P4\\[15\\]"]
pub mod p4_15;
#[doc = "P4_16 (rw) register accessor: I/O configuration register for pin P4\\[16\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_16`] module"]
pub type P4_16 = crate::Reg<p4_16::P4_16Spec>;
#[doc = "I/O configuration register for pin P4\\[16\\]"]
pub mod p4_16;
#[doc = "P4_17 (rw) register accessor: I/O configuration register for pin P4\\[17\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_17`] module"]
pub type P4_17 = crate::Reg<p4_17::P4_17Spec>;
#[doc = "I/O configuration register for pin P4\\[17\\]"]
pub mod p4_17;
#[doc = "P4_18 (rw) register accessor: I/O configuration register for pin P4\\[18\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_18`] module"]
pub type P4_18 = crate::Reg<p4_18::P4_18Spec>;
#[doc = "I/O configuration register for pin P4\\[18\\]"]
pub mod p4_18;
#[doc = "P4_19 (rw) register accessor: I/O configuration register for pin P4\\[19\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_19`] module"]
pub type P4_19 = crate::Reg<p4_19::P4_19Spec>;
#[doc = "I/O configuration register for pin P4\\[19\\]"]
pub mod p4_19;
#[doc = "P4_20 (rw) register accessor: I/O configuration register for pin P4\\[20\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_20`] module"]
pub type P4_20 = crate::Reg<p4_20::P4_20Spec>;
#[doc = "I/O configuration register for pin P4\\[20\\]"]
pub mod p4_20;
#[doc = "P4_21 (rw) register accessor: I/O configuration register for pin P4\\[21\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_21`] module"]
pub type P4_21 = crate::Reg<p4_21::P4_21Spec>;
#[doc = "I/O configuration register for pin P4\\[21\\]"]
pub mod p4_21;
#[doc = "P4_22 (rw) register accessor: I/O configuration register for pin P4\\[22\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_22`] module"]
pub type P4_22 = crate::Reg<p4_22::P4_22Spec>;
#[doc = "I/O configuration register for pin P4\\[22\\]"]
pub mod p4_22;
#[doc = "P4_23 (rw) register accessor: I/O configuration register for pin P4\\[23\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_23`] module"]
pub type P4_23 = crate::Reg<p4_23::P4_23Spec>;
#[doc = "I/O configuration register for pin P4\\[23\\]"]
pub mod p4_23;
#[doc = "P4_24 (rw) register accessor: I/O configuration register for pin P4\\[24\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_24`] module"]
pub type P4_24 = crate::Reg<p4_24::P4_24Spec>;
#[doc = "I/O configuration register for pin P4\\[24\\]"]
pub mod p4_24;
#[doc = "P4_25 (rw) register accessor: I/O configuration register for pin P4\\[25\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_25`] module"]
pub type P4_25 = crate::Reg<p4_25::P4_25Spec>;
#[doc = "I/O configuration register for pin P4\\[25\\]"]
pub mod p4_25;
#[doc = "P4_26 (rw) register accessor: I/O configuration register for pin P4\\[26\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_26`] module"]
pub type P4_26 = crate::Reg<p4_26::P4_26Spec>;
#[doc = "I/O configuration register for pin P4\\[26\\]"]
pub mod p4_26;
#[doc = "P4_27 (rw) register accessor: I/O configuration register for pin P4\\[27\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_27`] module"]
pub type P4_27 = crate::Reg<p4_27::P4_27Spec>;
#[doc = "I/O configuration register for pin P4\\[27\\]"]
pub mod p4_27;
#[doc = "P4_28 (rw) register accessor: I/O configuration register for pin P4\\[28\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_28`] module"]
pub type P4_28 = crate::Reg<p4_28::P4_28Spec>;
#[doc = "I/O configuration register for pin P4\\[28\\]"]
pub mod p4_28;
#[doc = "P4_29 (rw) register accessor: I/O configuration register for pin P4\\[29\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_29`] module"]
pub type P4_29 = crate::Reg<p4_29::P4_29Spec>;
#[doc = "I/O configuration register for pin P4\\[29\\]"]
pub mod p4_29;
#[doc = "P4_30 (rw) register accessor: I/O configuration register for pin P4\\[30\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_30`] module"]
pub type P4_30 = crate::Reg<p4_30::P4_30Spec>;
#[doc = "I/O configuration register for pin P4\\[30\\]"]
pub mod p4_30;
#[doc = "P4_31 (rw) register accessor: I/O configuration register for pin P4\\[31\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p4_31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4_31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4_31`] module"]
pub type P4_31 = crate::Reg<p4_31::P4_31Spec>;
#[doc = "I/O configuration register for pin P4\\[31\\]"]
pub mod p4_31;
#[doc = "P5_0 (rw) register accessor: I/O configuration register for pin P5\\[0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p5_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5_0`] module"]
pub type P5_0 = crate::Reg<p5_0::P5_0Spec>;
#[doc = "I/O configuration register for pin P5\\[0\\]"]
pub mod p5_0;
#[doc = "P5_1 (rw) register accessor: I/O configuration register for pin P5\\[1\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p5_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5_1`] module"]
pub type P5_1 = crate::Reg<p5_1::P5_1Spec>;
#[doc = "I/O configuration register for pin P5\\[1\\]"]
pub mod p5_1;
#[doc = "P5_2 (rw) register accessor: I/O configuration register for pin P5\\[2\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p5_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5_2`] module"]
pub type P5_2 = crate::Reg<p5_2::P5_2Spec>;
#[doc = "I/O configuration register for pin P5\\[2\\]"]
pub mod p5_2;
#[doc = "P5_3 (rw) register accessor: I/O configuration register for pin P5\\[3\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p5_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5_3`] module"]
pub type P5_3 = crate::Reg<p5_3::P5_3Spec>;
#[doc = "I/O configuration register for pin P5\\[3\\]"]
pub mod p5_3;
#[doc = "P5_4 (rw) register accessor: I/O configuration register for pin P5\\[4\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p5_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5_4`] module"]
pub type P5_4 = crate::Reg<p5_4::P5_4Spec>;
#[doc = "I/O configuration register for pin P5\\[4\\]"]
pub mod p5_4;
