use super::curve::Curve;

enum EnvelopeState {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

struct Envelope {
    decay: f32,
    attack: f32,
    curve: Curve,
    sustain: f32,
    release: f32,
    sample_rate: f32,
    state: EnvelopeState,
}

impl Envelope {
    pub(crate) fn new(
        attack: f32,
        decay: f32,
        sustain: f32,
        release: f32,
        sample_rate: f32,
    ) -> Self {
        let curve = Curve::new(0., 1., attack, sample_rate);

        Self {
            curve,
            decay,
            attack,
            sustain,
            release,
            sample_rate,
            state: EnvelopeState::Idle,
        }
    }

    fn process_attack_state(&mut self) {
        self.curve.process();
        if self.curve.get_is_finished() {
            self.state = EnvelopeState::Decay;
            self.curve = Curve::new(1., self.sustain, self.decay, self.sample_rate);
        }
    }

    fn process_decay_state(&mut self) {
        self.curve.process();
        if self.curve.get_is_finished() {
            self.state = EnvelopeState::Sustain;
        }
    }

    fn process_release_state(&mut self) {
        self.curve.process();
        if self.curve.get_is_finished() {
            self.state = EnvelopeState::Idle;
        }
    }

    pub(crate) fn process(&mut self) {
        match self.state {
            EnvelopeState::Attack => self.process_attack_state(),
            EnvelopeState::Decay => self.process_decay_state(),
            EnvelopeState::Release => self.process_release_state(),
            _ => (),
        }
    }

    pub(crate) fn get_value(&self) -> f32 {
        match self.state {
            EnvelopeState::Idle => 0.,
            EnvelopeState::Sustain => self.sustain,
            _ => self.curve.get_value(),
        }
    }

    pub(crate) fn set_note_on(&mut self) {
        self.state = EnvelopeState::Attack;
        self.curve = Curve::new(0., 1., self.attack, self.sample_rate);
    }

    pub(crate) fn set_note_off(&mut self) {
        self.state = EnvelopeState::Release;
        let current_value = self.curve.get_value();
        self.curve = Curve::new(current_value, 0., self.release, self.sample_rate);
    }
}
