#[cfg(test)]
pub fn assert_correct_pubkey_consts(
    expected_pk: solana_program::pubkey::Pubkey,
    actual_pk: solana_program::pubkey::Pubkey,
    actual_str: &str,
    actual_bytes: [u8; 32],
) {
    assert_eq!(actual_pk, expected_pk);
    assert_eq!(actual_str, &expected_pk.to_string());
    assert_eq!(actual_bytes, expected_pk.to_bytes());
}
