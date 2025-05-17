use crate::math::get_limited_value::get_limited_value;

#[derive(Debug, Clone)]
pub(crate) struct Slew {
    end: f32,
    step: f32,
    start: f32,
    value: f32,
}

impl Slew {
    pub(crate) fn new(start: f32, end: f32, duration: f32, sample_rate: f32) -> Self {
        let step = (end - start) * sample_rate / duration;
        Self {
            end,
            start,
            step,
            value: start,
        }
    }

    pub(crate) fn get_is_finished(&self) -> bool {
        self.value == self.end
    }

    pub(crate) fn get_value(&self) -> f32 {
        self.value
    }

    pub(crate) fn process(&mut self) {
        if self.get_is_finished() {
            return;
        }

        self.value = get_limited_value(self.value + self.step, self.start, self.end);
    }
}
