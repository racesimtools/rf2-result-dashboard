/*
result {
    Settings
    ServerName
    ClientFuelFisible
    PlayerFile
    DateTime
    TimeString
    Mod
    Season
    TrackVenue
    TrackCourse
    TrackEvent
    TrackData
    TrackLength
    GameVersion
    Dedicated
    ConnectionType {
        upload
        download
        body
    }
    RaceLaps
    RaceTime
    MechFailRate
    DamageMult
    FuelMult
    TireMult
    VehiclesAllowed
    ParcFerme
    FixedSetups
    FreeSettings
    FixedUpgrades
    Practice1|Practice2|Practice3|Qualify|Warmup|Race {
        DateTime
        TimeString
        Laps
        Minutes
        Stream {

        }
        MostLapsCompleted
        ..Driver {
            Name
            Connected
            VehFile
            UpgradeCode
            VehName
            Category
            CarType
            CarClass
            CarNumber
            TeamName
            isPlayer
            ServerScored
            Position
            ClassPosition
            Points
            ClassPoints
            LapRankIncludingDiscos
            ..Lap {
                num
                p
                et
                s1?
                s2?
                s3?
                fuel
                twfl
                twfr
                twrl
                twrr
                fcompund
                rcompound
                body
            }
            BestLapTime
            Laps
            Pitstops
            FinishStatus
            ..ControlAndAids {
                startLap
                endLap
                body
            }
        }
    }
}
*/

struct RaceResult {
    TimeString: String,
    Mod: String,
    TrackVenue: String,
    TrackCourse: String,
    TrackEvent: String,
    TrackLength: f64,
    GameVersion: String,
    RaceLaps: u32,
    RaceTime: u32,
    MechFailRate: u8,
    DamageMult: u8,
    FuelMult: u8,
    TireMult: u8,
    VehiclesAllowed: String,
    ParcFerme: u8,
    FixedSetups: bool,
    FreeSettings: bool,
    FixedUpgrades: bool,
    Practice1: Option<SessionResult>,
    Practice2: Option<SessionResult>,
    Practice3: Option<SessionResult>,
    Qualify: Option<SessionResult>,
    Warmup: Option<SessionResult>,
    Race: Option<SessionResult>,
}

struct SessionResult {
    TimeString: String,
    Laps: u32,
    Minutes: u32,
    MostLapsCompleted: u32,
    Drivers: []DriverResult
}

struct DriverResult {
   Laps: []LapResult,
   Name: String,
   VehName: String,
   CarNumber: String,
   TeamName: String,
   Position: u32,
   ClassPosition: u32,
   LapRankIncludingDiscos: u32,
   BestLapTime: f64,
   Laps: u32,
   Pitstops: u32,
}

struct LapResult {
    num: u32,
    p: u32,
    et: f64,
    s1: f64,
    s2: f64,
    s3: f64,
    fuel: f64,
    twfl: f64,
    twfr: f64,
    twrl: f64,
    twrr: f64,
    fcompound: String,
    rcompound: String,
    time: Option<f64>
}