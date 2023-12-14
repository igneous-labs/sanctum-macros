use sanctum_macros::declare_program_keys;

declare_program_keys!(
    "9BoN4yBYwH63LFM9fDamaHK62YjM56hWYZqok7MnAakJ",
    [
        ("state", b"state"),
        ("empty-kebab", b""),
        ("multiseed", b"two", b"seeds"),
    ]
);

#[cfg(test)]
mod tests {
    use crate::test_utils::assert_correct_pubkey_consts;

    use super::*;

    use solana_program::pubkey::Pubkey;
    use std::str::FromStr;

    #[test]
    fn program_id_check() {
        let expected_pk = Pubkey::from_str("9BoN4yBYwH63LFM9fDamaHK62YjM56hWYZqok7MnAakJ").unwrap();
        assert_correct_pubkey_consts(expected_pk, ID, ID_STR, ID_BYTES);
    }

    #[test]
    fn state_check() {
        assert_eq!(STATE_SEED, b"state");
        let (expected_pk, expected_bump) = Pubkey::find_program_address(&[STATE_SEED], &ID);
        assert_eq!(STATE_BUMP, expected_bump);
        assert_correct_pubkey_consts(expected_pk, STATE_ID, STATE_ID_STR, STATE_ID_BYTES);
    }

    #[test]
    fn empty_kebab_check() {
        assert_eq!(EMPTY_KEBAB_SEED, b"");
        let (expected_pk, expected_bump) = Pubkey::find_program_address(&[EMPTY_KEBAB_SEED], &ID);
        assert_eq!(EMPTY_KEBAB_BUMP, expected_bump);
        assert_correct_pubkey_consts(
            expected_pk,
            EMPTY_KEBAB_ID,
            EMPTY_KEBAB_ID_STR,
            EMPTY_KEBAB_ID_BYTES,
        );
        assert_ne!(ID, EMPTY_KEBAB_ID);
    }

    #[test]
    fn multiseed_check() {
        assert_eq!(MULTISEED_SEED_0, b"two");
        assert_eq!(MULTISEED_SEED_1, b"seeds");
        let (expected_pk, expected_bump) =
            Pubkey::find_program_address(&[MULTISEED_SEED_0, MULTISEED_SEED_1], &ID);
        assert_eq!(MULTISEED_BUMP, expected_bump);
        assert_correct_pubkey_consts(
            expected_pk,
            MULTISEED_ID,
            MULTISEED_ID_STR,
            MULTISEED_ID_BYTES,
        );
    }
}
