# sanctum-macros

Inspired by https://github.com/Ellipsis-Labs/ellipsis-macros

## Compile-Time Pubkey consts Generation

`solana_program` should be a declared dependency in the crates using these macros. 

### `declare_program_keys!`

```rust ignore
// first arg = program ID
// second arg = list of static PDA names and their seeds
declare_program_keys!(
    "9BoN4yBYwH63LFM9fDamaHK62YjM56hWYZqok7MnAakJ",
    [
        ("state", b"state"),
        ("empty-kebab", b""),
    ]
);
```

expands to

```rust ignore
pub const ID_STR: &str = "9BoN4yBYwH63LFM9fDamaHK62YjM56hWYZqok7MnAakJ";
pub const ID: solana_program::pubkey::Pubkey = solana_program::pubkey::Pubkey::new_from_array([121, 161, 186, 2, 16, 170, 248, 125, 201, 230, 113, 160, 74, 35, 69, 149, 10, 116, 97, 215, 244, 204, 210, 189, 7, 112, 233, 119, 14, 109, 226, 43]);

pub const STATE_SEED: &[u8] = b"state";
// BUMP and STATE_ID are results of running
// Pubkey::find_program_address(
//    &[STATE_SEED],
// &ID); 
pub const STATE_BUMP: u8 = 255;
pub const STATE_ID: solana_program::pubkey::Pubkey = solana_program::pubkey::Pubkey::new_from_array([182, 221, 112, 246, 145, 207, 204, 110, 1, 1, 34, 100, 242, 173, 44, 12, 6, 58, 98, 95, 54, 209, 117, 196, 110, 161, 65, 215, 10, 127, 217, 120]);
// base58 of STATE_ID
pub const STATE_ID_STR: &str = "DJq3bbgiJq34LKrH37UEb7rXDaWTUGjnieLZamkRvu5R";

pub const EMPTY_KEBAB_SEED: &[u8] = b"";
pub const EMPTY_KEBAB_BUMP: u8 = 255;
pub const EMPTY_KEBAB_ID: solana_program::pubkey::Pubkey = solana_program::pubkey::Pubkey::new_from_array([149, 184, 104, 22, 114, 239, 248, 126, 73, 171, 206, 5, 196, 95, 255, 54, 180, 176, 70, 241, 246, 15, 193, 242, 103, 208, 21, 144, 97, 138, 236, 108]);
pub const EMPTY_KEBAB_ID_STR: &str = "B5SqYyds9eLeX5mK4uycKGgZHft1URCbTzU6LoWhCV63";
```
