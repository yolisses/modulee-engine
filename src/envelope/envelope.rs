pub struct Envelope {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
    sample_rate: f32,
    state: EnvelopeState,
    current_level: f32,
    current_sample: usize,
}

#[derive(Debug, PartialEq)]
enum EnvelopeState {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

impl Envelope {
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32, sample_rate: f32) -> Self {
        Envelope {
            attack,
            decay,
            sustain,
            release,
            sample_rate,
            state: EnvelopeState::Idle,
            current_level: 0.0,
            current_sample: 0,
        }
    }

    pub fn note_on(&mut self) {
        self.state = EnvelopeState::Attack;
        self.current_sample = 0;
    }

    pub fn note_off(&mut self) {
        self.state = EnvelopeState::Release;
        self.current_sample = 0;
    }
    fn process_attack(&mut self) -> f32 {
        self.current_level += 1.0 / (self.attack * self.sample_rate);
        if self.current_level >= 1.0 {
            self.current_level = 1.0;
            self.state = EnvelopeState::Decay;
            self.current_sample = 0;
        }
        self.current_level
    }

    fn process_decay(&mut self) -> f32 {
        self.current_level -= (1.0 - self.sustain) / (self.decay * self.sample_rate);
        if self.current_level <= self.sustain {
            self.current_level = self.sustain;
            self.state = EnvelopeState::Sustain;
        }
        self.current_level
    }

    fn process_sustain(&self) -> f32 {
        self.sustain
    }

    fn process_release(&mut self) -> f32 {
        self.current_level -= self.sustain / (self.release * self.sample_rate);
        if self.current_level <= 0.0 {
            self.current_level = 0.0;
            self.state = EnvelopeState::Idle;
        }
        self.current_level
    }

    pub fn next_sample(&mut self) -> f32 {
        match self.state {
            EnvelopeState::Idle => 0.0,
            EnvelopeState::Attack => self.process_attack(),
            EnvelopeState::Decay => self.process_decay(),
            EnvelopeState::Sustain => self.process_sustain(),
            EnvelopeState::Release => self.process_release(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adsr_envelope_initial_state() {
        let envelope = Envelope::new(0.1, 0.1, 0.8, 0.1, 10.0);
        assert_eq!(envelope.state, EnvelopeState::Idle);
        assert_eq!(envelope.current_level, 0.0);
    }

    #[test]
    fn test_adsr_envelope_note_on() {
        let mut envelope = Envelope::new(0.1, 0.1, 0.8, 0.1, 10.0);
        envelope.note_on();
        assert_eq!(envelope.state, EnvelopeState::Attack);
        assert_eq!(envelope.current_sample, 0);
    }

    #[test]
    fn test_adsr_envelope_note_off() {
        let mut envelope = Envelope::new(0.1, 0.1, 0.8, 0.1, 10.0);
        envelope.note_off();
        assert_eq!(envelope.state, EnvelopeState::Release);
        assert_eq!(envelope.current_sample, 0);
    }

    #[test]
    fn test_adsr_envelope_attack_to_decay_transition() {
        let mut envelope = Envelope::new(0.1, 0.1, 0.8, 0.1, 10.0);
        envelope.note_on();
        for _ in 0..(0.1 * 10.0) as usize {
            envelope.next_sample();
        }
        assert_eq!(envelope.state, EnvelopeState::Decay);
        assert_eq!(envelope.current_level, 1.0);
    }

    #[test]
    fn test_adsr_envelope_decay_to_sustain_transition() {
        let mut envelope = Envelope::new(0.1, 0.1, 0.8, 0.1, 10.0);
        envelope.note_on();
        for _ in 0..(0.1 * 10.0) as usize {
            envelope.next_sample();
        }
        for _ in 0..(0.1 * 10.0) as usize {
            envelope.next_sample();
        }
        assert_eq!(envelope.state, EnvelopeState::Sustain);
        assert_eq!(envelope.current_level, 0.8);
    }

    #[test]
    fn test_adsr_envelope_release_to_idle_transition() {
        let mut envelope = Envelope::new(0.1, 0.1, 0.8, 0.1, 10.0);
        envelope.note_on();
        for _ in 0..(0.1 * 10.0) as usize {
            envelope.next_sample();
        }
        for _ in 0..(0.1 * 10.0) as usize {
            envelope.next_sample();
        }
        envelope.note_off();
        for _ in 0..(0.1 * 10.0) as usize {
            envelope.next_sample();
        }
        assert_eq!(envelope.state, EnvelopeState::Idle);
        assert_eq!(envelope.current_level, 0.0);
    }
}
