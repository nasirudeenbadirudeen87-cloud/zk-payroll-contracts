mod common;

use common::normalize_asset_symbol;

#[test]
fn normalization() {
    assert_eq!(normalize_asset_symbol(" btc "), Ok("BTC".to_string()));
    assert!(normalize_asset_symbol("BTC/USD").is_err());
}
