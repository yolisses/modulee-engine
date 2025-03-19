pub(crate) fn get_u64_seed_from_f32(value: f32) -> u64 {
    let bits = value.to_bits() as u64;
    bits << 32 | bits
}

#[cfg(test)]
mod tests {
    use crate::get_u64_seed_from_f32::get_u64_seed_from_f32;

    #[test]
    fn test_get_u64_seed_from_f32() {
        assert_eq!(get_u64_seed_from_f32(0.), 0);
        assert_eq!(get_u64_seed_from_f32(1.), 4575657222473777152);
        assert_eq!(get_u64_seed_from_f32(2.), 4611686019501129728);
        assert_eq!(get_u64_seed_from_f32(3.), 4629700418014806016);
        assert_eq!(get_u64_seed_from_f32(1.1), 4579260103035505869);
        assert_eq!(get_u64_seed_from_f32(1.2), 4582862983597234586);
        assert_eq!(get_u64_seed_from_f32(1.3), 4586465859863996006);
    }
}
