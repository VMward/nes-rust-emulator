use super::cpu::*;

#[derive(Copy, Clone)]
pub enum PulseDutyCycle {
    Duty12_5,
    Duty25_0,
    Duty50_0,
    Duty75_0,
}

#[derive(Copy, Clone)]
pub struct PulseSound {
    /// Duty Cycle $4000 - $4004
    pub duty_cycle: PulseDutyCycle,
    pub is_length_counter_halt: bool,
    pub is_constant_volume: bool,
    pub volume: u8,

    // $4001 / $4005
    pub is_sweep_enable: bool,
    pub sweep_period: u8,
    pub is_sweep_negative: bool,
    pub sweep_shift: u8,

    // $4002, $4003 / $4006, $4007
    pub timer_value: u16,
    pub length_counter_load: u8,
}
impl Default for PulseSound {
    fn default() -> Self {
        Self {
            duty_cycle: PulseDutyCycle::Duty12_5,
            is_length_counter_halt: false,
            is_constant_volume: false,
            volume: 0,
            is_sweep_enable: false,
            sweep_period: 0,
            is_sweep_negative: false,
            sweep_shift: 0,
            timer_value: 0,
            length_counter_load: 0,
        }
    }
}

impl PulseSound {
    pub fn get_freq(&self) -> u32 {
        CPU_FREQ / (16 * (u32::from(self.timer_value) + 1))
    }
}

#[derive(Copy, Clone)]
pub struct TriangleSound {
    // $4008
    /// 再生時間カウンタ有
    pub is_length_counter_halt: bool,
    pub counter_load: u8,
    // $400a, $400b
    pub timer_value: u16,
    pub length_counter_load: u8,
}

impl Default for TriangleSound {
    fn default() -> Self {
        Self {
            is_length_counter_halt: false,
            counter_load: 0,
            timer_value: 0,
            length_counter_load: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct NoiseSound {
    // $400c
    pub is_length_counter_halt: bool,
    pub is_constant_volume: bool,
    pub volume: u8,
    // $400E
    pub is_noise_type_loop: bool,
    pub noise_period: u8,
    // $400f
    pub length_counter_load: u8,
}

impl Default for NoiseSound {
    fn default() -> Self {
        Self {
            is_length_counter_halt: false,
            is_constant_volume: false,
            volume: 0,
            is_noise_type_loop: false,
            noise_period: 0,
            length_counter_load: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct DmcSound {
    // $4010
    pub is_irq_enable: bool,
    pub is_loop_enable: bool,
    pub frequency: u8,
    // $4011
    pub load_counter: u8,
    // $4012
    /// $C000-FFFF
    /// 11AAAAAA-AA000000
    pub sample_addr: u8,
    // $4013
    /// 0000LLLL, LLLL0001
    pub sample_length: u8,
}

impl Default for DmcSound {
    fn default() -> Self {
        Self {
            is_irq_enable: false,
            is_loop_enable: false,
            frequency: 0,
            load_counter: 0,
            sample_addr: 0,
            sample_length: 0,
        }
    }
}

#[derive(Clone)]
pub struct Apu {
    // Frame Sequencer、CPU
    // 11bit
    pub frame_seq_counter: u16,
}
impl Default for Apu {
    fn default() -> Self {
        Self {
            frame_seq_counter: 0,
        }
    }
}

impl Apu {
    // FrameSeq
    // TODO: Deadcode
    #[allow(dead_code)]
    fn increment_seq(&mut self, cpu_cyc: u8) {
        self.frame_seq_counter = (self.frame_seq_counter + u16::from(cpu_cyc)) & 0x03ff;
        // 11bit
    }
    // APU
    pub fn step(&mut self, _cpu: &mut Cpu, _cpu_cyc: u8) {
        // TODO:
    }
}
