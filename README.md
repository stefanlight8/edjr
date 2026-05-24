# edjr
[![crates.io](https://img.shields.io/crates/v/edjr)](https://crates.io/crates/edjr)
[![docs.rs](https://docs.rs/edjr/badge.svg)](https://docs.rs/edjr)

Elite Dangerous Journal Reader

A library for parsing Elite Dangerous's journal files.
edjr supports only Elite Dangerous Odyssey.

## Features
- `full`: enables all journal events. 
- `tokio`: provide [tokio]-based journal reader.
- `stream`: provides stream for journal reader.
- `faction`: enables faction-specific events and fields.
- `powerplay`: enables powerplay-specific events and fields.
- `fc`: enables fleet carrier-specific events and fields.
- `passengers`: enables passengers-specific events and fields.

## Examples
### Read all (sync)
```rs
use {
    edjr::{journal::Journal, read::Read},
    std::{error::Error, fs::File, path::PathBuf},
};

fn main() -> Result<(), Box<dyn Error>> {
    let journal_path =
        PathBuf::from("/Path/to/Journals/Journal.date.log");
    let mut journal = Journal::<File>::open(journal_path)?;
    let entries = journal.read_all()?;

    Ok(())
}

```
### Read all (async, features = [tokio])
```rs
use {
    edjr::{journal::Journal, async_read::AsyncRead},
    std::{error::Error, path::PathBuf},
    tokio::fs::File,
};

#[tokio::main]
fn main() -> Result<(), Box<dyn Error>> {
    let journal_path =
        PathBuf::from("/Path/to/Journals/Journal.date.log");
    let mut journal = Journal::<File>::open(journal_path).await?;
    let entries = journal.read_all().await?;

    Ok(())
}

```
View more examples in [`examples/`](./examples)

## Supported events
<details>
<summary>Show</summary>

- [x] AfmuRepairs
- [x] ApproachBody
- [x] ApproachSettlement
- [x] Backpack
- [x] BackpackChange
- [x] BookTaxi
- [x] Bounty
- [x] BuyAmmo
- [x] BuyDrones
- [x] BuyWeapon
- [x] Cargo
- [x] CargoDepot
- [x] CargoTransfer
- [x] CarrierJump
- [x] ChangeCrewRole
- [x] ClearImpound
- [x] CockpitBreached
- [x] CodexEntry
- [x] CollectCargo
- [x] CollectItems
- [x] ColonisationConstructionDepot
- [x] Commander
- [x] CommitCrime
- [x] CommunityGoal
- [x] CommunityGoalDiscard
- [x] CommunityGoalJoin
- [x] CommunityGoalReward
- [x] CrewAssign
- [x] CrewLaunchFighter
- [x] CrewMemberJoins
- [x] CrewMemberQuits
- [x] CrewMemberRoleChange
- [x] CrimeVictim
- [x] DataScanned
- [x] DatalinkScan
- [x] DatalinkVoucher
- [x] Died
- [x] Disembark
- [x] DockFighter
- [x] DockSRV
- [x] Docked
- [x] DockingCancelled
- [x] DockingDenied
- [x] DockingGranted
- [x] DockingRequested
- [x] DockingTimeout
- [x] EjectCargo
- [x] Embark
- [x] EndCrewSession
- [x] EngineerContribution
- [x] EngineerCraft
- [x] EngineerProgress
- [x] EscapeInterdiction
- [x] FCMaterials
- [x] FSDJump
- [x] FSDTarget
- [x] FSSAllBodiesFound
- [x] FSSBodySignals
- [x] FSSDiscoveryScan
- [x] FSSSignalDiscovered
- [x] FactionKillBond
- [x] FetchRemoteModule
- [x] FighterDestroyed
- [x] FighterRebuilt
- [x] Fileheader
- [x] Friends
- [x] FuelScoop
- [x] HeatDamage
- [x] HeatWarning
- [x] HullDamage
- [x] Interdicted
- [x] Interdiction
- [x] InvitedToSquadron
- [x] JetConeBoost
- [x] JetConeDamage
- [x] JoinACrew
- [x] JoinedSquadron
- [x] LaunchDrone
- [x] LaunchFighter
- [x] LaunchSRV
- [x] LeaveBody
- [x] LeftSquadron
- [x] Liftoff
- [x] LoadGame
- [x] Loadout
- [x] Location
- [x] Market
- [x] MarketBuy
- [x] MarketSell
- [x] MassModuleStore
- [x] MaterialCollected
- [x] MaterialDiscovered
- [x] MaterialTrade
- [x] Materials
- [x] MissionAbandoned
- [x] MissionAccepted
- [x] MissionCompleted
- [x] MissionFailed
- [x] MissionRedirected
- [x] Missions
- [x] ModuleBuy
- [x] ModuleBuyAndStore
- [x] ModuleInfo
- [x] ModuleRetrieve
- [x] ModuleSell
- [x] ModuleSellRemote
- [x] ModuleStore
- [x] ModuleSwap
- [x] MultiSellExplorationData
- [x] Music
- [x] NavBeaconScan
- [x] NavRoute
- [x] NavRouteClear
- [x] NewCommander
- [x] NpcCrewPaidWage
- [x] Outfitting
- [x] PVPKill
- [x] Passengers
- [x] PayBounties
- [x] PayFines
- [x] Powerplay
- [x] PowerplayCollect
- [x] PowerplayMerits
- [x] PowerplayRank
- [x] Progress
- [x] Promotion
- [x] QuitACrew
- [x] Rank
- [x] RebootRepair
- [x] ReceiveText
- [x] RedeemVoucher
- [x] RefuelAll
- [x] RefuelPartial
- [x] Repair
- [x] RepairAll
- [x] RepairDrone
- [x] Reputation
- [x] RequestPowerMicroResources
- [x] ReservoirReplenished
- [x] RestockVehicle
- [x] Resupply
- [x] Resurrect
- [x] SAAScanComplete
- [x] SAASignalsFound
- [x] SRVDestroyed
- [x] Scan
- [x] ScanBaryCentre
- [x] ScanOrganic
- [x] Scanned
- [x] Screenshot
- [x] SearchAndRescue
- [x] SelfDestruct
- [x] SellDrones
- [x] SellOrganicData
- [x] SellWeapon
- [x] SendText
- [x] SetUserShipName
- [x] ShieldState
- [x] ShipLocker
- [x] ShipTargeted
- [x] Shipyard
- [x] ShipyardBuy
- [x] ShipyardNew
- [x] ShipyardSell
- [x] ShipyardSwap
- [x] ShipyardTransfer
- [x] Shutdown
- [x] SquadronCreated
- [x] SquadronStartup
- [x] StartJump
- [x] Statistics
- [x] StoredModules
- [x] StoredShips
- [x] SuitLoadout
- [x] SupercruiseDestinationDrop
- [x] SupercruiseEntry
- [x] SupercruiseExit
- [x] SwitchSuitLoadout
- [x] Synthesis
- [x] SystemsShutdown
- [x] TechnologyBroker
- [x] Touchdown
- [x] USSDrop
- [x] UnderAttack
- [x] Undocked
- [x] UseConsumable
- [x] VehicleSwitch
- [x] WingAdd
- [x] WingInvite
- [x] WingJoin
- [x] WingLeave

</details>

## Installation
```sh
$ cargo add edjr
// or via GitHub repository
$ cargo add edjr --git https://github.com/stefanlight8/edjr
```

[tokio]: https://tokio.rs/
