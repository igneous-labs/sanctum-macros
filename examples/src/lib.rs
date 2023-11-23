use sanctum_macros::declare_program_keys;

declare_program_keys!(
    "9BoN4yBYwH63LFM9fDamaHK62YjM56hWYZqok7MnAakJ",
    [("state", b"state"), ("empty-kebab", b"")]
);

#[cfg(test)]
mod tests {
    use super::*;

    use solana_program::pubkey::Pubkey;
    use std::str::FromStr;

    #[test]
    fn program_id_check() {
        assert_eq!(ID_STR, "9BoN4yBYwH63LFM9fDamaHK62YjM56hWYZqok7MnAakJ");
        assert_eq!(ID_STR, ID.to_string());
        assert_eq!(ID, Pubkey::from_str(ID_STR).unwrap());
    }

    #[test]
    fn state_check() {
        assert_eq!(STATE_SEED, b"state");
        let (expected_pk, expected_bump) = Pubkey::find_program_address(&[STATE_SEED], &ID);
        assert_eq!(STATE_ID, expected_pk);
        assert_eq!(STATE_BUMP, expected_bump);
        assert_eq!(STATE_ID_STR, expected_pk.to_string());
    }

    #[test]
    fn empty_kebab_check() {
        assert_eq!(EMPTY_KEBAB_SEED, b"");
        let (expected_pk, expected_bump) = Pubkey::find_program_address(&[EMPTY_KEBAB_SEED], &ID);
        assert_eq!(EMPTY_KEBAB_ID, expected_pk);
        assert_eq!(EMPTY_KEBAB_BUMP, expected_bump);
        assert_eq!(EMPTY_KEBAB_ID_STR, expected_pk.to_string());
        assert_ne!(ID, EMPTY_KEBAB_ID);
    }
}
