use anchor_lang::prelude::*;

use crate::errors::VaultError;

/// ScaledUiAmount extension type. spl-token-2022 extension enum.
pub const SCALED_UI_AMOUNT: u16 = 25;

/// Effective ScaledUiAmount multiplier, hand-parsed TLV.
/// Scaffold from design §5.6. Unit-test against the real AAPLx mint bytes in T14.
///
/// Mint base = 82 bytes, padded to 165, then 1 byte AccountType (1 = Mint),
/// then TLV entries `[u16 type][u16 len][data]`.
pub fn effective_multiplier_e12(mint_data: &[u8], now: i64) -> Result<u128> {
    let mut i = 166usize;
    while i + 4 <= mint_data.len() {
        let t = u16::from_le_bytes([mint_data[i], mint_data[i + 1]]);
        let l = u16::from_le_bytes([mint_data[i + 2], mint_data[i + 3]]) as usize;
        require!(i + 4 + l <= mint_data.len(), VaultError::InvalidMint);
        let d = &mint_data[i + 4..i + 4 + l];
        if t == SCALED_UI_AMOUNT {
            require!(l >= 56, VaultError::InvalidMint);
            let multiplier = f64::from_le_bytes(d[32..40].try_into().unwrap());
            let timestamp = i64::from_le_bytes(d[40..48].try_into().unwrap());
            let new_multiplier = f64::from_le_bytes(d[48..56].try_into().unwrap());
            let effective = if now >= timestamp {
                new_multiplier
            } else {
                multiplier
            };
            require!(effective.is_finite() && effective > 0.0, VaultError::InvalidMint);
            return Ok((effective * 1e12) as u128);
        }
        if t == 0 {
            break;
        }
        i += 4 + l;
    }
    err!(VaultError::InvalidMint)
}
