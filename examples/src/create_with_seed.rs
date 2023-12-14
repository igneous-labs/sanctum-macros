use sanctum_macros::create_with_seed;

create_with_seed!(
    "9BoN4yBYwH63LFM9fDamaHK62YjM56hWYZqok7MnAakJ",
    "seed",
    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
);

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use solana_program::pubkey::Pubkey;

    use crate::test_utils::assert_correct_pubkey_consts;

    use super::*;

    #[test]
    fn id_check() {
        let expected_pk = Pubkey::create_with_seed(&BASE_ID, SEED, &OWNER_ID).unwrap();
        assert_correct_pubkey_consts(expected_pk, ID, ID_STR, ID_BYTES);
    }

    #[test]
    fn base_check() {
        let expected_pk = Pubkey::from_str("9BoN4yBYwH63LFM9fDamaHK62YjM56hWYZqok7MnAakJ").unwrap();
        assert_correct_pubkey_consts(expected_pk, BASE_ID, BASE_ID_STR, BASE_ID_BYTES);
    }

    #[test]
    fn seed_check() {
        assert_eq!("seed", SEED);
    }

    #[test]
    fn owner_check() {
        let expected_pk = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
        assert_correct_pubkey_consts(expected_pk, OWNER_ID, OWNER_ID_STR, OWNER_ID_BYTES);
    }
}
