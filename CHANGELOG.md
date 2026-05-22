# 0.1.1
- `stream()`: made it synchronous and just returning stream without result.
  Important: now it takes self and journal reader beings moved inside of stream.
- Add events `Shutdown`, `Touchdown`, `SupercruiseDestinationDrop`, `SupercruiseExit`, `SupercruiseEntry`, `SquadronCreated`, `SupercruiseStartup`, `StartJump`.
- Fix some events by aliasing their names.
