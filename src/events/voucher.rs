use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FactionVoucher {
    pub faction: String,
    pub amount: u64,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RedeemVoucherEvent {
    pub amount: u64,
    pub broker_percentage: Option<f64>,
    pub faction: Option<String>,
    pub factions: Option<Vec<FactionVoucher>>,
    #[serde(alias = "Type")]
    pub voucher_type: VoucherType,
}
