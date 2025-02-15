pub(crate) trait SetNoteTrait {
    fn set_note_on(&mut self, pitch: f32);
    fn set_note_off(&mut self, pitch: f32);
}
