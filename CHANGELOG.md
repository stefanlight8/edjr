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
