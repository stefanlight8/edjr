//! Journal event
use {crate::events::*, serde::Deserialize};

/// Journal Event.
///
/// Each variant represents journal event and contains its body.
#[derive(Debug, Deserialize)]
#[serde(tag = "event", rename_all_fields = "PascalCase")]
pub enum JournalEvent {
    #[cfg(feature = "repair")]
    AfmuRepairs(AfmuRepairsEvent),

    #[cfg(feature = "approach")]
    ApproachBody(ApproachBodyEvent),

    #[cfg(feature = "approach")]
    ApproachSettlement(ApproachSettlementEvent),

    #[cfg(feature = "backpack")]
    Backpack(BackpackEvent),

    #[cfg(feature = "backpack")]
    BackpackChange(BackpackChangeEvent),

    #[cfg(feature = "taxi")]
    BookTaxi(BookTaxiEvent),

    #[cfg(feature = "bounty")]
    Bounty(BountyEvent),

    #[cfg(feature = "ammo")]
    BuyAmmo(BuyAmmoEvent),

    #[cfg(feature = "drones")]
    BuyDrones(BuyDronesEvent),

    #[cfg(feature = "weapon")]
    BuyWeapon(BuyWeaponEvent),

    #[cfg(feature = "cargo")]
    Cargo(CargoEvent),

    #[cfg(feature = "cargo")]
    CargoDepot(CargoDepotEvent),

    #[cfg(feature = "cargo")]
    CargoTransfer(CargoTransferEvent),

    #[cfg(feature = "fc")]
    CarrierJump(CarrierJumpEvent),

    #[cfg(feature = "crew")]
    ChangeCrewRole(ChangeCrewRoleEvent),

    #[cfg(feature = "impound")]
    ClearImpound(ClearImpoundEvent),

    CockpitBreached,

    #[cfg(feature = "codex")]
    CodexEntry(CodexEntryEvent),

    #[cfg(feature = "cargo")]
    CollectCargo(CollectCargoEvent),

    #[cfg(feature = "backpack")]
    CollectItems(CollectItemsEvent),

    #[cfg(feature = "colonisation")]
    ColonisationConstructionDepot(ColonisationConstructionDepotEvent),

    #[cfg(feature = "commander")]
    Commander(CommanderEvent),

    #[cfg(feature = "crime")]
    CommitCrime(CommitCrimeEvent),

    #[cfg(feature = "community_goal")]
    CommunityGoal(CommunityGoalEvent),

    #[cfg(feature = "community_goal")]
    CommunityGoalDiscard(CommunityGoalDiscardEvent),

    #[cfg(feature = "community_goal")]
    CommunityGoalJoin(CommunityGoalJoinEvent),

    #[cfg(feature = "community_goal")]
    CommunityGoalReward(CommunityGoalRewardEvent),

    #[cfg(feature = "crew")]
    CrewAssign(CrewAssignEvent),

    #[cfg(feature = "crew")]
    CrewLaunchFighter(CrewLaunchFighterEvent),

    #[cfg(feature = "crew")]
    CrewMemberJoins(CrewMemberJoinsEvent),

    #[cfg(feature = "crew")]
    CrewMemberQuits(CrewMemberQuitsEvent),

    #[cfg(feature = "crew")]
    CrewMemberRoleChange(CrewMemberRoleChangeEvent),

    #[cfg(feature = "crime")]
    CrimeVictim(CrimeVictimEvent),

    #[cfg(feature = "data")]
    DataScanned(DataScannedEvent),

    #[cfg(feature = "data")]
    DatalinkScan(DatalinkScanEvent),

    #[cfg(feature = "data")]
    DatalinkVoucher(DatalinkVoucherEvent),

    #[cfg(feature = "died")]
    Died(DiedEvent),

    #[cfg(feature = "disembark")]
    Disembark(DisembarkEvent),

    #[cfg(feature = "fighter")]
    DockFighter(DockFighterEvent),

    #[cfg(feature = "srv")]
    #[serde(alias = "DockSRV")]
    DockSrv(DockSrvEvent),

    #[cfg(feature = "dock")]
    Docked(DockedEvent),

    #[cfg(feature = "dock")]
    DockingCancelled(DockingCancelledEvent),

    #[cfg(feature = "dock")]
    DockingDenied(DockingDeniedEvent),

    #[cfg(feature = "dock")]
    DockingGranted(DockingGrantedEvent),

    #[cfg(feature = "dock")]
    DockingRequested(DockingRequestedEvent),

    #[cfg(feature = "dock")]
    DockingTimeout(DockingTimeoutEvent),

    #[cfg(feature = "cargo")]
    EjectCargo(EjectCargoEvent),

    #[cfg(feature = "embark")]
    Embark(EmbarkEvent),

    #[cfg(feature = "crew")]
    EndCrewSession(EndCrewSessionEvent),

    #[cfg(feature = "engineer")]
    EngineerContribution(EngineerContributionEvent),

    #[cfg(feature = "engineer")]
    EngineerCraft(EngineerCraftEvent),

    #[cfg(feature = "engineer")]
    EngineerProgress(EngineerProgressEvent),

    #[cfg(feature = "interdiction")]
    EscapeInterdiction(EscapeInterdictionEvent),

    #[cfg(feature = "fc")]
    #[serde(alias = "FCMaterials")]
    FcMaterials(FcMaterialsEvent),

    #[cfg(feature = "fsd")]
    #[serde(alias = "FSDJump")]
    FsdJump(FsdJumpEvent),

    #[cfg(feature = "fsd")]
    #[serde(alias = "FSDTarget")]
    FsdTarget(FsdTargetEvent),

    #[cfg(feature = "fss")]
    #[serde(alias = "FSSAllBodiesFound")]
    FssAllBodiesFound(FssAllBodiesFoundEvent),

    #[cfg(feature = "fss")]
    #[serde(alias = "FSSBodySignals")]
    FssBodySignals(FssBodySignalsEvent),

    #[cfg(feature = "fss")]
    #[serde(alias = "FSSDiscoveryScan")]
    FssDiscoveryScan(FssDiscoveryScanEvent),

    #[cfg(feature = "fss")]
    #[serde(alias = "FSSSignalDiscovered")]
    FssSignalDiscovered(FssSignalDiscoveredEvent),

    #[cfg(feature = "faction")]
    FactionKillBond(FactionKillBondEvent),

    #[cfg(feature = "module")]
    FetchRemoteModule(FetchRemoteModuleEvent),

    #[cfg(feature = "fighter")]
    FighterDestroyed(FighterDestroyedEvent),

    #[cfg(feature = "fighter")]
    FighterRebuilt(FighterRebuiltEvent),

    #[cfg(feature = "fileheader")]
    Fileheader(FileheaderEvent),

    #[cfg(feature = "friends")]
    Friends(FriendsEvent),

    #[cfg(feature = "fuel")]
    FuelScoop(FuelScoopEvent),

    HeatDamage,
    HeatWarning,

    #[cfg(feature = "hull")]
    HullDamage(HullDamageEvent),

    #[cfg(feature = "interdiction")]
    Interdicted(InterdictedEvent),

    #[cfg(feature = "interdiction")]
    Interdiction(InterdictionEvent),

    #[cfg(feature = "squadron")]
    InvitedToSquadron(InvitedToSquadronEvent),

    #[cfg(feature = "jetcone")]
    JetConeBoost(JetConeBoostEvent),

    #[cfg(feature = "jetcone")]
    JetConeDamage(JetConeDamageEvent),

    #[cfg(feature = "crew")]
    JoinACrew(JoinACrewEvent),

    #[cfg(feature = "squadron")]
    JoinedSquadron(JoinedSquadronEvent),

    #[cfg(feature = "drones")]
    LaunchDrone(LaunchDroneEvent),

    #[cfg(feature = "fighter")]
    LaunchFighter(LaunchFighterEvent),

    #[cfg(feature = "srv")]
    #[serde(rename = "LaunchSRV")]
    LaunchSrv(LaunchSrvEvent),

    #[cfg(feature = "body")]
    LeaveBody(LeaveBodyEvent),

    #[cfg(feature = "squadron")]
    LeftSquadron(LeftSquadronEvent),

    #[cfg(feature = "liftoff")]
    Liftoff(LiftoffEvent),

    #[cfg(feature = "game")]
    LoadGame(LoadGameEvent),

    #[cfg(feature = "loadout")]
    Loadout(LoadoutEvent),

    #[cfg(feature = "location")]
    Location(LocationEvent),

    #[cfg(feature = "market")]
    Market(MarketEvent),

    #[cfg(feature = "market")]
    MarketBuy(MarketBuyEvent),

    #[cfg(feature = "market")]
    MarketSell(MarketSellEvent),

    #[cfg(feature = "module")]
    MassModuleStore(MassModuleStoreEvent),

    #[cfg(feature = "material")]
    MaterialCollected(MaterialCollectedEvent),

    #[cfg(feature = "material")]
    MaterialDiscovered(MaterialDiscoveredEvent),

    #[cfg(feature = "material")]
    MaterialTrade(MaterialTradeEvent),

    #[cfg(feature = "material")]
    Materials(MaterialsEvent),

    #[cfg(feature = "mission")]
    MissionAbandoned(MissionAbandonedEvent),

    #[cfg(feature = "mission")]
    MissionAccepted(MissionAcceptedEvent),

    #[cfg(feature = "mission")]
    MissionCompleted(MissionCompletedEvent),

    #[cfg(feature = "mission")]
    MissionFailed(MissionFailedEvent),

    #[cfg(feature = "mission")]
    MissionRedirected(MissionRedirectedEvent),

    #[cfg(feature = "mission")]
    Missions(MissionsEvent),

    #[cfg(feature = "module")]
    ModuleBuy(ModuleBuyEvent),

    #[cfg(feature = "module")]
    ModuleBuyAndStore(ModuleBuyAndStoreEvent),

    #[cfg(feature = "module")]
    ModuleInfo,

    #[cfg(feature = "module")]
    ModuleRetrieve(ModuleRetrieveEvent),

    #[cfg(feature = "module")]
    ModuleSell(ModuleSellEvent),

    #[cfg(feature = "module")]
    ModuleSellRemote(ModuleSellRemoteEvent),

    #[cfg(feature = "module")]
    ModuleStore(ModuleStoreEvent),

    #[cfg(feature = "module")]
    ModuleSwap(ModuleSwapEvent),

    #[cfg(feature = "data")]
    MultiSellExplorationData(MultiSellExplorationDataEvent),

    #[cfg(feature = "music")]
    Music(MusicEvent),

    #[cfg(feature = "navigation")]
    NavBeaconScan(NavBeaconScanEvent),

    #[cfg(feature = "navigation")]
    NavRoute,

    #[cfg(feature = "navigation")]
    NavRouteClear,

    #[cfg(feature = "commander")]
    NewCommander(NewCommanderEvent),

    #[cfg(feature = "crew")]
    NpcCrewPaidWage(NpcCrewPaidWageEvent),

    #[cfg(feature = "outfitting")]
    Outfitting(OutfittingEvent),

    #[cfg(feature = "pvp")]
    #[serde(alias = "PVPKill")]
    PvpKill(PvpKillEvent),

    #[cfg(feature = "passengers")]
    Passengers(PassengersEvent),

    #[cfg(feature = "bounty")]
    PayBounties(PayBountiesEvent),

    #[cfg(feature = "fines")]
    PayFines(PayFinesEvent),

    #[cfg(feature = "powerplay")]
    Powerplay(PowerplayEvent),

    #[cfg(feature = "powerplay")]
    PowerplayCollect(PowerplayCollectEvent),

    #[cfg(feature = "powerplay")]
    PowerplayMerits(PowerplayMeritsEvent),

    #[cfg(feature = "powerplay")]
    PowerplayRank(PowerplayRankEvent),

    #[cfg(feature = "ranks")]
    Progress(ProgressEvent),

    #[cfg(feature = "ranks")]
    Promotion(PromotionEvent),

    #[cfg(feature = "crew")]
    QuitACrew(QuitACrewEvent),

    #[cfg(feature = "ranks")]
    Rank(RankEvent),

    #[cfg(feature = "repair")]
    RebootRepair(RebootRepairEvent),

    #[cfg(feature = "text")]
    ReceiveText(ReceiveTextEvent),

    #[cfg(feature = "voucher")]
    RedeemVoucher(RedeemVoucherEvent),

    #[cfg(feature = "fuel")]
    RefuelAll(RefuelAllEvent),

    #[cfg(feature = "fuel")]
    RefuelPartial(RefuelPartialEvent),

    #[cfg(feature = "powerplay")]
    RequestPowerMicroResources(RequestPowerMicroResourcesEvent),

    #[cfg(feature = "repair")]
    Repair(RepairEvent),

    #[cfg(feature = "repair")]
    RepairAll(RepairAllEvent),

    #[cfg(feature = "repair")]
    RepairDrone(RepairDroneEvent),

    #[cfg(feature = "reputation")]
    Reputation(ReputationEvent),

    #[cfg(feature = "vehicle")]
    RestockVehicle(RestockVehicleEvent),

    Resupply,

    #[cfg(feature = "resurrect")]
    Resurrect(ResurrectEvent),

    #[cfg(feature = "fuel")]
    ReservoirReplenished(ReservoirReplenishedEvent),

    #[cfg(feature = "saa")]
    #[serde(alias = "SAAScanComplete")]
    SaaScanComplete(SaaScanCompleteEvent),

    #[cfg(feature = "saa")]
    #[serde(alias = "SAASignalsFound")]
    SaaSignalsFound(SaaSignalsFoundEvent),

    #[cfg(feature = "srv")]
    #[serde(alias = "SRVDestroyed")]
    SrvDestroyed(SrvDestroyedEvent),

    #[cfg(feature = "scan")]
    Scan(ScanEvent),

    #[cfg(feature = "scan")]
    ScanBaryCentre(ScanBaryCentreEvent),

    #[cfg(feature = "scan")]
    ScanOrganic(ScanOrganicEvent),

    #[cfg(feature = "scan")]
    Scanned(ScannedEvent),

    #[cfg(feature = "screenshot")]
    Screenshot(ScreenshotEvent),

    #[cfg(feature = "search_and_rescue")]
    SearchAndRescue(SearchAndRescueEvent),

    SelfDestruct,

    #[cfg(feature = "drones")]
    SellDrones(SellDronesEvent),

    #[cfg(feature = "data")]
    SellOrganicData(SellfOrganicDataEvent),

    #[cfg(feature = "weapon")]
    SellWeapon(SellWeaponEvent),

    #[cfg(feature = "text")]
    SendText(SendTextEvent),

    #[cfg(feature = "ship")]
    SetUserShipName(SetUserShipNameEvent),

    #[cfg(feature = "shield")]
    ShieldState(ShieldStateEvent),

    #[cfg(feature = "ship")]
    ShipLocker(ShipLockerEvent),

    #[cfg(feature = "target")]
    ShipTargeted(ShipTargetedEvent),

    #[cfg(feature = "shipyard")]
    Shipyard(ShipyardEvent),

    #[cfg(feature = "shipyard")]
    ShipyardBuy(ShipyardBuyEvent),

    #[cfg(feature = "shipyard")]
    ShipyardNew(ShipyardNewEvent),

    #[cfg(feature = "shipyard")]
    ShipyardSell(ShipyardSellEvent),

    #[cfg(feature = "shipyard")]
    ShipyardSwap(ShipyardSwapEvent),

    #[cfg(feature = "shipyard")]
    ShipyardTransfer(ShipyardTransferEvent),

    Shutdown,

    #[cfg(feature = "squadron")]
    SquadronCreated(SquadronCreatedEvent),

    #[cfg(feature = "squadron")]
    SquadronStartup(SquadronStartupEvent),

    #[cfg(feature = "jump")]
    StartJump(StartJumpEvent),

    #[cfg(feature = "statistics")]
    Statistics(StatisticsEvent),

    #[cfg(feature = "module")]
    StoredModules(StoredModulesEvent),

    #[cfg(feature = "ship")]
    StoredShips(StoredShipsEvent),

    #[cfg(feature = "loadout")]
    SuitLoadout(SuitLoadoutEvent),

    #[cfg(feature = "supercruise")]
    SupercruiseDestinationDrop(SupercruiseDestinationDropEvent),

    #[cfg(feature = "supercruise")]
    SupercruiseEntry(SupercruiseEntryEvent),

    #[cfg(feature = "supercruise")]
    SupercruiseExit(SupercruiseExitEvent),

    #[cfg(feature = "loadout")]
    SwitchSuitLoadout(SwitchSuitLoadoutEvent),

    #[cfg(feature = "synthesis")]
    Synthesis(SynthesisEvent),

    SystemsShutdown,

    #[cfg(feature = "broker")]
    TechnologyBroker(TechnologyBrokerEvent),

    #[cfg(feature = "touchdown")]
    Touchdown(TouchdownEvent),

    #[cfg(feature = "uss")]
    #[serde(alias = "USSDrop")]
    UssDrop(UssDropEvent),

    #[cfg(feature = "under_attack")]
    UnderAttack(UnderAttackEvent),

    #[cfg(feature = "dock")]
    Undocked(UndockedEvent),

    #[cfg(feature = "consumable")]
    UseConsumable(UseConsumableEvent),

    #[cfg(feature = "vehicle")]
    VehicleSwitch(VehicleSwitchEvent),

    #[cfg(feature = "wing")]
    WingAdd(WingAddEvent),

    #[cfg(feature = "wing")]
    WingInvite(WingInviteEvent),

    #[cfg(feature = "wing")]
    WingJoin(WingJoinEvent),

    #[cfg(feature = "wing")]
    WingLeave,

    #[serde(other)]
    Unknown,
}
