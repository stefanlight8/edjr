use {
    crate::elite::{
        allegiance::Allegiance,
        backpack::BackpackItem,
        body::BodyType,
        combat::Killer,
        commander::{Commander, CommanderPackage},
        community_goal::CommunityGoal,
        crew::{Crew, CrewRole},
        crime::CrimeType,
        engineer::Engineer,
        faction::{Faction, FactionConflict},
        fleet_carriers::DockingAccess,
        material::{Material, TraderType},
        mission::{FactionEffect, Mission},
        module::ModuleEngineering,
        passenger::PassengerType,
        powerplay::Powerplay,
        rank::{EmpireRank, FederationRank, Rank},
        ship::{FuelCapacity, Ship, ShipModule},
        signal::{Signal, SignalType},
        station::{Station, StationType},
        system::System,
        vessel::Vessel,
    },
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Reward {
    faction: String,
    reward: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Cargo {
    #[serde(alias = "Name", alias = "Type")]
    name: String,
    #[serde(alias = "Name_Localised", alias = "Type_Localised")]
    name_display: Option<String>,
    count: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Resource {
    name: String,
    #[serde(alias = "Name_Localised")]
    name_display: Option<String>,
    payment: u64,
    required_amount: u64,
    provided_amount: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LandingPads {
    large: u64,
    medium: u64,
    small: u64,
}

#[derive(Debug, Deserialize)]
pub enum FriendStatus {
    Added,
    Declined,
    Lost,
    Offline,
    Online,
    Requested,
}

#[derive(Debug, Deserialize)]
pub enum LaunchDroneType {
    Collection,
    Repair,
    FuelTransfer,
}

#[derive(Debug, Deserialize)]
pub enum GameMode {
    Open,
    Solo,
    Group,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Module {
    slot: String,
    name: String,
    #[serde(alias = "Name_Localised")]
    name_display: String,
    hot: bool,
    engineer_modifications: Option<String>,
    level: Option<u8>,
    quality: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub enum Target {
    You,
    Mothership,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Discover {
    num_bodies: u64,
    system_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PassengersManifest {
    #[serde(alias = "MissionID")]
    mission_id: u64,
    count: u64,
    #[serde(alias = "Type")]
    passengers_type: PassengerType,
    #[serde(default, alias = "VIP")]
    vip: bool,
    #[serde(default)]
    wanted: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Channel {
    Local,
    Npc,
    Player,
    StarSystem,
    Wing,
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
pub struct FactionVoucher {
    faction: String,
    amount: u64,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "event", rename_all_fields = "PascalCase")]
pub enum JournalEvent {
    AfmuRepair {
        fully_repaired: bool,
        health: f64,
        module: String,
        #[serde(alias = "Module_Localised")]
        module_display: String,
    },
    ApproachBody {
        body: String,
        #[serde(alias = "BodyID")]
        body_id: u64,
        star_system: String,
        system_address: u64,
    },
    ApproachSettlement {
        #[serde(alias = "BodyID")]
        body_id: u64,
        body_name: String,
        latitude: f64,
        longitude: f64,
        #[serde(alias = "MarketID")]
        market_id: Option<u64>,
        name: String,
        #[serde(alias = "Name_Localised")]
        name_display: Option<String>,
        #[serde(flatten)]
        station: Option<Station>,
        system_address: u64,
    },
    Backpack {
        components: Option<Vec<BackpackItem>>,
        consumables: Option<Vec<BackpackItem>>,
        data: Option<Vec<BackpackItem>>,
        items: Option<Vec<BackpackItem>>,
        // I guess, TODO: need correction
    },
    BackpackChange {
        added: Option<Vec<BackpackItem>>,
        removed: Option<Vec<BackpackItem>>,
    },
    BookTaxi {
        cost: u64,
        destination_location: String,
        destination_system: String,
    },
    Bounty {
        pilot_name: String,
        #[serde(alias = "PilotName_Localised")]
        pilot_name_display: Option<String>,
        rewards: Vec<Reward>,
        shared_with_others: Option<u64>,
        target: String,
        #[serde(alias = "Target_Localised")]
        target_display: Option<String>,
        total_reward: u64,
        victim_faction: String,
        #[serde(alias = "VictimFaction_Localised")]
        victim_faction_display: Option<String>,
    },
    BuyAmmo {
        cost: u64,
    },
    BuyDrones {
        buy_price: u64,
        count: u64,
        total_cost: u64,
    },
    BuyWeapon {
        class: u64,
        name: String,
        #[serde(alias = "Name_Localised")]
        name_display: String,
        price: u64,
        #[serde(alias = "SuitModuleID")]
        suit_module_id: u64,
        // TODO: WeaponMods
    },
    Cargo {
        count: u64,
        vessel: Vessel,
        inventory: Option<Vec<Cargo>>,
    },
    CargoDepot {
        cargo_type: String,
        count: u64,
        progress: f64,
        items_collected: u64,
        items_delivered: u64,
        total_items_to_deliver: u64,
        #[serde(alias = "MissionID")]
        mission_id: u64,
        #[serde(alias = "StartMarketID")]
        start_market_id: u64,
        #[serde(alias = "EndMarketID")]
        end_market_id: u64,
    },
    CargoTransfer {
        transfers: Vec<Cargo>,
    },
    CarrierJump {
        body: String,
        #[serde(alias = "BodyID")]
        body_id: u64,
        body_type: BodyType,
        conflicts: Option<Vec<FactionConflict>>,
        controlling_power: Option<String>,
        factions: Option<Vec<Faction>>,
        #[serde(alias = "MarketID")]
        market_id: Option<u64>,
        #[serde(flatten)]
        powerplay: Option<Powerplay>,
        star_pos: [f64; 3],
        star_system: String,
        #[serde(flatten)]
        station: Option<Station>,
        system_address: u64,
        #[serde(default)]
        docked: bool,
        #[serde(default)]
        taxi: bool,
        #[serde(default)]
        multicrew: bool,
        #[serde(default)]
        on_foot: bool,
    },
    ChangeCrewRole {
        role: CrewRole,
        #[serde(default)]
        telepresence: bool,
    },
    ClearImpound {
        #[serde(alias = "MarketID")]
        market_id: Option<u64>,
        #[serde(alias = "ShipID")]
        ship_id: Option<u64>,
        #[serde(alias = "ShipMarketID")]
        ship_market_id: Option<u64>,
        ship_type: String,
        system: String,
    },
    CockpitBreached,
    CodexEntry {
        #[serde(alias = "BodyID")]
        body_id: u64,
        category: String,
        #[serde(alias = "Category_Localised")]
        category_display: String,
        #[serde(default)]
        is_new_entry: bool,
        name: String,
        #[serde(alias = "Name_Localised")]
        name_display: String,
        nearest_destination: Option<String>,
        region: String,
        #[serde(alias = "Region_Localised")]
        region_display: String,
        sub_category: String,
        #[serde(alias = "SubCategory_Localised")]
        sub_category_display: String,
        system_address: u64,
        voucher_amount: Option<u64>,
    },
    CollectCargo {
        stolen: bool,
        #[serde(flatten)]
        cargo: Cargo,
        #[serde(alias = "MarketID")]
        market_id: Option<u64>,
    },
    // TODO: collect items
    ColonisationConstructionDepot {
        construction_complete: bool,
        construction_failed: bool,
        construction_progress: f64,
        #[serde(alias = "MarketID")]
        market_id: Option<u64>,
        resources_required: Vec<Resource>,
    },
    Commander {
        #[serde(flatten)]
        commander: Commander,
    },
    CommitCrime {
        crime_type: CrimeType,
        faction: String,
        bounty: Option<u64>,
        fine: Option<u64>,
        victim: Option<String>,
        #[serde(alias = "Victim_Localised")]
        victim_display: Option<String>,
    },
    CommunityGoal {
        current_goals: Vec<CommunityGoal>,
    },
    CommunityGoalDiscard {
        #[serde(alias = "CGID")]
        cgid: u64,
        name: String,
        system: String,
    },
    CommunityGoalJoin {
        #[serde(alias = "CGID")]
        cgid: u64,
        name: String,
        system: String,
    },
    CommunityGoalReward {
        #[serde(alias = "CGID")]
        cgid: u64,
        name: String,
        reward: u64,
        system: String,
    },
    CrewAssign {
        #[serde(alias = "CrewID")]
        crew_id: u64,
        name: String,
        role: String,
    },
    CrewLaunchFighter {
        crew: String,
        #[serde(default)]
        telepresense: bool,
    },
    CrewMemberJoins {
        crew: String,
        #[serde(default)]
        telepresense: bool,
    },
    CrewMemberQuits {
        crew: String,
        #[serde(default)]
        telepresense: bool,
    },
    CrewMemberRoleChange {
        crew: String,
        role: CrewRole,
        #[serde(default)]
        telepresense: bool,
    },
    CrimeVictim {
        crime_type: CrimeType,
        offender: String,
        bounty: Option<u64>,
        fine: Option<u64>,
    },
    DataScanned {
        #[serde(alias = "Type")]
        data_type: String,
        #[serde(alias = "Type_Localised")]
        data_type_display: Option<String>,
    },
    DatalinkScan {
        message: String,
        #[serde(alias = "Message_Localised")]
        message_display: String,
    },
    DatalinkVoucher {
        reward: u64,
        payee_faction: Allegiance,
        victim_faction: Option<Allegiance>,
    },
    Died {
        killer_name: Option<String>,
        #[serde(alias = "KillerName_Localised")]
        killer_name_display: Option<String>,
        killer_rank: Option<String>, // lol frontier there uses a string instead of their combat rank...
        killer_ship: Option<String>,
        killers: Option<Vec<Killer>>,
    },
    Disembark {
        body: String,
        #[serde(alias = "BodyID")]
        body_id: u64,
        #[serde(alias = "ID")]
        id: Option<u64>,
        #[serde(alias = "MarketID")]
        market_id: Option<u64>,
        star_system: String,
        station_name: Option<String>,
        station_type: Option<StationType>,
        system_address: u64,
        #[serde(default)]
        multicrew: bool,
        #[serde(default)]
        on_planet: bool,
        #[serde(default)]
        on_station: bool,
        #[serde(default, alias = "SRV")]
        srv: bool,
        #[serde(default)]
        taxi: bool,
    },
    DockFighter {
        #[serde(alias = "ID")]
        id: u64,
    },
    #[serde(alias = "DockSRV")]
    DockSrv {
        #[serde(alias = "ID")]
        id: u64,
        #[serde(alias = "SRVType")]
        srv_type: String,
        #[serde(alias = "SRVType_Localised")]
        srv_type_display: String,
    },
    Docked {
        dist_from_star_ls: Option<f64>,
        landing_pads: Option<LandingPads>,
        #[serde(alias = "MarketID")]
        market_id: Option<u64>,
        star_system: String,
        #[serde(flatten)]
        station: Option<Station>,
        system_address: u64,
        #[serde(default)]
        multicrew: bool,
        #[serde(default)]
        active_fine: bool,
        #[serde(default)]
        cockpit_breach: bool,
        #[serde(default)]
        taxi: bool,
        #[serde(default)]
        wanted: bool,
    },
    DockingCancelled {
        #[serde(alias = "MarketID")]
        market_id: Option<u64>,
        station_name: String,
        station_type: StationType,
    },
    DockingGranted {
        landing_pad: u64,
        #[serde(alias = "MarketID")]
        market_id: Option<u64>,
        station_name: String,
        station_type: StationType,
    },
    DockingRequested {
        landing_pads: Option<LandingPads>,
        #[serde(alias = "MarketID")]
        market_id: Option<u64>,
        station_name: String,
        station_type: StationType,
    },
    DockingTimeout {
        #[serde(alias = "MarketID")]
        market_id: Option<u64>,
        station_name: String,
        station_type: StationType,
    },
    EjectCargo {
        abandoned: bool,
        #[serde(flatten)]
        cargo: Cargo,
    },
    Embark {
        body: String,
        #[serde(alias = "BodyID")]
        body_id: u64,
        crew: Option<Vec<Crew>>,
        #[serde(alias = "ID")]
        id: Option<u64>,
        #[serde(alias = "MarketID")]
        market_id: Option<u64>,
        multicrew: bool,
        on_planet: bool,
        on_station: bool,
        #[serde(alias = "SRV")]
        srv: bool,
        star_system: String,
        station_name: Option<String>,
        station_type: Option<StationType>,
        system_address: u64,
        taxi: bool,
    },
    EndCrewSession {
        on_crime: bool,
        #[serde(default)]
        telepresence: bool,
    },
    EngineerContribution {
        commodity: Option<String>,
        #[serde(alias = "Commodity_Localised")]
        commodity_display: Option<String>,
        #[serde(flatten)]
        engineer: Engineer,
        quantity: u64,
        total_quantity: u64,
        #[serde(alias = "Type")]
        contribution_type: String,
    },
    EngineerCraft {
        apply_experimental_effect: Option<String>,
        ingredients: Vec<Material>,
        #[serde(flatten)]
        engineer: Option<Engineer>,
        #[serde(flatten)]
        modification: ModuleEngineering,
    },
    EngineerProgress {
        #[serde(flatten)]
        engineer: Option<Engineer>,
        engineers: Option<Vec<Engineer>>,
    },
    EspaceInterdiction {
        inderdictor: String,
        #[serde(alias = "Interdictor_Localised")]
        inderdictor_display: Option<String>,
        is_player: bool,
    },
    #[serde(alias = "FCMaterials")]
    FcMaterials {
        carrier_name: String,
        #[serde(alias = "CarrierID")]
        carrier_id: String,
        #[serde(alias = "MarketID")]
        market_id: Option<u64>,
    },
    #[serde(alias = "FSDJump")]
    FsdJump {
        body: String,
        #[serde(alias = "BodyID")]
        body_id: u64,
        body_type: BodyType,
        boost_used: Option<u8>,
        conflicts: Option<Vec<FactionConflict>>,
        controlling_power: Option<String>,
        factions: Option<Vec<Faction>>,
        fuel_level: f64,
        jump_dist: f64,
        population: u64,
        #[serde(flatten)]
        powerplay: Option<Powerplay>,
        star_pos: [f64; 3],
        star_system: String,
        #[serde(flatten)]
        system: Option<System>,
        system_address: u64,
        #[serde(default)]
        multicrew: bool,
        #[serde(default)]
        taxi: bool,
        // TODO: ThargoidWar
    },
    #[serde(alias = "FSDTarget")]
    FsdTarget {
        name: String,
        star_class: String,
        system_address: u64,
        remaining_jumps_in_route: Option<u64>,
    },
    FSSAllBodiesFound {
        count: u64,
        system_address: u64,
        system_name: String,
    },
    FSSBodySignals {
        #[serde(alias = "BodyID")]
        body_id: u64,
        body_name: String,
        system_address: u64,
        signals: Vec<Signal>,
    },
    FSSDiscoveryScan {
        body_count: u64,
        non_body_count: u64,
        progress: f64,
        system_address: u64,
        system_name: String,
    },
    FSSSignalDiscovered {
        #[serde(default)]
        is_station: bool,
        opposing_power: Option<String>,
        signal_name: String,
        #[serde(alias = "SignalName_Localised")]
        signal_name_display: Option<String>,
        signal_type: Option<SignalType>,
        spawning_faction: Option<String>,
        #[serde(alias = "SpawningFaction_Localised")]
        spawning_faction_display: Option<String>,
        spawning_power: Option<String>,
        spawning_state: Option<String>,
        #[serde(alias = "SpawningState_Localised")]
        spawning_state_display: Option<String>,
        system_address: u64,
        threat_level: Option<u8>,
        time_remaining: Option<f64>,
        #[serde(alias = "USSType")]
        uss_type: Option<String>,
        #[serde(alias = "USSType_Localised")]
        uss_type_display: Option<String>,
    },
    FactionKillBond {
        awarding_faction: String,
        #[serde(alias = "AwardingFaction_Localised")]
        awarding_faction_display: Option<String>,
        reward: u64,
        victim_faction: String,
        #[serde(alias = "VictimFaction_Localised")]
        victim_faction_display: Option<String>,
    },
    FetchRemoteModule {
        server_id: u64, // ServerId – nicee!
        ship: String,
        #[serde(alias = "ShipID")] // ShipID – whyyy...
        ship_id: u64,
        storage_slot: u64,
        stored_item: String,
        #[serde(alias = "StoredItem_Localised")]
        stored_item_display: Option<String>,
        transfer_cost: u64,
        transfer_time: u64,
    },
    FighterDestroyed {
        #[serde(alias = "ID")]
        id: u64,
    },
    FighterRebuilt {
        #[serde(alias = "ID")]
        id: u64,
        loadout: String,
    },
    Fileheader {
        odyssey: bool,
        #[serde(alias = "build")]
        build: String,
        #[serde(alias = "gameversion")]
        game_version: String,
        #[serde(alias = "language")]
        language: String,
        #[serde(alias = "part")]
        part: u64,
    },
    Friends {
        name: String,
        status: FriendStatus,
    },
    FuelScoop {
        scooped: f64,
        total: f64,
    },
    HeatDamage,
    HeatWarning,
    HullDamage {
        #[serde(default)]
        fighter: bool,
        health: f64,
        player_pilot: bool,
    },
    Interdicted {
        interdictor: String,
        #[serde(alias = "Interdictor_Localised")]
        interdictor_display: Option<String>,
        is_player: bool,
        submitted: bool,
        combat_rank: Option<Rank>,
        faction: Option<String>,
    },
    Interdiction {
        is_player: bool,
        success: bool,
        power: Option<Allegiance>,
        inderdicted: Option<String>,
        combat_rank: Option<Rank>,
        faction: Option<String>,
    },
    InvitedToSquadron {
        squadron_name: String,
    },
    JetConeBoost {
        boost_value: f64,
    },
    JetConeDamage {
        module: String,
        #[serde(alias = "Module_Localised")]
        module_display: String,
    },
    JoinACrew {
        captain: String,
        #[serde(default)]
        telepresence: bool,
    },
    JoinedSquadron {
        #[serde(alias = "SquadronID")]
        squadron_id: u64,
        squadron_name: String,
    },
    LaunchDrone {
        #[serde(alias = "Type")]
        launch_type: LaunchDroneType,
    },
    LaunchFighter {
        #[serde(alias = "ID")]
        id: u64,
        loadout: String,
        player_controlled: bool,
    },
    #[serde(alias = "LaunchSRV")]
    LaunchSrv {
        #[serde(alias = "ID")]
        id: u64,
        loadout: String,
        player_controlled: bool,
        #[serde(alias = "SRVType")]
        srv_type: String,
        #[serde(alias = "SRVType_Localised")]
        srv_type_display: String,
    },
    LeaveBody {
        body: String,
        #[serde(alias = "BodyID")]
        body_id: u64,
        star_system: String,
        system_address: u64,
    },
    LeftSquadron {
        #[serde(alias = "SquadronID")]
        squadron_id: Option<u64>,
        squadron_name: Option<String>,
    },
    Liftoff {
        system_address: u64,
        body: Option<String>,
        #[serde(alias = "BodyID")]
        body_id: u64,
        latitude: f64,
        longitude: f64,
        nearest_destination: Option<String>,
        #[serde(alias = "NearestDestination_Localised")]
        nearest_destination_display: Option<String>,
        #[serde(default)]
        on_planet: bool,
        #[serde(default)]
        on_station: bool,
        #[serde(default)]
        player_controlled: bool,
        #[serde(default)]
        multicrew: bool,
        #[serde(default)]
        taxi: bool,
    },
    LoadGame {
        commander: String,
        credits: u64,
        #[serde(alias = "FID")]
        fid: String,
        fuel_capacity: Option<f64>,
        fuel_level: Option<f64>,
        game_mode: Option<GameMode>,
        group: Option<String>,
        loan: u64,
        #[serde(default)]
        horizons: bool,
        #[serde(default)]
        odyssey: bool,
        #[serde(flatten)]
        ship: Option<Ship>,
        #[serde(default)]
        start_dead: bool,
        #[serde(default)]
        start_landed: bool,
        #[serde(alias = "build")]
        build: Option<String>,
        #[serde(alias = "gameversion")]
        game_version: Option<String>,
        #[serde(alias = "language")]
        language: Option<String>,
    },
    Loadout {
        cargo_capacity: u64,
        fuel_capacity: FuelCapacity,
        #[serde(default)]
        hot: bool,
        hull_health: f64,
        #[serde(default)]
        hull_value: u64,
        max_jump_range: f64,
        modules: Vec<ShipModule>,
        #[serde(default)]
        modules_value: u64,
        rebuy: u64,
        #[serde(flatten)]
        ship: Ship,
        unladen_mass: f64,
    },
    Location {
        body: String,
        #[serde(alias = "BodyID")]
        body_id: u64,
        body_type: BodyType,
        conflicts: Option<Vec<FactionConflict>>,
        controlling_power: Option<String>,
        dist_from_star_ls: Option<f64>,
        factions: Option<Vec<Faction>>,
        #[serde(default, alias = "InSRV")]
        in_srv: bool,
        latitude: Option<f64>,
        longitude: Option<f64>,
        #[serde(default)]
        docked: bool,
        #[serde(default)]
        multicrew: bool,
        #[serde(default)]
        on_foot: bool,
        population: u64,
        #[serde(flatten)]
        powerplay: Option<Powerplay>,
        #[serde(flatten)]
        system: Option<System>,
        system_address: u64,
        #[serde(flatten)]
        station: Option<Station>,
        star_pos: [f64; 3],
        star_system: String,
        #[serde(default)]
        taxi: bool,
        #[serde(default)]
        wanted: bool,
        // TODO: thargoid war
    },
    Market {
        #[serde(alias = "MarketID")]
        market_id: u64,
        star_system: String,
        station_name: String,
        station_type: StationType,
        carrier_docking_access: Option<DockingAccess>,
    },
    MarketBuy {
        buy_price: u64,
        count: u64,
        #[serde(alias = "MarketID")]
        market_id: Option<u64>,
        total_cost: u64,
        #[serde(alias = "Type")]
        commodity: String,
        #[serde(alias = "Type_Localised")]
        commodity_display: Option<String>,
    },
    MarketSell {
        avg_price_paid: u64,
        sell_price: u64,
        count: u64,
        #[serde(alias = "MarketID")]
        market_id: Option<u64>,
        total_sale: u64,
        #[serde(alias = "Type")]
        commodity: String,
        #[serde(alias = "Type_Localised")]
        commodity_display: Option<String>,
    },
    MassModuleStore {
        items: Vec<Module>,
        #[serde(alias = "MarketID")]
        market_id: Option<u64>,
        #[serde(flatten)]
        ship: Ship,
    },
    MaterialCollected {
        #[serde(flatten)]
        material: Material,
    },
    MaterialDiscovered {
        #[serde(flatten)]
        material: Material,
    },
    MaterialTrade {
        #[serde(alias = "MarketID")]
        market_id: u64,
        paid: Material,
        received: Material,
        trader_type: TraderType,
    },
    Materials {
        encoded: Vec<Material>,
        manufactured: Vec<Material>,
        raw: Vec<Material>,
    },
    MissionAbandoned {
        #[serde(flatten)]
        mission: Mission,
    },
    MissionAccepted {
        #[serde(flatten)]
        mission: Mission,
    },
    MissionCompleted {
        #[serde(flatten)]
        mission: Mission,
        donated: Option<u64>,
        faction_effects: Vec<FactionEffect>,
        materials_reward: Option<Vec<Material>>,
    },
    MissionFailed {
        #[serde(flatten)]
        mission: Mission,
    },
    MissionRedirected {
        #[serde(flatten)]
        mission: Mission,
        new_destination_station: String,
        new_destination_system: String,
        old_destination_station: String,
        old_destination_system: String,
    },
    Missions {
        active: Option<Vec<Mission>>,
        complete: Option<Vec<Mission>>,
        failed: Option<Vec<Mission>>,
    },
    ModuleBuy {
        #[serde(alias = "MarketID")]
        market_id: u64,
        buy_item: String,
        #[serde(alias = "BuyItem_Localised")]
        buy_item_display: String,
        buy_price: u64,
        sell_item: Option<String>,
        #[serde(alias = "SellItem_Localised")]
        sell_item_display: Option<String>,
        sell_price: Option<u64>,
        ship: String,
        #[serde(alias = "ShipID")]
        ship_id: u64,
        slot: String,
        stored_item: Option<String>,
        #[serde(alias = "StoredItem_Localised")]
        stored_item_display: Option<String>,
    },
    ModuleBuyAndStore {
        #[serde(alias = "MarketID")]
        market_id: u64,
        buy_item: String,
        #[serde(alias = "BuyItem_Localised")]
        buy_item_display: String,
        buy_price: u64,
        ship: String,
        #[serde(alias = "ShipID")]
        ship_id: u64,
    },
    ModuleInfo, // I don't know, my scheme doesn't contain any information, TODO: fill module info if there's any information
    ModuleRetrieve {
        ship: String,
        #[serde(alias = "ShipID")]
        ship_id: u64,
        engineer_modifications: Option<String>,
        level: Option<u8>,
        quality: Option<f64>,
        slot: String,
        retrieved_item: String,
        #[serde(alias = "RetrievedItem_Localised")]
        retrieved_item_display: String,
        swap_out_item: Option<String>,
        #[serde(alias = "SwapOutItem_Localised")]
        swap_out_item_display: Option<String>,
        #[serde(default)]
        hot: bool,
    },
    ModuleSell {
        #[serde(alias = "MarketID")]
        market_id: u64,
        sell_item: String,
        #[serde(alias = "SellItem_Localised")]
        sell_item_display: String,
        sell_price: u64,
        ship: String,
        #[serde(alias = "ShipID")]
        ship_id: u64,
        slot: String,
    },
    ModuleSellRemote {
        server_id: u64,
        sell_item: String,
        #[serde(alias = "SellItem_Localised")]
        sell_item_display: String,
        sell_price: u64,
        ship: String,
        #[serde(alias = "ShipID")]
        ship_id: u64,
        storage_slot: u16,
    },
    ModuleStore {
        ship: String,
        #[serde(alias = "ShipID")]
        ship_id: u64,
        engineer_modifications: Option<String>,
        level: Option<u8>,
        quality: Option<f64>,
        slot: String,
        stored_item: String,
        #[serde(alias = "StoredItem_Localised")]
        stored_item_display: String,
        #[serde(default)]
        hot: bool,
    },
    ModuleSwap {
        #[serde(alias = "MarketID")]
        market_id: u64,
        ship: String,
        #[serde(alias = "ShipID")]
        ship_id: u64,
        from_slot: String,
        from_item: String,
        #[serde(alias = "FromItem_Localised")]
        from_item_display: String,
        to_slot: String,
        to_item: Option<String>,
        #[serde(alias = "ToItem_Localised")]
        to_item_display: Option<String>,
        #[serde(default)]
        hot: bool,
    },
    MultiSellExplorationData {
        base_value: u64,
        bonus: u64,
        discovered: Vec<Discover>,
        total_earnings: u64,
    },
    Music {
        music_track: String,
    },
    NavBeaconScan {
        num_bodies: u64,
        system_address: u64,
    },
    NavRoute,
    NavRouteClear,
    NewCommander {
        #[serde(flatten)]
        commander: Commander,
        package: CommanderPackage,
    },
    NpcCrewPaidWage {
        amount: u64,
        npc_crew_id: u64,
        npc_crew_name: String,
    },
    Outfitting {
        #[serde(alias = "MarketID")]
        market_id: u64,
        star_system: String,
        station_name: String,
    },
    #[serde(alias = "PVPKill")]
    PvpKill {
        combat_rank: Rank,
        victim: String,
    },
    Passengers {
        manifest: Vec<PassengersManifest>,
    },
    PayBounties {
        all_fines: bool,
        amount: u64,
        broker_percentage: Option<f64>,
        faction: Option<String>,
        #[serde(alias = "ShipID")]
        ship_id: u64,
    },
    PayFines {
        all_fines: bool,
        amount: u64,
        broker_percentage: Option<f64>,
        faction: Option<String>,
        #[serde(alias = "ShipID")]
        ship_id: u64,
    },
    Powerplay {
        merits: u64,
        power: String,
        rank: u64,
        time_pledged: u64,
    },
    PowerplayCollect {
        power: String,
        count: u64,
        #[serde(alias = "Type")]
        collected: String,
        #[serde(alias = "Type_Localised")]
        collected_display: String,
    },
    PowerplayMerits {
        merits_gained: u64,
        power: String,
        total_merits: u64,
    },
    PowerplayRank {
        power: String,
        rank: u64,
    },
    Progress {
        #[serde(alias = "CQC")]
        cqc: u64,
        combat: u64,
        empire: u64,
        exobiologist: u64,
        explore: u64,
        federation: u64,
        soldier: u64,
        trade: u64,
    },
    Promotion {
        combat: Option<Rank>,
        soldier: Option<Rank>,
        empire: Option<EmpireRank>,
        explore: Option<Rank>,
        exobiologist: Option<Rank>,
        federation: Option<FederationRank>,
        trade: Option<Rank>,
    },
    QuitACrew {
        captain: String,
    },
    Rank {
        #[serde(alias = "CQC")]
        cqc: Rank,
        combat: Rank,
        empire: Rank,
        exobiologist: Rank,
        explore: Rank,
        federation: Rank,
        soldier: Rank,
        trade: Rank,
    },
    RebootRepair {
        modules: Vec<String>,
    },
    ReceiveText {
        channel: Channel,
        from: String,
        #[serde(alias = "From_Localised")]
        from_display: Option<String>,
        message: String,
        #[serde(alias = "Message_Localised")]
        message_display: Option<String>,
    },
    RedeemVoucher {
        amount: u64,
        broker_percentage: Option<f64>,
        faction: Option<String>,
        factions: Option<Vec<FactionVoucher>>,
        #[serde(alias = "Type")]
        voucher_type: VoucherType,
    },
    RefuelAll {
        amount: f64,
        cost: u64,
    },
    RefuelPartial {
        amount: f64,
        cost: u64,
    },
    Repair {
        cost: u64,
        items: Vec<String>,
    },
    RepairAll {
        cost: u64,
    },
    RepairDrone {
        hull_repaired: f64,
        cockpit_repaired: Option<f64>,
        corrosion_repaired: Option<f64>,
    },
    Reputation {
        alliance: Option<f64>,
        empire: Option<f64>,
        federation: Option<f64>,
        independent: Option<f64>,
    },
    SelfDestruct,
    #[serde(alias = "USSDrop")]
    UssDrop {
        #[serde(alias = "USSThreat")]
        uss_threat: u8,
        #[serde(alias = "USSType")]
        uss_type: String,
        #[serde(alias = "USSType_Localised")]
        uss_type_display: String,
    },
    UnderAttack {
        target: Target,
    },
    Undocked {
        station_name: String,
        station_type: StationType,
        #[serde(alias = "MarketID")]
        market_id: u64,
        #[serde(default)]
        multicrew: bool,
        #[serde(default)]
        taxi: bool,
    },
    UseConsumable {
        name: String,
        #[serde(alias = "Name_Localised")]
        name_display: String,
    },
    WingAdd {
        name: String,
    },
    WingInvite {
        name: String,
    },
    WingJoin {
        others: Option<Vec<String>>,
    },
    WingLeave,
    #[serde(other)]
    Unknown,
}
