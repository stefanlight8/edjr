//! Elite Events
//!
//! # Examples
//! ```no_run
//! use {std::{error::Error, fs::File}, edjr::{Journal, Read, event::Commander}};
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let journal = Journal::<File>::open("/Path/to/my/journals/Journal.date.log")?;
//!
//!     for entry in journal.read_all()? {
//!         match entry.event {
//!             Commander(commader) => println!("I'm commander {}!", commander.name),
//!             _ => ()
//!         }
//!     };
//!
//!     Ok(())
//! }
//! ```
#[cfg(feature = "ammo")]
pub mod ammo;
#[cfg(feature = "approach")]
pub mod approach;
#[cfg(feature = "backpack")]
pub mod backpack;
#[cfg(feature = "body")]
pub mod body;
#[cfg(feature = "bounty")]
pub mod bounty;
#[cfg(feature = "broker")]
pub mod broker;
#[cfg(feature = "cargo")]
pub mod cargo;
#[cfg(feature = "codex")]
pub mod codex;
#[cfg(feature = "colonisation")]
pub mod colonisation;
#[cfg(feature = "commander")]
pub mod commander;
#[cfg(feature = "community_goal")]
pub mod community_goal;
#[cfg(feature = "consumable")]
pub mod consumable;
#[cfg(feature = "crew")]
pub mod crew;
#[cfg(feature = "crime")]
pub mod crime;
#[cfg(feature = "data")]
pub mod data;
#[cfg(feature = "died")]
pub mod died;
#[cfg(feature = "disembark")]
pub mod disembark;
#[cfg(feature = "dock")]
pub mod dock;
#[cfg(feature = "drones")]
pub mod drones;
#[cfg(feature = "embark")]
pub mod embark;
#[cfg(feature = "engineer")]
pub mod engineer;
#[cfg(feature = "faction")]
pub mod faction;
#[cfg(feature = "fc")]
pub mod fc;
#[cfg(feature = "fighter")]
pub mod fighter;
#[cfg(feature = "fileheader")]
pub mod fileheader;
#[cfg(feature = "fines")]
pub mod fines;
#[cfg(feature = "friends")]
pub mod friends;
#[cfg(feature = "fsd")]
pub mod fsd;
#[cfg(feature = "fss")]
pub mod fss;
#[cfg(feature = "fuel")]
pub mod fuel;
#[cfg(feature = "game")]
pub mod game;
#[cfg(feature = "hull")]
pub mod hull;
#[cfg(feature = "impound")]
pub mod impound;
#[cfg(feature = "interdiction")]
pub mod interdiction;
#[cfg(feature = "jetcone")]
pub mod jetcone;
#[cfg(feature = "jump")]
pub mod jump;
#[cfg(feature = "liftoff")]
pub mod liftoff;
#[cfg(feature = "loadout")]
pub mod loadout;
#[cfg(feature = "location")]
pub mod location;
#[cfg(feature = "market")]
pub mod market;
#[cfg(feature = "material")]
pub mod material;
#[cfg(feature = "mission")]
pub mod mission;
#[cfg(feature = "module")]
pub mod module;
#[cfg(feature = "music")]
pub mod music;
#[cfg(feature = "navigation")]
pub mod navigation;
#[cfg(feature = "outfitting")]
pub mod outfitting;
#[cfg(feature = "passengers")]
pub mod passengers;
#[cfg(feature = "powerplay")]
pub mod powerplay;
#[cfg(feature = "pvp")]
pub mod pvp;
#[cfg(feature = "ranks")]
pub mod ranks;
#[cfg(feature = "repair")]
pub mod repair;
#[cfg(feature = "reputation")]
pub mod reputation;
#[cfg(feature = "resurrect")]
pub mod resurrect;
#[cfg(feature = "saa")]
pub mod saa;
#[cfg(feature = "scan")]
pub mod scan;
#[cfg(feature = "screenshot")]
pub mod screenshot;
#[cfg(feature = "search_and_rescue")]
pub mod search_and_rescue;
#[cfg(feature = "shield")]
pub mod shield;
#[cfg(feature = "ship")]
pub mod ship;
#[cfg(feature = "shipyard")]
pub mod shipyard;
#[cfg(feature = "squadron")]
pub mod squadron;
#[cfg(feature = "srv")]
pub mod srv;
#[cfg(feature = "statistics")]
pub mod statistics;
#[cfg(feature = "supercruise")]
pub mod supercruise;
#[cfg(feature = "synthesis")]
pub mod synthesis;
#[cfg(feature = "target")]
pub mod target;
#[cfg(feature = "taxi")]
pub mod taxi;
#[cfg(feature = "text")]
pub mod text;
#[cfg(feature = "touchdown")]
pub mod touchdown;
#[cfg(feature = "under_attack")]
pub mod under_attack;
#[cfg(feature = "uss")]
pub mod uss;
#[cfg(feature = "vehicle")]
pub mod vehicle;
#[cfg(feature = "voucher")]
pub mod voucher;
#[cfg(feature = "weapon")]
pub mod weapon;
#[cfg(feature = "wing")]
pub mod wing;

#[cfg(feature = "ammo")]
pub use ammo::BuyAmmoEvent;
#[cfg(feature = "approach")]
pub use approach::{ApproachBodyEvent, ApproachSettlementEvent}; // TODO: maybe move approach body to body and rename approach into settlement?
#[cfg(feature = "backpack")]
pub use backpack::{BackpackChangeEvent, BackpackEvent, CollectItemsEvent};
#[cfg(feature = "body")]
pub use body::LeaveBodyEvent;
#[cfg(feature = "bounty")]
pub use bounty::{BountyEvent, PayBountiesEvent};
#[cfg(feature = "broker")]
pub use broker::TechnologyBrokerEvent;
#[cfg(feature = "cargo")]
pub use cargo::{
    CargoDepotEvent, CargoEvent, CargoTransferEvent, CollectCargoEvent, EjectCargoEvent,
};
#[cfg(feature = "codex")]
pub use codex::CodexEntryEvent;
#[cfg(feature = "colonisation")]
pub use colonisation::ColonisationConstructionDepotEvent;
#[cfg(feature = "commander")]
pub use commander::{CommanderEvent, NewCommanderEvent};
#[cfg(feature = "community_goal")]
pub use community_goal::{
    CommunityGoalDiscardEvent, CommunityGoalEvent, CommunityGoalJoinEvent, CommunityGoalRewardEvent,
};
#[cfg(feature = "consumable")]
pub use consumable::UseConsumableEvent;
#[cfg(feature = "crew")]
pub use crew::{
    ChangeCrewRoleEvent, CrewAssignEvent, CrewLaunchFighterEvent, CrewMemberJoinsEvent,
    CrewMemberQuitsEvent, CrewMemberRoleChangeEvent, EndCrewSessionEvent, JoinACrewEvent,
    NpcCrewPaidWageEvent, QuitACrewEvent,
};
#[cfg(feature = "crime")]
pub use crime::{CommitCrimeEvent, CrimeVictimEvent};
#[cfg(feature = "data")]
pub use data::{
    DataScannedEvent, DatalinkScanEvent, DatalinkVoucherEvent, MultiSellExplorationDataEvent,
    SellfOrganicDataEvent,
};
#[cfg(feature = "died")]
pub use died::DiedEvent;
#[cfg(feature = "disembark")]
pub use disembark::DisembarkEvent;
#[cfg(feature = "dock")]
pub use dock::{
    DockedEvent, DockingCancelledEvent, DockingDeniedEvent, DockingGrantedEvent,
    DockingRequestedEvent, DockingTimeoutEvent, UndockedEvent,
};
#[cfg(feature = "drones")]
pub use drones::{BuyDronesEvent, LaunchDroneEvent, SellDronesEvent};
#[cfg(feature = "embark")]
pub use embark::EmbarkEvent;
#[cfg(feature = "engineer")]
pub use engineer::{EngineerContributionEvent, EngineerCraftEvent, EngineerProgressEvent};
#[cfg(feature = "faction")]
pub use faction::FactionKillBondEvent;
#[cfg(feature = "fc")]
pub use fc::{CarrierJumpEvent, FcMaterialsEvent};
#[cfg(feature = "fighter")]
pub use fighter::{
    DockFighterEvent, FighterDestroyedEvent, FighterRebuiltEvent, LaunchFighterEvent,
};
#[cfg(feature = "fileheader")]
pub use fileheader::FileheaderEvent;
#[cfg(feature = "fines")]
pub use fines::PayFinesEvent;
#[cfg(feature = "friends")]
pub use friends::FriendsEvent;
#[cfg(feature = "fsd")]
pub use fsd::{FsdJumpEvent, FsdTargetEvent};
#[cfg(feature = "fss")]
pub use fss::{
    FssAllBodiesFoundEvent, FssBodySignalsEvent, FssDiscoveryScanEvent, FssSignalDiscoveredEvent,
};
#[cfg(feature = "fuel")]
pub use fuel::{FuelScoopEvent, RefuelAllEvent, RefuelPartialEvent, ReservoirReplenishedEvent};
#[cfg(feature = "game")]
pub use game::LoadGameEvent;
#[cfg(feature = "hull")]
pub use hull::HullDamageEvent;
#[cfg(feature = "impound")]
pub use impound::ClearImpoundEvent;
#[cfg(feature = "interdiction")]
pub use interdiction::{EscapeInterdictionEvent, InterdictedEvent, InterdictionEvent};
#[cfg(feature = "jetcone")]
pub use jetcone::{JetConeBoostEvent, JetConeDamageEvent};
#[cfg(feature = "jump")]
pub use jump::StartJumpEvent;
#[cfg(feature = "liftoff")]
pub use liftoff::LiftoffEvent;
#[cfg(feature = "loadout")]
pub use loadout::{LoadoutEvent, SuitLoadoutEvent, SwitchSuitLoadoutEvent};
#[cfg(feature = "location")]
pub use location::LocationEvent;
#[cfg(feature = "market")]
pub use market::{MarketBuyEvent, MarketEvent, MarketSellEvent};
#[cfg(feature = "material")]
pub use material::{
    MaterialCollectedEvent, MaterialDiscoveredEvent, MaterialTradeEvent, MaterialsEvent,
};
#[cfg(feature = "mission")]
pub use mission::{
    MissionAbandonedEvent, MissionAcceptedEvent, MissionCompletedEvent, MissionFailedEvent,
    MissionRedirectedEvent, MissionsEvent,
};
#[cfg(feature = "module")]
pub use module::{
    FetchRemoteModuleEvent, MassModuleStoreEvent, ModuleBuyAndStoreEvent, ModuleBuyEvent,
    ModuleRetrieveEvent, ModuleSellEvent, ModuleSellRemoteEvent, ModuleStoreEvent, ModuleSwapEvent,
    StoredModulesEvent,
};
#[cfg(feature = "music")]
pub use music::MusicEvent;
#[cfg(feature = "navigation")]
pub use navigation::NavBeaconScanEvent;
#[cfg(feature = "outfitting")]
pub use outfitting::OutfittingEvent;
#[cfg(feature = "passengers")]
pub use passengers::PassengersEvent;
#[cfg(feature = "powerplay")]
pub use powerplay::{
    PowerplayCollectEvent, PowerplayEvent, PowerplayMeritsEvent, PowerplayRankEvent,
    RequestPowerMicroResourcesEvent,
};
#[cfg(feature = "pvp")]
pub use pvp::PvpKillEvent;
#[cfg(feature = "ranks")]
pub use ranks::{ProgressEvent, PromotionEvent, RankEvent};
#[cfg(feature = "repair")]
pub use repair::{
    AfmuRepairsEvent, RebootRepairEvent, RepairAllEvent, RepairDroneEvent, RepairEvent,
};
#[cfg(feature = "reputation")]
pub use reputation::ReputationEvent;
#[cfg(feature = "resurrect")]
pub use resurrect::{ResurrectEvent, ResurrectOption};
#[cfg(feature = "saa")]
pub use saa::{SaaScanCompleteEvent, SaaSignalsFoundEvent};
#[cfg(feature = "scan")]
pub use scan::{ScanBaryCentreEvent, ScanEvent, ScanOrganicEvent, ScannedEvent};
#[cfg(feature = "screenshot")]
pub use screenshot::ScreenshotEvent;
#[cfg(feature = "search_and_rescue")]
pub use search_and_rescue::SearchAndRescueEvent;
#[cfg(feature = "shield")]
pub use shield::ShieldStateEvent;
#[cfg(feature = "ship")]
pub use ship::{SetUserShipNameEvent, ShipLockerEvent, StoredShipsEvent};
#[cfg(feature = "shipyard")]
pub use shipyard::{
    ShipyardBuyEvent, ShipyardEvent, ShipyardNewEvent, ShipyardSellEvent, ShipyardSwapEvent,
    ShipyardTransferEvent,
};
#[cfg(feature = "squadron")]
pub use squadron::{
    InvitedToSquadronEvent, JoinedSquadronEvent, LeftSquadronEvent, SquadronCreatedEvent,
    SquadronStartupEvent,
};
#[cfg(feature = "srv")]
pub use srv::{DockSrvEvent, LaunchSrvEvent, SrvDestroyedEvent};
#[cfg(feature = "statistics")]
pub use statistics::StatisticsEvent;
#[cfg(feature = "supercruise")]
pub use supercruise::{
    SupercruiseDestinationDropEvent, SupercruiseEntryEvent, SupercruiseExitEvent,
};
#[cfg(feature = "synthesis")]
pub use synthesis::SynthesisEvent;
#[cfg(feature = "target")]
pub use target::ShipTargetedEvent;
#[cfg(feature = "taxi")]
pub use taxi::BookTaxiEvent;
#[cfg(feature = "text")]
pub use text::{ReceiveTextEvent, SendTextEvent};
#[cfg(feature = "touchdown")]
pub use touchdown::TouchdownEvent;
#[cfg(feature = "under_attack")]
pub use under_attack::UnderAttackEvent;
#[cfg(feature = "uss")]
pub use uss::UssDropEvent;
#[cfg(feature = "vehicle")]
pub use vehicle::{RestockVehicleEvent, VehicleSwitchEvent};
#[cfg(feature = "voucher")]
pub use voucher::RedeemVoucherEvent;
#[cfg(feature = "weapon")]
pub use weapon::{BuyWeaponEvent, SellWeaponEvent};
#[cfg(feature = "wing")]
pub use wing::{WingAddEvent, WingInviteEvent, WingJoinEvent};
