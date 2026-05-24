use {
    chrono::{DateTime, Utc},
    clap::{Arg, Command},
    edjr::{Journal, JournalEvent, elite::ship::Ship, events::DockedEvent},
    futures_lite::StreamExt,
    std::{
        collections::{HashMap, HashSet},
        error::Error,
        fmt,
        path::PathBuf,
    },
    tokio::fs::File,
};

const WIDTH: usize = 18;

#[derive(Default)]
struct Session {
    started: Option<DateTime<Utc>>,
    duration: f64,
    jumps: u64,
    ships: HashMap<u64, Ship>,
    visited_systems: HashSet<String>,
    visited_stations: HashSet<String>,
}

impl fmt::Display for Session {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Session Statistics")?;
        writeln!(f, "{}", "─".repeat(WIDTH))?;
        writeln!(f)?;
        writeln!(
            f,
            "{:<WIDTH$} {}",
            "Started",
            self.started
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_else(|| "N/A".to_string()),
        )?;
        writeln!(f, "{:<WIDTH$} {:.0}s", "Duration", self.duration,)?;
        writeln!(f, "{:<WIDTH$} {}", "Jumps", self.jumps)?;
        writeln!(f)?;

        writeln!(f, "Used Ships:")?;
        if self.ships.is_empty() {
            writeln!(f, "- None")?;
        } else {
            for ship in self.ships.values() {
                writeln!(
                    f,
                    "- {}: {} ({})",
                    ship.ship,
                    ship.ship_name.clone().as_deref().unwrap_or(""),
                    ship.ship_ident.clone().as_deref().unwrap_or(""),
                )?;
            }
        }

        writeln!(f, "Visited Stations:")?;
        if self.visited_stations.is_empty() {
            writeln!(f, "- None")?;
        } else {
            for station in &self.visited_stations {
                writeln!(f, "- {}", station,)?;
            }
        }

        writeln!(f, "Visited Systems:")?;
        if self.visited_systems.is_empty() {
            writeln!(f, "- None")?;
        } else {
            for system in &self.visited_systems {
                writeln!(f, "- {}", system,)?;
            }
        }

        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
enum StatisticsError {
    #[error("provided path is invalid journal")]
    InvalidJournal,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let command = Command::new("statistics")
        .about("Get some statistics about provided journal")
        .arg(
            Arg::new("path")
                .value_name("PATH")
                .required(true)
                .value_parser(clap::value_parser!(PathBuf)),
        );
    let arguments = command.get_matches();

    let journal_path: &PathBuf = arguments.get_one("path").unwrap();
    let journal = Journal::<File>::open(journal_path).await?;
    let mut stream = journal.stream().boxed();

    let mut session = Session::default();
    let mut last_event_ts: Option<DateTime<Utc>> = None;

    if let Some(Ok(entry)) = stream.next().await {
        last_event_ts = Some(entry.timestamp);

        match entry.event {
            JournalEvent::Fileheader(_) => session.started = Some(entry.timestamp),
            _ => (),
        }
    } else {
        return Err(StatisticsError::InvalidJournal.into());
    }

    while let Some(Ok(entry)) = stream.next().await {
        if let Some(last_ts) = last_event_ts {
            let delta = entry.timestamp - last_ts;

            session.duration += delta.as_seconds_f64();
        }
        last_event_ts = Some(entry.timestamp);

        match entry.event {
            JournalEvent::Undocked(event) => {
                session.visited_stations.insert(event.station_name);
            }
            JournalEvent::Docked(DockedEvent {
                station: Some(station),
                ..
            }) => {
                println!("{:?}", station);
                session.visited_stations.insert(station.station_name);
            }
            JournalEvent::Loadout(event) => {
                session
                    .ships
                    .entry(event.ship.ship_id)
                    .or_insert(event.ship);
            }
            JournalEvent::Location(event) => {
                session.visited_systems.insert(event.star_system);
            }
            _ => (),
        }
    }

    println!("{}", session);

    Ok(())
}
