use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DockingAccess {
    All,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CarrierType {
    FleetCarrier,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxRate {
    #[serde(rename = "TaxRate_rearm")]
    pub rearm: f64,
    #[serde(rename = "TaxRate_refuel")]
    pub refuel: f64,
    #[serde(rename = "TaxRate_repair")]
    pub repair: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierFinance {
    pub available_balance: u64,
    pub carrier_balance: u64,
    pub reserve_balance: u64,
    pub reserve_percent: f64,
    pub tax_rate: TaxRate,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierSpaceUsage {
    pub cargo: u64,
    pub cargo_space_reserved: u64,
    pub crew: u64,
    pub free_space: u64,
    pub module_packs: u64,
    pub ship_packs: u64,
    pub total_capacity: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CarrierCrewRole {
    BlackMarket,
    Captain,
    Refuel,
    Rearm,
    Repair,
    Commodities,
    VoucherRedemption,
    Exploration,
    Shipyard,
    Outfitting,
    CarrierFuel,
    VistaGenomics,
    PioneerSupplies,
    Bartender,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierCrew {
    pub crew_role: CarrierCrewRole,
    #[serde(default)]
    pub activated: bool,
    #[serde(default)]
    pub enabled: bool,
    pub crew_name: Option<String>,
}
