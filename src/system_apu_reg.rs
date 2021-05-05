use super :: apu :: *;
use super :: system :: *;

pub const APU_PULSE_1_OFFSET: usize = 0x00;
pub const APU_PULSE_2_OFFSET: usize = 0x04;
pub const APU_TRIANGLE_OFFSET: usize = 0x08;
pub const APU_NOISE_OFFSET: usize = 0x0c;
pub const APU_DMC_OFFSET: usize = 0x10;
pub const APU_STATUS_OFFSET: usize = 0x15;
pub const APU_FRAMECOUNTER_OFFSET: usize = 0x15;

/// APU & I / O (PAD) Register Implement
/// APU only (DMA does not use system_ppu_reg.rs, pad does not use register variables)
/// If fixed-point arithmetic is included, calculate it with a separate function (it doesn't matter if it just returns a constant)
/// I'm worried about copying the structure, so if playback is disabled, return None from the beginning
impl System {
    /// Get the square wave settings
    /// `index` --0 or 1
    pub fn read_apu_pulse_config (&self, index: u8)-> Option <PulseSound> {
        let mut dst = PulseSound :: default ();
        debug_assert! (index <2);
        // Return immediately if playback is disabled
        if! self.read_apu_is_enable_pulse (index) {
            return None;
        }
        // Switch base address with pulse 1/2
        let base_offset = if index == 0 {
            APU_PULSE_1_OFFSET
        } else {
            APU_PULSE_2_OFFSET
        };
        // Just read in order and decide the value
        dst.duty_cycle = match (self.io_reg [base_offset + 0] >> 6) & 0x03 {
            0 => PulseDutyCycle :: Duty12_5,
            1 => PulseDutyCycle :: Duty25_0,
            2 => PulseDutyCycle :: Duty50_0,
            3 => PulseDutyCycle :: Duty75_0,
            _ => panic! ("Invalid pulse duty_cycle: {}", self.io_reg [base_offset + 0]),
        };
        // $ 4000 DDLCVVVV
        dst.is_length_counter_halt = (self.io_reg [base_offset + 0] & 0x20) == 0x20;
        dst.is_constant_volume = (self.io_reg [base_offset + 0] & 0x10) == 0x10;
        dst.volume = self.io_reg [base_offset + 0] & 0x0f;
        // $ 4001 EPPPNSSS
        dst.is_sweep_enable = (self.io_reg [base_offset + 1] & 0x80) == 0x80;
        dst.sweep_period = (self.io_reg [base_offset + 1] & 0x70) >> 4;
        dst.is_sweep_negative = (self.io_reg [base_offset + 1] & 0x04) == 0x04;
        dst.sweep_shift = self.io_reg [base_offset + 1] & 0x07;
        // $ 4002 TTTTTTTT (timer lower)
        // $ 4003 LLLLLTTT (timer Upper)
        dst.timer_value = u16 :: from (self.io_reg [base_offset + 2])
            | (u16 :: from (self.io_reg [base_offset + 3] & 0x07) << 8);
        dst.length_counter_load = (self.io_reg [base_offset + 3] & 0xf8) >> 3;

        Some (dst)
    }

    /// Get the triangle wave settings
    pub fn read_apu_tri_config(&self) -> Option <TriangleSound> {
        // Return immediately if playback is disabled
        if! self.read_apu_is_enable_tri () {
            return None;
        }

        let mut dst = TriangleSound :: default ();
        // $ 4008 CRRRRRRRR
        dst.is_length_counter_halt = (self.io_reg [APU_TRIANGLE_OFFSET + 0] & 0x80) == 0x80;
        dst.counter_load = self.io_reg [APU_TRIANGLE_OFFSET + 0] & 0x7f;
        // $ 400a TTTTTTTT (timer lower)
        // $ 400b LLLLLTTT (timer upper)
        dst.timer_value = u16 :: from (self.io_reg [APU_TRIANGLE_OFFSET + 2])
            | (u16 :: from (self.io_reg [APU_TRIANGLE_OFFSET + 3] & 0x07) << 8);
        dst.length_counter_load = (self.io_reg [APU_TRIANGLE_OFFSET + 3] & 0xf8) >> 3;

        Some (dst)
    }

    /// Get the noise wave settings
    pub fn read_apu_noise_config(&self) -> Option <NoiseSound> {
        // Return immediately if playback is disabled
        if! self.read_apu_is_enable_noise () {
            return None;
        }
        let mut dst = NoiseSound :: default ();
        // $ 400c --LCVVVV
        dst.is_length_counter_halt = (self.io_reg [APU_NOISE_OFFSET + 0] & 0x20) == 0x20;
        dst.is_constant_volume = (self.io_reg [APU_NOISE_OFFSET + 0] & 0x10) == 0x10;
        dst.volume = self.io_reg [APU_NOISE_OFFSET + 0] & 0x0f;
        // $ 400E L --- PPPP
        dst.is_noise_type_loop = (self.io_reg [APU_NOISE_OFFSET + 2] & 0x80) == 0x80;
        dst.noise_period = self.io_reg [APU_NOISE_OFFSET + 2] & 0x0f;
        // $ 400F LLLLL ---
        dst.length_counter_load = (self.io_reg [APU_NOISE_OFFSET + 3] & 0xf8) >> 3;

        Some (dst)
    }

    /// Get the DMC settings
    pub fn read_apu_dmc_config(&self) -> Option <DmcSound> {
        // Return immediately if playback is disabled
        if! self.read_apu_is_enable_dmc () {
            return None;
        }
        let mut dst = DmcSound :: default ();
        // $ 4010 IL--RRRR
        dst.is_irq_enable = (self.io_reg [APU_DMC_OFFSET + 0] & 0x80) == 0x80;
        dst.is_loop_enable = (self.io_reg [APU_DMC_OFFSET + 0] & 0x40) == 0x40;
        dst.frequency = self.io_reg [APU_DMC_OFFSET + 0] & 0x0f;
        // $ 4011 -DDDDDDDD
        dst.load_counter = self.io_reg [APU_DMC_OFFSET + 1] & 0x7f;
        // $ 4012 Sample Address
        dst.sample_addr = self.io_reg [APU_DMC_OFFSET + 2];
        // $ 4013 Sample Length
        dst.sample_length = self.io_reg [APU_DMC_OFFSET + 3];

        Some (dst)
    }

    // $ 4015 Status
    // --- DNTPP
    pub fn read_apu_is_enable_dmc(&self) -> bool {
        (self.io_reg [APU_STATUS_OFFSET] & 0x10) == 0x10
    }
    pub fn read_apu_is_enable_noise(&self) -> bool {
        (self.io_reg [APU_STATUS_OFFSET] & 0x08) == 0x08
    }
    pub fn read_apu_is_enable_tri(&self) -> bool {
        (self.io_reg [APU_STATUS_OFFSET] & 0x04) == 0x04
    }
    pub fn read_apu_is_enable_pulse(&self, index: u8) -> bool {
        debug_assert!(index < 2);
        if index == 0 {
            (self.io_reg[APU_STATUS_OFFSET] & 0x01) == 0x01
        } else {
            (self.io_reg[APU_STATUS_OFFSET] & 0x02) == 0x02
        }
    }

    // TODO: $4017 Frame Counter
}
