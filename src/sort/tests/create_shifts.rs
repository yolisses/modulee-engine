use crate::sort::shifts::Shifts;

pub(crate) fn create_shifts(data: &[(usize, usize)]) -> Shifts {
    let mut shifts = Shifts::default();
    for (key, value) in data {
        shifts.insert(*key, *value);
    }
    shifts
}
