use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FactionVoucher {
    faction: String,
    amount: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VoucherType {
    #[serde(alias = "CombatBond")]
    CombatBond,
    Bounty,
    Codex,
    Scannable,
    Settlement,
    Trade,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RedeemVoucherEvent {
    amount: u64,
    broker_percentage: Option<f64>,
    faction: Option<String>,
    factions: Option<Vec<FactionVoucher>>,
    #[serde(alias = "Type")]
    voucher_type: VoucherType,
}
