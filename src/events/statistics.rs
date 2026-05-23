use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BankAccountStatistics {
    #[serde(rename = "Current_Wealth")]
    current_wealth: u64,
    #[serde(rename = "Insurance_Claims")]
    insurance_claims: u64,
    #[serde(rename = "Owned_Ship_Count")]
    owned_ship_count: u64,
    #[serde(rename = "Premium_Stock_Bought")]
    premium_stock_bought: Option<u64>,
    #[serde(rename = "Spent_On_Ammo_Consumables")]
    spent_on_ammo_consumables: u64,
    #[serde(rename = "Spent_On_Fuel")]
    spent_on_fuel: u64,
    #[serde(rename = "Spent_On_Insurance")]
    spent_on_insurance: u64,
    #[serde(rename = "Spent_On_Outfitting")]
    spent_on_outfitting: u64,
    #[serde(rename = "Spent_On_Premium_Stock")]
    spent_on_premium_stock: Option<u64>,
    #[serde(rename = "Spent_On_Repairs")]
    spent_on_repairs: u64,
    #[serde(rename = "Spent_On_Ships")]
    spent_on_ships: u64,
    #[serde(rename = "Spent_On_Suit_Consumables")]
    spent_on_suit_consumables: Option<u64>,
    #[serde(rename = "Spent_On_Suits")]
    spent_on_suits: Option<u64>,
    #[serde(rename = "Spent_On_Weapons")]
    spent_on_weapons: Option<u64>,
    #[serde(rename = "Suits_Owned")]
    suits_owned: Option<u64>,
    #[serde(rename = "Weapons_Owned")]
    weapons_owned: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct CqcStatistics {
    #[serde(rename = "CQC_Credits_Earned")]
    credits_earned: u64,
    #[serde(rename = "CQC_KD")]
    kd: f64,
    #[serde(rename = "CQC_Kills")]
    kills: u64,
    #[serde(rename = "CQC_Time_Played")]
    time_played: u64,
    #[serde(rename = "CQC_WL")]
    wl: u64,
}

#[derive(Debug, Deserialize)]
pub struct CombatStatistics {
    #[serde(rename = "Assassination_Profits")]
    assassination_profits: u64,
    #[serde(rename = "Assassinations")]
    assassinations: u64,
    #[serde(rename = "Bounties_Claimed")]
    bounties_claimed: u64,
    #[serde(rename = "Bounty_Hunting_Profit")]
    bounty_hunting_profit: u64,
    #[serde(rename = "Combat_Bond_Profits")]
    combat_bond_profits: Option<u64>,
    #[serde(rename = "Combat_Bonds")]
    combat_bonds: Option<u64>,
    #[serde(rename = "ConflictZone_High")]
    conflict_zone_high: Option<u64>,
    #[serde(rename = "ConflictZone_High_Wins")]
    conflict_zone_high_wins: Option<u64>,
    #[serde(rename = "ConflictZone_Low")]
    conflict_zone_low: Option<u64>,
    #[serde(rename = "ConflictZone_Low_Wins")]
    conflict_zone_low_wins: Option<u64>,
    #[serde(rename = "ConflictZone_Medium")]
    conflict_zone_medium: Option<u64>,
    #[serde(rename = "ConflictZone_Medium_Wins")]
    conflict_zone_medium_wins: Option<u64>,
    #[serde(rename = "ConflictZone_Total")]
    conflict_zone_total: Option<u64>,
    #[serde(rename = "ConflictZone_Total_Wins")]
    conflict_zone_total_wins: Option<u64>,
    #[serde(rename = "Dropships_Booked")]
    dropships_booked: Option<u64>,
    #[serde(rename = "Dropships_Cancelled")]
    dropships_cancelled: Option<u64>,
    #[serde(rename = "Dropships_Taken")]
    dropships_taken: Option<u64>,
    #[serde(rename = "Highest_Single_Reward")]
    highest_single_reward: u64,
    #[serde(rename = "OnFoot_Combat_Bonds")]
    on_foot_combat_bonds: Option<u64>,
    #[serde(rename = "OnFoot_Combat_Bonds_Profits")]
    on_foot_combat_bonds_profits: Option<u64>,
    #[serde(rename = "OnFoot_Scavs_Killed")]
    on_foot_scavs_killed: Option<u64>,
    #[serde(rename = "OnFoot_Ships_Destroyed")]
    on_foot_ships_destroyed: Option<u64>,
    #[serde(rename = "OnFoot_Skimmers_Killed")]
    on_foot_skimmers_killed: Option<u64>,
    #[serde(rename = "OnFoot_Vehicles_Destroyed")]
    on_foot_vehicles_destroyed: Option<u64>,
    #[serde(rename = "Settlement_Conquered")]
    settlement_conquered: Option<u64>,
    #[serde(rename = "Settlement_Defended")]
    settlement_defended: Option<u64>,
    #[serde(rename = "Skimmers_Killed")]
    skimmers_killed: u64,
}

#[derive(Debug, Deserialize)]
pub struct ExplorationStatistics {
    #[serde(rename = "Efficient_Scans")]
    efficient_scans: u64,
    #[serde(rename = "Exploration_Profits")]
    exploration_profits: u64,
    #[serde(rename = "First_Footfalls")]
    first_footfalls: Option<u64>,
    #[serde(rename = "Greatest_Distance_From_Start")]
    greatest_distance_from_start: f64,
    #[serde(rename = "Highest_Payout")]
    highest_payout: u64,
    #[serde(rename = "OnFoot_Distance_Travelled")]
    on_foot_distance_travelled: Option<u64>,
    #[serde(rename = "Planet_Footfalls")]
    planet_footfalls: Option<u64>,
    #[serde(rename = "Planets_Scanned_To_Level_2")]
    planets_scanned_to_level_2: u64,
    #[serde(rename = "Planets_Scanned_To_Level_3")]
    planets_scanned_to_level_3: u64,
    #[serde(rename = "Settlements_Visited")]
    settlements_visited: Option<u64>,
    #[serde(rename = "Shuttle_Distance_Travelled")]
    shuttle_distance_travelled: Option<f64>,
    #[serde(rename = "Spent_On_Shuttles")]
    spent_on_shuttles: Option<u64>,
    #[serde(rename = "Systems_Visited")]
    systems_visited: u64,
    #[serde(rename = "Time_Played")]
    time_played: u64,
    #[serde(rename = "Total_Hyperspace_Distance")]
    total_hyperspace_distance: f64,
    #[serde(rename = "Total_Hyperspace_Jumps")]
    total_hyperspace_jumps: u64,
}

#[derive(Debug, Deserialize)]
pub struct CraftingStatistics {
    #[serde(rename = "Count_Of_Used_Engineers")]
    count_of_used_engineers: u64,
    #[serde(rename = "Recipes_Generated")]
    recipes_generated: u64,
    #[serde(rename = "Recipes_Generated_Rank_1")]
    recipes_generated_rank_1: u64,
    #[serde(rename = "Recipes_Generated_Rank_2")]
    recipes_generated_rank_2: u64,
    #[serde(rename = "Recipes_Generated_Rank_3")]
    recipes_generated_rank_3: u64,
    #[serde(rename = "Recipes_Generated_Rank_4")]
    recipes_generated_rank_4: u64,
    #[serde(rename = "Recipes_Generated_Rank_5")]
    recipes_generated_rank_5: u64,
    #[serde(rename = "Suit_Mods_Applied")]
    suit_mods_applied: Option<u64>,
    #[serde(rename = "Suit_Mods_Applied_Full")]
    suit_mods_applied_full: Option<u64>,
    #[serde(rename = "Suits_Upgraded")]
    suits_upgraded: Option<u64>,
    #[serde(rename = "Suits_Upgraded_Full")]
    suits_ugraded_full: Option<u64>,
    #[serde(rename = "Weapon_Mods_Applied")]
    weapon_mods_applied: Option<u64>,
    #[serde(rename = "Weapon_Mods_Applied_Full")]
    weapon_mods_applied_full: Option<u64>,
    #[serde(rename = "Weapons_Upgraded")]
    weapon_upgraded: Option<u64>,
    #[serde(rename = "Weapons_Upgraded_Full")]
    weapons_ugraded_full: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct MaterialTradeStatistics {
    #[serde(rename = "Assets_Traded_In")]
    assets_traded_in: Option<u64>,
    #[serde(rename = "Assets_Traded_Out")]
    assets_traded_out: Option<u64>,
    #[serde(rename = "Encoded_Materials_Traded")]
    encoded_materials_traded: Option<u64>,
    #[serde(rename = "Grade_1_Materials_Traded")]
    grade_1_materials_traded: Option<u64>,
    #[serde(rename = "Grade_2_Materials_Traded")]
    grade_2_materials_traded: Option<u64>,
    #[serde(rename = "Grade_3_Materials_Traded")]
    grade_3_materials_traded: Option<u64>,
    #[serde(rename = "Grade_4_Materials_Traded")]
    grade_4_materials_traded: Option<u64>,
    #[serde(rename = "Grade_5_Materials_Traded")]
    grade_5_materials_traded: Option<u64>,
    #[serde(rename = "Materials_Traded")]
    materials_traded: Option<u64>,
    #[serde(rename = "Raw_Materials_Traded")]
    raw_materials_traded: Option<u64>,
    #[serde(rename = "Trades_Completed")]
    trades_completed: u64,
}

#[derive(Debug, Deserialize)]
pub struct MiningStatistics {
    #[serde(rename = "Materials_Collected")]
    materials_collected: u64,
    #[serde(rename = "Mining_Profits")]
    mining_profits: u64,
    #[serde(rename = "Quantity_Mined")]
    quantity_mined: u64,
}

#[derive(Debug, Deserialize)]
pub struct PassengersStatistics {
    #[serde(rename = "Passengers_Missions_Accepted")]
    passengers_missions_accepted: u64,
    #[serde(rename = "Passengers_Missions_Bulk")]
    passengers_missions_bulk: u64,
    #[serde(rename = "Passengers_Missions_Delivered")]
    passengers_missions_delivered: u64,
    #[serde(rename = "Passengers_Missions_Ejected")]
    passengers_missions_ejected: u64,
    #[serde(rename = "Passengers_Missions_VIP")]
    passengers_missions_vip: u64,
}

#[derive(Debug, Deserialize)]
pub struct SearchAndRescueStatistics {
    #[serde(rename = "Maglocks_Opened")]
    maglocks_opened: u64,
    #[serde(rename = "Panels_Opened")]
    panels_opened: u64,
    #[serde(rename = "Salvage_Illegal_POI")]
    salvage_illegal_poi: u64,
    #[serde(rename = "Salvage_Illegal_Settlements")]
    salvage_illegal_settlements: u64,
    #[serde(rename = "Salvage_Legal_POI")]
    salvage_legal_poi: u64,
    #[serde(rename = "Salvage_Legal_Settlements")]
    salvage_legal_settlements: u64,
    #[serde(rename = "SearchRescue_Count")]
    search_rescue_count: u64,
    #[serde(rename = "SearchRescue_Profit")]
    search_rescue_profit: u64,
    #[serde(rename = "SearchRescue_Traded")]
    search_rescue_traded: u64,
    #[serde(rename = "Settlements_State_FireOut")]
    settlements_state_fireout: u64,
    #[serde(rename = "Settlements_State_Reboot")]
    settlements_state_reboot: u64,
}

#[derive(Debug, Deserialize)]
pub struct SmugglingStatistics {
    #[serde(rename = "Average_Profit")]
    average_profit: u64,
    #[serde(rename = "Black_Markets_Profits")]
    black_markets_profits: u64,
    #[serde(rename = "Black_Markets_Traded_With")]
    black_markets_traded_with: u64,
    #[serde(rename = "Highest_Single_Transaction")]
    highest_single_transactions: u64,
    #[serde(rename = "Resources_Smuggled")]
    resources_smuggled: u64,
}

#[derive(Debug, Deserialize)]
pub struct SquadronStatistics {
    #[serde(rename = "Squadron_Bank_Commodities_Deposited_Num")]
    bank_commodities_deposited_num: u64,
    #[serde(rename = "Squadron_Bank_Commodities_Deposited_Value")]
    bank_commodities_deposited_value: u64,
    #[serde(rename = "Squadron_Bank_Commodities_Withdrawn_Num")]
    bank_commodities_withdrawn_num: u64,
    #[serde(rename = "Squadron_Bank_Commodities_Withdrawn_Value")]
    bank_commodities_withdrawn_value: u64,
    #[serde(rename = "Squadron_Bank_Credits_Deposited")]
    bank_credits_deposited: u64,
    #[serde(rename = "Squadron_Bank_Credits_Withdrawn")]
    bank_withdrawn: u64,
    #[serde(rename = "Squadron_Bank_PersonalAssets_Deposited_Num")]
    bank_personal_assets_deposited_num: u64,
    #[serde(rename = "Squadron_Bank_PersonalAssets_Deposited_Value")]
    bank_personal_assets_deposited_value: u64,
    #[serde(rename = "Squadron_Bank_PersonalAssets_Withdrawn_Num")]
    bank_personal_assets_withdrawn_num: u64,
    #[serde(rename = "Squadron_Bank_PersonalAssets_Withdrawn_Value")]
    bank_personal_assets_withdrawn_value: u64,
    #[serde(rename = "Squadron_Bank_Ships_Deposited_Num")]
    bank_ships_deposited_num: u64,
    #[serde(rename = "Squadron_Bank_Ships_Deposited_Value")]
    bank_ships_deposited_value: u64,
    #[serde(rename = "Squadron_Leaderboard_aegis_highestcontribution")]
    leaderbord_aegis_highest_contribution: u64,
    #[serde(rename = "Squadron_Leaderboard_bgs_highestcontribution")]
    leaderbord_bgs_highest_contribution: u64,
    #[serde(rename = "Squadron_Leaderboard_bounty_highestcontribution")]
    leaderbord_bounty_highest_contribution: u64,
    #[serde(rename = "Squadron_Leaderboard_colonisation_contribution_highestcontribution")]
    leaderbord_colonisation_highest_contribution: u64,
    #[serde(rename = "Squadron_Leaderboard_combat_highestcontribution")]
    leaderbord_combat_highest_contribution: u64,
    #[serde(rename = "Squadron_Leaderboard_cqc_highestcontribution")]
    leaderbord_cqc_highest_contribution: u64,
    #[serde(rename = "Squadron_Leaderboard_exploration_highestcontribution")]
    leaderbord_exploration_highest_contribution: u64,
    #[serde(rename = "Squadron_Leaderboard_mining_highestcontribution")]
    leaderbord_mining_highest_contribution: u64,
    #[serde(rename = "Squadron_Leaderboard_podiums")]
    leaderbord_podiums: u64,
    #[serde(rename = "Squadron_Leaderboard_powerplay_highestcontribution")]
    leaderbord_powerplay_highest_contribution: u64,
    #[serde(rename = "Squadron_Leaderboard_trade_highestcontribution")]
    leaderbord_trade_highest_contribution: u64,
    #[serde(rename = "Squadron_Leaderboard_trade_illicit_highestcontribution")]
    leaderbord_trade_illicit_highest_contribution: u64,
}

#[derive(Debug, Deserialize)]
pub struct ThargoidStatistics {
    #[serde(rename = "TG_ENCOUNTER_KILLED")]
    encounter_killed: u64,
    #[serde(rename = "TG_ENCOUNTER_TOTAL")]
    encounter_total: u64,
    #[serde(rename = "TG_ENCOUNTER_TOTAL_LAST_SHIP")]
    encounter_last_ship: String,
    #[serde(rename = "TG_ENCOUNTER_TOTAL_LAST_SYSTEM")]
    encounter_last_system: String,
    #[serde(rename = "TG_ENCOUNTER_TOTAL_LAST_TIMESTAMP")]
    encounter_last_timestamp: String,
}

#[derive(Debug, Deserialize)]
pub struct TradingStatistics {
    #[serde(rename = "Average_Profit")]
    average_profit: f64,
    #[serde(rename = "Assets_Sold")]
    assets_sold: Option<u64>,
    #[serde(rename = "Data_Sold")]
    data_sold: Option<u64>,
    #[serde(rename = "Goods_Sold")]
    goods_sold: Option<u64>,
    #[serde(rename = "Highest_Single_Transaction")]
    highest_single_transaction: u64,
    #[serde(rename = "Market_Profits")]
    market_profits: u64,
    #[serde(rename = "Markets_Traded_With")]
    markets_traded_with: u64,
    #[serde(rename = "Resources_Traded")]
    resources_traded: u64,
}

#[derive(Debug, Deserialize)]
pub struct CrewStatistics {
    #[serde(rename = "NpcCrew_Died")]
    npc_crew_died: u64,
    #[serde(rename = "NpcCrew_Fired")]
    npc_crew_fired: u64,
    #[serde(rename = "NpcCrew_Hired")]
    npc_crew_hired: u64,
    #[serde(rename = "NpcCrew_TotalWages")]
    npc_crew_total_wages: u64,
}

#[derive(Debug, Deserialize)]
pub struct CrimeStatistics {
    #[serde(rename = "Bounties_Received")]
    bounties_received: u64,
    #[serde(rename = "Citizens_Murdered")]
    citizens_murdered: Option<u64>,
    #[serde(rename = "Data_Stolen")]
    data_stolen: Option<u64>,
    #[serde(rename = "Fines")]
    fines: u64,
    #[serde(rename = "Goods_Stolen")]
    goods_stolen: Option<u64>,
    #[serde(rename = "Guards_Murdered")]
    guards_murdered: Option<u64>,
    #[serde(rename = "Highest_Bounty")]
    highest_bounty: u64,
    #[serde(rename = "Malware_Uploaded")]
    malware_uploaded: Option<u64>,
    #[serde(rename = "Notoriety")]
    notoriety: u64,
    #[serde(rename = "Omnipol_Murdered")]
    omnipol_murdered: Option<u64>,
    #[serde(rename = "Production_Sabotage")]
    production_sabotage: Option<u64>,
    #[serde(rename = "Production_Theft")]
    production_theft: Option<u64>,
    #[serde(rename = "Profiles_Cloned")]
    profiles_stolen: Option<u64>,
    #[serde(rename = "Sample_Stolen")]
    sample_stolen: Option<u64>,
    #[serde(rename = "Settlements_State_Shutdown")]
    settlements_state_shutdown: Option<u64>,
    #[serde(rename = "Total_Bounties")]
    total_bounties: u64,
    #[serde(rename = "Total_Fines")]
    total_fines: u64,
    #[serde(rename = "Total_Murders")]
    total_murders: Option<u64>,
    #[serde(rename = "Total_Stolen")]
    total_stolen: Option<u64>,
    #[serde(rename = "Turrets_Destroyed")]
    turrets_destroyed: Option<u64>,
    #[serde(rename = "Turrets_Overloaded")]
    turrets_overloaded: Option<u64>,
    #[serde(rename = "Turrets_Total")]
    turrets_total: Option<u64>,
    #[serde(rename = "Value_Stolen_StateChange")]
    value_stolen_state_change: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct ExobiologyStatistics {
    #[serde(rename = "First_Logged")]
    first_logged: u64,
    #[serde(rename = "First_Logged_Profits")]
    first_logged_profits: u64,
    #[serde(rename = "Organic_Data")]
    organic_data: u64,
    #[serde(rename = "Organic_Data_Profits")]
    organic_data_profits: u64,
    #[serde(rename = "Organic_Genus")]
    organic_genus: u64,
    #[serde(rename = "Organic_Genus_Encountered")]
    organic_genus_encountered: u64,
    #[serde(rename = "Organic_Planets")]
    organic_planets: u64,
    #[serde(rename = "Organic_Species")]
    organic_species: u64,
    #[serde(rename = "Organic_Species_Encountered")]
    organic_species_encountered: u64,
    #[serde(rename = "Organic_Systems")]
    organic_systems: u64,
    #[serde(rename = "Organic_Variant_Encountered")]
    organic_systems_encountered: u64,
}

#[derive(Debug, Deserialize)]
pub struct MulticrewStatistics {
    #[serde(rename = "Multicrew_Credits_Total")]
    multicrew_credits_total: u64,
    #[serde(rename = "Multicrew_Fighter_Time_Total")]
    multicrew_fighter_time_total: u64,
    #[serde(rename = "Multicrew_Fines_Total")]
    multicrew_fines_total: u64,
    #[serde(rename = "Multicrew_Gunner_Time_Total")]
    multicrew_gunner_time_total: u64,
    #[serde(rename = "Multicrew_Time_Total")]
    multicrew_time_total: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StatisticsEvent {
    #[serde(rename = "Bank_Account")]
    bank_account: Option<BankAccountStatistics>,
    #[serde(rename = "CQC")]
    cqc: Option<CqcStatistics>,
    combat: Option<CombatStatistics>,
    exploration: Option<ExplorationStatistics>,
    crafting: Option<CraftingStatistics>,
    crime: Option<CrimeStatistics>,
    exobiology: Option<ExobiologyStatistics>,
    #[serde(rename = "Material_Trader_Stats")]
    material_trading: Option<MaterialTradeStatistics>,
    mining: Option<MiningStatistics>,
    passengers: Option<PassengersStatistics>,
    search_and_rescue: Option<SearchAndRescueStatistics>,
    smuggling: Option<SmugglingStatistics>,
    squadron: Option<SquadronStatistics>,
    thargoid: Option<ThargoidStatistics>,
    trading: Option<TradingStatistics>,
    crew: Option<CrewStatistics>,
    multicrew: Option<MulticrewStatistics>,
}
