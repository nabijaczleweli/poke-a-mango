use chrono::{DateTime, FixedOffset, ParseError, Local};
use self::super::{read_toml_file, write_toml_file};
use self::super::super::Error;
use std::iter::FromIterator;
use std::cmp::Ordering;
use std::path::Path;


/// A leaderboard entry.
///
/// # Examples
///
/// Reading a leaderboard, adding an entry to it, then writing it back.
///
/// ```
/// # use std::fs::{File, create_dir_all};
/// # use poke_a_mango::ops::Leader;
/// # use std::env::temp_dir;
/// let tf = temp_dir().join("poke-a-mango-doctest").join("ops-Leader-0");
/// create_dir_all(&tf).unwrap();
///
/// let tf = tf.join("leaderboard.toml");
/// File::create(&tf).unwrap();
///
/// let mut leaders = Leader::read(&tf).unwrap();
/// assert!(leaders.is_empty());
/// leaders.push(Leader::now("nabijaczleweli".to_string(), 105));
/// assert_eq!(Leader::write(leaders, &tf), Ok(()));
/// ```
#[derive(Debug, Clone, Hash, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Leader {
    /// Name the user entered when prompted.
    pub name: String,
    /// Time the score was achieved.
    pub time: DateTime<FixedOffset>,
    /// User's score.
    pub score: u64,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, RustcEncodable, RustcDecodable)]
struct LeaderForSerialisation {
    name: String,
    time: String,
    score: u64,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, RustcEncodable, RustcDecodable)]
struct Leaders {
    leader: Vec<LeaderForSerialisation>,
}


impl Leader {
    /// Create a new leaderboard entry with `time` set to now.
    ///
    /// # Examples
    ///
    /// ```
    /// # use poke_a_mango::ops::Leader;
    /// let ldr = Leader::now("nabijaczleweli".to_string(), 36);
    /// assert_eq!(ldr.name, "nabijaczleweli");
    /// assert_eq!(ldr.score, 36);
    /// ```
    pub fn now(name: String, score: u64) -> Leader {
        let now = Local::now();
        Leader {
            name: name,
            time: now.with_timezone(now.offset()),
            score: score,
        }
    }

    /// Read leaderboard from the specified file.
    ///
    /// If the specified file doesn't exist an empty leaderboard is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::fs::{File, create_dir_all};
    /// # use poke_a_mango::ops::Leader;
    /// # use std::env::temp_dir;
    /// let tf = temp_dir().join("poke-a-mango-doctest").join("ops-Leader-read-0");
    /// create_dir_all(&tf).unwrap();
    ///
    /// let tf = tf.join("leaderboard.toml");
    /// File::create(&tf).unwrap();
    ///
    /// assert_eq!(Leader::read(&tf), Ok(vec![]));
    /// ```
    pub fn read(p: &Path) -> Result<Vec<Leader>, Error> {
        if p.exists() {
            let queued_leaders: Leaders = try!(read_toml_file(p, "leaderboard"));
            Ok(Result::from_iter(queued_leaders.leader.into_iter().map(|qts| qts.into()).collect::<Vec<_>>()).unwrap())
        } else {
            Ok(vec![])
        }
    }

    /// Save leaderboard to the specified file.
    ///
    /// # Examples
    ///
    /// Reading a leaderboard, adding an entry to it, then writing it back.
    ///
    /// ```
    /// # extern crate poke_a_mango;
    /// # extern crate chrono;
    /// # use std::fs::{File, create_dir_all};
    /// # use self::chrono::{Duration, Local};
    /// # use self::poke_a_mango::ops::Leader;
    /// # use std::env::temp_dir;
    /// # fn main() {
    /// let tf = temp_dir().join("poke-a-mango-doctest").join("ops-Leader-write-0");
    /// create_dir_all(&tf).unwrap();
    ///
    /// Leader::write(vec![Leader {
    ///                        name: "nabijaczleweli".to_string(),
    ///                        time: {
    ///                            let now = Local::now();
    ///                            now.with_timezone(now.offset())
    ///                        },
    ///                        score: 105,
    ///                    },
    ///                    Leader {
    ///                        name: "skorezore".to_string(),
    ///                        time: {
    ///                            let now = Local::now();
    ///                            now.with_timezone(now.offset()) - Duration::minutes(30)
    ///                        },
    ///                        score: 51,
    ///                    }],
    ///               &tf.join("leaderboard.toml"))
    ///     .unwrap();
    /// # }
    /// ```
    pub fn write(queued_leaders: Vec<Leader>, p: &Path) -> Result<(), Error> {
        write_toml_file(&Leaders { leader: queued_leaders.into_iter().map(LeaderForSerialisation::from).collect() },
                        p,
                        "leaderboard")
    }
}

impl Ord for Leader {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Leader {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}


impl From<Leader> for LeaderForSerialisation {
    fn from(qt: Leader) -> LeaderForSerialisation {
        LeaderForSerialisation {
            name: qt.name,
            time: qt.time.to_rfc3339(),
            score: qt.score,
        }
    }
}

impl Into<Result<Leader, ParseError>> for LeaderForSerialisation {
    fn into(self) -> Result<Leader, ParseError> {
        Ok(Leader {
            name: self.name,
            time: try!(DateTime::parse_from_rfc3339(&self.time)),
            score: self.score,
        })
    }
}
