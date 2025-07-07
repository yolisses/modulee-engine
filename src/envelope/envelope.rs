use super::curve::Curve;

#[derive(Debug, Clone)]
enum EnvelopeState {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

#[derive(Debug, Clone)]
pub(crate) struct Envelope {
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

    pub(crate) fn update_parameters(
        &mut self,
        attack: f32,
        decay: f32,
        sustain: f32,
        release: f32,
    ) {
        self.attack = attack;
        self.decay = decay;
        self.sustain = sustain;
        self.release = release;

        match self.state {
            EnvelopeState::Attack => self.curve.update_duration(attack),
            EnvelopeState::Decay => self.curve.update_duration(decay),
            EnvelopeState::Release => self.curve.update_duration(release),
            _ => (),
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

    pub(crate) fn get_is_pending(&self) -> bool {
        !matches!(self.state, EnvelopeState::Idle)
    }
}

#[cfg(test)]
mod tests {
    use super::Envelope;
    use crate::tests::relative_eq_array::assert_array_approx_eq;

    fn get_test_values(envelope: &mut Envelope, iterations: usize) -> Vec<f32> {
        let mut values = vec![];
        for _ in 0..iterations {
            envelope.process();
            values.push(envelope.get_value());
        }
        values
    }

    #[test]
    fn test_envelope() {
        let mut envelope = Envelope::new(4., 3., 0.4, 4., 1.);

        // Idle
        assert_array_approx_eq(&get_test_values(&mut envelope, 3), &vec![0., 0., 0.]);
        assert_eq!(envelope.get_is_pending(), false);

        envelope.set_note_on();

        // Attack
        assert_array_approx_eq(
            &get_test_values(&mut envelope, 4),
            &vec![0.25, 0.5, 0.75, 1.],
        );
        assert_eq!(envelope.get_is_pending(), true);

        // Decay
        assert_array_approx_eq(&get_test_values(&mut envelope, 3), &vec![0.8, 0.6, 0.4]);
        assert_eq!(envelope.get_is_pending(), true);

        // Sustain
        assert_array_approx_eq(&get_test_values(&mut envelope, 7), &vec![0.4; 7]);
        assert_eq!(envelope.get_is_pending(), true);

        envelope.set_note_off();

        // Release
        assert_array_approx_eq(&get_test_values(&mut envelope, 4), &vec![0.3, 0.2, 0.1, 0.]);
        assert_eq!(envelope.get_is_pending(), true);

        // Idle
        assert_array_approx_eq(&get_test_values(&mut envelope, 7), &vec![0.; 7]);
        assert_eq!(envelope.get_is_pending(), false);
    }
}
