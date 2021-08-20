use num_traits::FromPrimitive;

pub fn json_discriminant_array_to_vec<T: FromPrimitive>(unparsed: &str) -> Result<Vec<T>, &str> {
    unparsed
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().ok().and_then(T::from_u64).ok_or(s))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::rbac::test_helpers::TestPolicy::{self, *};

    use super::*;

    #[test]
    fn test_json_discriminant_array_to_vec() {
        let unparsed = "[0, 2, 5, 8]";
        let actual = json_discriminant_array_to_vec::<TestPolicy>(unparsed).expect("Vec of Policy");
        assert_eq!(actual, vec![Policy0, Policy2, Policy5, Policy8]);

        let unparsed = "[1, 3, 999]";
        let actual =
            json_discriminant_array_to_vec::<TestPolicy>(unparsed).expect_err("should error");
        assert_eq!(actual, "999");
    }
}
