# 0.2.0
- Remove traits and moved their methods to journals implementation.

  **Now library doesn't require to import `Read`, `AsyncRead` and etc., to work with journals.**
  That was somewhat abstraction to separate sync and async layers, but `Journal<T>` where T is already a marker of sync/async implementations, it's become useless and ugly for library's API.
- Add `Serialize` to all deserializable objects.
- Add `CrewFireEvent`, `CarrierDepositFuelEvent`, `CarrierJumpRequestEvent`, `CarrierLocationEvent`, `CarrierStatsEvent`.
  - Add `TaxRate`, `CarrierType`, `CarrierFinance`, `CarrierSpaceUsage`, `CarrierCrewRole`, `CarrierCrew`.
- Move all statistics structs to `elite::statistics` module.
- Add Merc Coins fields to `BankAccountStatistics` and add `MainGame` and `Operation` to `GameMode`.
- Add `modifier` to `Module` and `quality`, `modifiers` to `ModuleEngineering`.
- Rename `srv` flags to `in_srv`.
- Add `JumpImminent` to docking denied `Reason`.
- Add `poll` to `Journal<tokio::fs::File>`.

# 0.1.4
- **Add features flags for enabling events and some elite objects.**

  Now edjr allows to select which events you need to be compiled or you can use feature `full` to get all of supported.
  And some of features like `passengers`, `powerplay`, `faction` introducing their fields in other events, like `faction` adds `factions` and `conlicts` fields to the [`LocationEvent`](./src/events/location.rs).
  I recommend to use `full` in development and determine which events and objects you use after.
- Rename `elite::passenger` to `elite::passengers`, `elite::fleet_carriers` to `elite::fc`.
- Add `station_name` to [`Station`](./src/elite/station.rs).

# 0.1.3
- Fix documentation.

# 0.1.2
- Remove `events::exploration`, `events::heat` modules.
- Make all events and objects fields public
- Add `BrokerType`, `CodexEntryCategory`, `Genus`, `SrvType` objects.
- Add `RequestPowerMicroResources`, `ReservoirReplenished`, `RestockVehicle`, `Resupply`, `Resurrect`, `SAAScanComplete`, `SAASignalsFound`, `SRVDestroyed`, `Scan`, `ScanBaryCentre`, `ScanOrganic`, `Scanned`, `Screenshot`, `SearchAndRescue`, `SellDrones`, `SellOrganicData`, `SellWeapon`, `SendText`, `SetUserShipName`, `ShieldState`, `ShipTargeted`, `Shipyard`, `ShipyardBuy`, `ShipyardNew`, `ShipyardSell`, `ShipyardSwap`, `ShipyardTransfer`, `Statistics`, `StoredModules`, `StoredShips`, `SuitLoadout`, `SupercruiseDestinationDrop`, `SupercruiseEntry`, `SupercruiseExit`, `SwtichSuitLoadout`, `Synthesis`, `SystemsShutdown`, `TechnologyBroker`, `Touchdown`, `VehicleSwitch` events.
- Add documentation.

# 0.1.1
- `stream()`: made it synchronous and just returning stream without result.
  Important: now it takes self and journal reader beings moved inside of stream.
- Add events `Shutdown`, `Touchdown`, `SupercruiseDestinationDrop`, `SupercruiseExit`, `SupercruiseEntry`, `SquadronCreated`, `SupercruiseStartup`, `StartJump`.
- Fix some events by aliasing their names.
