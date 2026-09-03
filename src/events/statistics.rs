use serde::{Deserialize, Serialize};

use crate::elite::statistics::{
    BankAccountStatistics, CombatStatistics, CqcStatistics, CraftingStatistics, CrewStatistics,
    CrimeStatistics, ExobiologyStatistics, ExplorationStatistics, FleetCarrierStatistics,
    MaterialTradeStatistics, MiningStatistics, MulticrewStatistics, PassengersStatistics,
    SearchAndRescueStatistics, SmugglingStatistics, SquadronStatistics, ThargoidStatistics,
    TradingStatistics,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StatisticsEvent {
    #[serde(rename = "Bank_Account")]
    pub bank_account: Option<BankAccountStatistics>,
    #[serde(rename = "CQC")]
    pub cqc: Option<CqcStatistics>,
    pub combat: Option<CombatStatistics>,
    pub exploration: Option<ExplorationStatistics>,
    pub crafting: Option<CraftingStatistics>,
    pub crime: Option<CrimeStatistics>,
    pub exobiology: Option<ExobiologyStatistics>,
    #[serde(rename = "FLEETCARRIER")]
    pub fleet_carrier: Option<FleetCarrierStatistics>,
    #[serde(rename = "Material_Trader_Stats")]
    pub material_trading: Option<MaterialTradeStatistics>,
    pub mining: Option<MiningStatistics>,
    pub passengers: Option<PassengersStatistics>,
    pub search_and_rescue: Option<SearchAndRescueStatistics>,
    pub smuggling: Option<SmugglingStatistics>,
    pub squadron: Option<SquadronStatistics>,
    pub thargoid: Option<ThargoidStatistics>,
    pub trading: Option<TradingStatistics>,
    pub crew: Option<CrewStatistics>,
    pub multicrew: Option<MulticrewStatistics>,
}
