use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Allegiance {
    PilotsFederation,
    Alliance,
    Empire,
    Federation,
    Independent,
    Guardian,
    Thargoid,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StationEconomy {
    name: String,
    #[serde(alias = "Name_Localised")]
    name_display: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum FactionState {
    Boom,
    Bust,
    Election,
    Expansion,
    Investment,
    None,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Faction {
    name: String,
    faction_state: Option<FactionState>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StationService {
    Dock,
    Autodock,
    BlackMarket,
    Commodities,
    Contacts,
    Exploration,
    Rearm,
    Missions,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
pub enum Vessel {
    Ship,
    #[serde(alias = "SRV")]
    Srv,
}

#[derive(Debug, Deserialize)]
pub enum BodyType {
    Null,
    Planet,
    PlanetaryRing,
    Star,
    Station,
    StellarRing,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Station {
    station_allegiance: Allegiance,
    station_economies: Vec<StationEconomy>,
    station_economy: String,
    #[serde(alias = "SystemEconomy_Localised")]
    station_economy_display: String,
    system_second_economy: Option<String>,
    #[serde(alias = "SystemSecondEconomy_Localised")]
    system_second_economy_display: Option<String>,
    station_faction: Faction,
    station_goverment: String,
    #[serde(alias = "SystemGoverment_Localised")]
    station_goverment_localised: String,
    station_services: Vec<StationService>,
    station_state: Option<StationState>,
    station_type: Option<StationType>,
    system_security: Option<String>,
    #[serde(alias = "SystemSecurity_Localised")]
    system_security_display: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Consumable {
    name: String,
    #[serde(alias = "Name_Localised")]
    name_display: Option<String>,
    owner_id: u64,
}

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
    name_display: String,
    count: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PowerplayConflict {
    power: String,
    conflict_progress: f64,
}

#[derive(Debug, Deserialize, Default)]
pub enum PowerplayState {
    Exploited,
    #[default]
    Unoccupied,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Powerplay {
    powerplay_conflict_progress: Vec<PowerplayConflict>,
    powerplay_state: PowerplayState,
    powerplay_state_control_progress: f64,
    powerplay_state_reinforcement: u64,
    powerplay_state_undermining: u64,
    powers: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub enum CrewRole {
    FighterCon,
    FireCon,
    Helm,
    Idle,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Resource {
    name: String,
    #[serde(alias = "Name_Localised")]
    name_display: Option<String>,
    count: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CrimeType {
    Assault,
    CollidedAtSpeedInNoFireZone,
    CollidedAtSpeedInNoFireZoneHullDamage,
    DockingMajorBlockingAirlock,
    DockingMajorBlockingLandingPad,
    DockingMajorTresspass,
    DockingMinorBlockingAirlock,
    DockingMinorBlockingLandingPad,
    DockingMinorTresspass,
    FireInNoFireZone,
    Inderdiction,
    Murder,
    #[serde(alias = "onFoot_damagingDefences")]
    OnFootDamagingDefences,
    #[serde(alias = "onFoot_identifyTheft")]
    OnFootIdentifyTheft,
    #[serde(alias = "onFoot_murder")]
    OnFootMurder,
    #[serde(alias = "onFoot_trespass")]
    OnFootTrespass,
    RecklessWeaponsDischarge,
    ShuttleDestruction,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommunityGoal {
    #[serde(alias = "CGID")]
    cgid: u64,
    title: String,
}

#[derive(Debug, Deserialize)]
pub enum CombatRank {
    Harmless,
    MostlyHarmless,
    Novice,
    Competent,
    Expert,
    Master,
    Dangerous,
    Deadly,
    Elite,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Killer {
    name: String,
    ship: String,
    rank: CombatRank,
}

#[derive(Debug, Deserialize)]
pub enum StationType {
    Coriolis,
    Dodec,
    Orbis,
    Ocellus,
    Outpost,
    CraterOutpost,
    CraterPort,
    SurfaceStation,
    OnFootSettlement,
    MegaShip,
    FleetCarrier,
    Bernal,
    AsteroidBase,
    PlanetaryConstructionDepot,
    SpaceConstructionDepot,
}

#[derive(Debug, Deserialize)]
pub enum StationState {
    Construction,
    UnderAttack,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LandingPads {
    large: u64,
    medium: u64,
    small: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Modifier {
    label: String,
    value: f64,
    original_value: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Engineer {
    engineer: String,
    #[serde(alias = "EngineerID")]
    engineer_id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Signal {
    #[serde(alias = "Type")]
    signal_type: String,
    #[serde(alias = "Type_Localised")]
    signal_type_display: String,
}

#[derive(Debug, Deserialize)]
pub enum SignalType {
    Codex,
    Combat,
    FleetCarrier,
    Generic,
    Installation,
    Megaship,
    NavBeacon,
    Outpost,
    ResourceExtraction,
    SquadronCarrier,
    StationAsteroid,
    StationBernalSphere,
    StationCoriolis,
    StationDodec,
    StationMegaShip,
    StationONeilCylinder,
    StationONeilOrbis,
    Titan,
    TouristBeacon,
    #[serde(alias = "USS")]
    Uss,
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
    Refuel,
}

#[derive(Debug, Deserialize)]
pub enum GameMode {
    Open,
    Solo,
    Group,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ship {
    ship: String,
    #[serde(default, alias = "Ship_Localised")]
    ship_display: String,
    #[serde(alias = "ShipID")]
    ship_id: u64,
    ship_ident: Option<String>,
    ship_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Module {
    slot: String,
    item: String,
    on: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FuelCapacity {
    main: f64,
    reserve: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct System {
    system_allegiance: Option<Allegiance>,
    system_economy: String,
    #[serde(alias = "SystemEconomy_Localised")]
    system_economy_display: String,
    system_faction: Option<Faction>,
    system_goverment: String,
    #[serde(alias = "SystemGoverment_Localised")]
    system_goverment_display: String,
    system_second_economy: String,
    #[serde(alias = "SystemSecondEconomy_Localised")]
    system_second_economy_display: String,
    system_security: String,
    #[serde(alias = "SystemSecurity_Localised")]
    system_security_display: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DockingAccess {
    All,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Crew {
    name: String,
    role: CrewRole,
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
        // TODO: components, data, items
        consumables: Vec<Consumable>,
    },
    BackpackChange {
        added: Vec<Consumable>,
        removed: Vec<Consumable>,
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
        // type: String
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
        inventory: Vec<Cargo>,
        vessel: Vessel,
    },
    CargoTransfer {
        transfers: Vec<Cargo>,
    },
    CarrierJump {
        body: String,
        #[serde(alias = "BodyID")]
        body_id: u64,
        body_type: BodyType,
        // TODO: conflicts
        controlling_power: Option<String>,
        docked: bool,
        factions: Vec<Faction>,
        #[serde(alias = "MarketID")]
        market_id: Option<u64>,
        multicrew: bool,
        on_foot: bool,
        #[serde(flatten)]
        powerplay: Option<Powerplay>,
        star_pos: [f64; 3],
        star_system: String,
        #[serde(flatten)]
        station: Option<Station>,
        system_address: u64,
        taxi: bool,
    },
    ChangeCrewRole {
        role: CrewRole,
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
        #[serde(alias = "MarketID")]
        market_id: Option<u64>,
        stolen: bool,
        #[serde(flatten)]
        cargo: Cargo,
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
        #[serde(alias = "FID")]
        fid: String,
        name: String,
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
        telepresense: bool,
    },
    CrewMemberJoins {
        crew: String,
        telepresense: bool,
    },
    CrewMemberQuits {
        crew: String,
        telepresense: bool,
    },
    CrewMemberRoleChange {
        crew: String,
        role: CrewRole,
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
        message_display: String,
    },
    DatalinkVoucher {
        payee_faction: Allegiance,
        reward: u64,
        victim_faction: Allegiance,
    },
    Died {
        killer_name: Option<String>,
        #[serde(alias = "KillerName_Localised")]
        killer_name_display: Option<String>,
        killer_rank: Option<CombatRank>,
        killer_ship: Option<String>,
        killers: Option<Vec<Killer>>,
    },
    Disembark {
        body: String,
        #[serde(alias = "BodyID")]
        body_id: u64,
        #[serde(alias = "ID")]
        id: u64,
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
        #[serde(default)]
        dist_from_star_ls: f64,
        landing_pads: Option<LandingPads>,
        #[serde(alias = "MarketID")]
        market_id: Option<u64>,
        multicrew: bool,
        star_system: String,
        #[serde(flatten)]
        station: Option<Station>,
        system_address: u64,
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
        crew: Vec<Crew>,
        #[serde(alias = "ID")]
        id: u64,
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
        #[serde(alias = "BlueprintID")]
        blueprint_id: u64,
        blueprint_name: String,
        #[serde(flatten)]
        engineer: Engineer,
        experimental_effect: Option<String>,
        #[serde(alias = "ExperimentalEffect_Localised")]
        experimental_effect_display: Option<String>,
        ingredients: Vec<Resource>,
        level: u8,
        modifiers: Vec<Modifier>,
        module: String,
        quality: f64,
        slot: String,
    },
    EngineerProgress {
        #[serde(flatten)]
        engineer: Option<Engineer>,
        engineers: Vec<Engineer>,
    },
    EspaceInterdiction {
        inderdictor: String,
        #[serde(alias = "Interdictor_Localised")]
        inderdictor_display: Option<String>,
        is_player: bool,
    },
    #[serde(alias = "FCMaterials")]
    FcMaterials {
        carrier_id: String,
        carrier_name: String,
        #[serde(alias = "MarketID")]
        market_id: Option<u64>,
    },
    #[serde(alias = "FSDJump")]
    FsdJump {
        body: String,
        #[serde(alias = "BodyID")]
        body_id: u64,
        body_type: BodyType,
        #[serde(default)]
        boost_used: u8,
        // TODO: conflicts
        controlling_power: Option<String>,
        factions: Vec<Faction>,
        fuel_level: f64,
        jump_dist: f64,
        multicrew: bool,
        population: u64,
        #[serde(flatten)]
        powerplay: Option<Powerplay>,
        star_pos: [f64; 3],
        star_system: String,
        #[serde(flatten)]
        system: Option<System>,
        system_address: u64,
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
        fighter: bool,
        health: f64,
        player_pilot: bool,
    },
    Interdicted {
        inderdictor: String,
        #[serde(alias = "Interdictor_Localised")]
        inderdictor_display: Option<String>,
        is_player: bool,
        submitted: bool,
        combat_rank: Option<CombatRank>,
        faction: Option<Faction>,
    },
    Interdiction {
        inderdicted: Option<String>,
        is_player: bool,
        success: bool,
        combat_rank: Option<CombatRank>,
        faction: Option<Faction>,
        power: Allegiance,
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
        horizons: bool,
        loan: u64,
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
        modules: Vec<Module>,
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
        // TODO: conflicts
        controlling_power: Option<String>,
        #[serde(default)]
        dist_from_star_ls: f64,
        docked: bool,
        factions: Vec<Faction>,
        #[serde(default, alias = "InSRV")]
        in_srv: bool,
        latitude: Option<f64>,
        longitude: Option<f64>,
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
        carrier_docking_access: DockingAccess,
        #[serde(alias = "MarketID")]
        market_id: Option<u64>,
        star_system: String,
        station_name: String,
        station_type: StationType,
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
        commodity_display: String,
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
        commodity_display: String,
    },
    MassModuleStore {
        items: Vec<Module>,
        #[serde(alias = "MarketID")]
        market_id: Option<u64>,
        #[serde(flatten)]
        ship: Ship,
    },
    #[serde(other)]
    Unknown,
}
