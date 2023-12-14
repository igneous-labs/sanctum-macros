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
pub const ID_BYTES: [u8; 32] = [121, 161, 186, 2, 16, 170, 248, 125, 201, 230, 113, 160, 74, 35, 69, 149, 10, 116, 97, 215, 244, 204, 210, 189, 7, 112, 233, 119, 14, 109, 226, 43];
pub const ID: solana_program::pubkey::Pubkey = solana_program::pubkey::Pubkey::new_from_array(ID_BYTES);

// let (STATE_ID, STATE_BUMP) = Pubkey::find_program_address(&[STATE_SEED], &ID);
pub const STATE_SEED: &[u8] = b"state";
pub const STATE_BUMP: u8 = 255;
pub const STATE_ID_BYTES: [u8; 32] = [182, 221, 112, 246, 145, 207, 204, 110, 1, 1, 34, 100, 242, 173, 44, 12, 6, 58, 98, 95, 54, 209, 117, 196, 110, 161, 65, 215, 10, 127, 217, 120];
pub const STATE_ID: solana_program::pubkey::Pubkey = solana_program::pubkey::Pubkey::new_from_array(STATE_ID_BYTES);
pub const STATE_ID_STR: &str = "DJq3bbgiJq34LKrH37UEb7rXDaWTUGjnieLZamkRvu5R";

// let (EMPTY_KEBAB_ID, EMPTY_KEBAB_BUMP) = Pubkey::find_program_address(&[EMPTY_KEBAB_SEED], &ID);
pub const EMPTY_KEBAB_SEED: &[u8] = b"";
pub const EMPTY_KEBAB_BUMP: u8 = 255;
pub const EMPTY_KEBAB_ID_BYTES: [u8; 32] = [149, 184, 104, 22, 114, 239, 248, 126, 73, 171, 206, 5, 196, 95, 255, 54, 180, 176, 70, 241, 246, 15, 193, 242, 103, 208, 21, 144, 97, 138, 236, 108];
pub const EMPTY_KEBAB_ID: solana_program::pubkey::Pubkey = solana_program::pubkey::Pubkey::new_from_array(EMPTY_KEBAB_ID_BYTES);
pub const EMPTY_KEBAB_ID_STR: &str = "B5SqYyds9eLeX5mK4uycKGgZHft1URCbTzU6LoWhCV63";
```

### `create_with_seed!`

Create a `Pubkey` with [`Pubkey::create_with_seed`](https://docs.rs/solana-program/latest/solana_program/pubkey/struct.Pubkey.html#method.create_with_seed).

```rust ignore
// args: (name of const, base, seed, owner)
create_with_seed!(
    "9BoN4yBYwH63LFM9fDamaHK62YjM56hWYZqok7MnAakJ",
    "seed",
    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
);
```

expands to

```rust ignore
// let ID = Pubkey::create_with_seed(&BASE_ID, SEED, OWNER).unwrap();
pub const ID_BYTES: [u8; 32] = [213, 14, 91, 195, 42, 234, 158, 119, 156, 244, 205, 121, 213, 202, 90, 242, 52, 33, 180, 126, 177, 142, 7, 59, 192, 104, 238, 67, 90, 112, 177, 48];
pub const ID_STR: &str = "FLgYDs1daqCvXAmz5vqij39zFdSraks4S6xuKvf2NJsZ";
pub const ID: solana_program::pubkey::Pubkey = solana_program::pubkey::Pubkey::new_from_array(ID_BYTES);

pub const BASE_ID_BYTES: [u8; 32] = [121, 161, 186, 2, 16, 170, 248, 125, 201, 230, 113, 160, 74, 35, 69, 149, 10, 116, 97, 215, 244, 204, 210, 189, 7, 112, 233, 119, 14, 109, 226, 43];
pub const BASE_ID_STR: &str = "9BoN4yBYwH63LFM9fDamaHK62YjM56hWYZqok7MnAakJ";
pub const BASE_ID: solana_program::pubkey::Pubkey = solana_program::pubkey::Pubkey::new_from_array(BASE_ID_BYTES);

pub const SEED: &str = "seed";

pub const OWNER_ID_BYTES: [u8; 32] = [6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172, 28, 180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169];
pub const OWNER_ID_STR: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
pub const OWNER_ID: solana_program::pubkey::Pubkey = solana_program::pubkey::Pubkey::new_from_array(OWNER_ID_BYTES);
```

When used with [`system_instruction::create_account_with_seed()`](https://docs.rs/solana-program/latest/solana_program/system_instruction/fn.create_account_with_seed.html), a new account owned by `OWNER_ID` program will be created at `ID`. `BASE_ID` must be a signer.
