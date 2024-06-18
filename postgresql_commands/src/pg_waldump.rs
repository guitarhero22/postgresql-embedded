use crate::traits::CommandBuilder;
use crate::Settings;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `pg_waldump` decodes and displays `PostgreSQL` write-ahead logs for debugging.
#[derive(Clone, Debug, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct PgWalDumpBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    backkup_details: bool,
    block: Option<OsString>,
    end: Option<OsString>,
    follow: bool,
    fork: Option<OsString>,
    limit: Option<OsString>,
    path: Option<OsString>,
    quiet: bool,
    rmgr: Option<OsString>,
    relation: Option<OsString>,
    start: Option<OsString>,
    timeline: Option<OsString>,
    version: bool,
    fullpage: bool,
    xid: Option<OsString>,
    stats: Option<OsString>,
    save_fullpage: Option<OsString>,
    help: bool,
}

impl PgWalDumpBuilder {
    /// Create a new [`PgWalDumpBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`PgWalDumpBuilder`] from [Settings]
    pub fn from(settings: &dyn Settings) -> Self {
        Self::new().program_dir(settings.get_binary_dir())
    }

    /// Location of the program binary
    #[must_use]
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// output detailed information about backup blocks
    #[must_use]
    pub fn backup_details(mut self) -> Self {
        self.backkup_details = true;
        self
    }

    /// with --relation, only show records that modify block N
    #[must_use]
    pub fn block<S: AsRef<OsStr>>(mut self, block: S) -> Self {
        self.block = Some(block.as_ref().to_os_string());
        self
    }

    /// stop reading at WAL location RECPTR
    #[must_use]
    pub fn end<S: AsRef<OsStr>>(mut self, end: S) -> Self {
        self.end = Some(end.as_ref().to_os_string());
        self
    }

    /// keep retrying after reaching end of WAL
    #[must_use]
    pub fn follow(mut self) -> Self {
        self.follow = true;
        self
    }

    /// only show records that modify blocks in fork FORK
    #[must_use]
    pub fn fork<S: AsRef<OsStr>>(mut self, fork: S) -> Self {
        self.fork = Some(fork.as_ref().to_os_string());
        self
    }

    /// number of records to display
    #[must_use]
    pub fn limit<S: AsRef<OsStr>>(mut self, limit: S) -> Self {
        self.limit = Some(limit.as_ref().to_os_string());
        self
    }

    /// directory in which to find WAL segment files
    #[must_use]
    pub fn path<S: AsRef<OsStr>>(mut self, path: S) -> Self {
        self.path = Some(path.as_ref().to_os_string());
        self
    }

    /// do not print any output, except for errors
    #[must_use]
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    /// only show records generated by resource manager RMGR
    #[must_use]
    pub fn rmgr<S: AsRef<OsStr>>(mut self, rmgr: S) -> Self {
        self.rmgr = Some(rmgr.as_ref().to_os_string());
        self
    }

    /// only show records that modify blocks in relation T/D/R
    #[must_use]
    pub fn relation<S: AsRef<OsStr>>(mut self, relation: S) -> Self {
        self.relation = Some(relation.as_ref().to_os_string());
        self
    }

    /// start reading at WAL location RECPTR
    #[must_use]
    pub fn start<S: AsRef<OsStr>>(mut self, start: S) -> Self {
        self.start = Some(start.as_ref().to_os_string());
        self
    }

    /// timeline from which to read WAL records
    #[must_use]
    pub fn timeline<S: AsRef<OsStr>>(mut self, timeline: S) -> Self {
        self.timeline = Some(timeline.as_ref().to_os_string());
        self
    }

    /// output version information, then exit
    #[must_use]
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// only show records with a full page write
    #[must_use]
    pub fn fullpage(mut self) -> Self {
        self.fullpage = true;
        self
    }

    /// only show records with transaction ID XID
    #[must_use]
    pub fn xid<S: AsRef<OsStr>>(mut self, xid: S) -> Self {
        self.xid = Some(xid.as_ref().to_os_string());
        self
    }

    /// show statistics instead of records
    #[must_use]
    pub fn stats<S: AsRef<OsStr>>(mut self, stats: S) -> Self {
        self.stats = Some(stats.as_ref().to_os_string());
        self
    }

    /// save full page images to DIR
    #[must_use]
    pub fn save_fullpage<S: AsRef<OsStr>>(mut self, save_fullpage: S) -> Self {
        self.save_fullpage = Some(save_fullpage.as_ref().to_os_string());
        self
    }

    /// show help, then exit
    #[must_use]
    pub fn help(mut self) -> Self {
        self.help = true;
        self
    }
}

impl CommandBuilder for PgWalDumpBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "pg_waldump".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if self.backkup_details {
            args.push("--bkp-details".into());
        }

        if let Some(block) = &self.block {
            args.push("--block".into());
            args.push(block.into());
        }

        if let Some(end) = &self.end {
            args.push("--end".into());
            args.push(end.into());
        }

        if self.follow {
            args.push("--follow".into());
        }

        if let Some(fork) = &self.fork {
            args.push("--fork".into());
            args.push(fork.into());
        }

        if let Some(limit) = &self.limit {
            args.push("--limit".into());
            args.push(limit.into());
        }

        if let Some(path) = &self.path {
            args.push("--path".into());
            args.push(path.into());
        }

        if self.quiet {
            args.push("--quiet".into());
        }

        if let Some(rmgr) = &self.rmgr {
            args.push("--rmgr".into());
            args.push(rmgr.into());
        }

        if let Some(relation) = &self.relation {
            args.push("--relation".into());
            args.push(relation.into());
        }

        if let Some(start) = &self.start {
            args.push("--start".into());
            args.push(start.into());
        }

        if let Some(timeline) = &self.timeline {
            args.push("--timeline".into());
            args.push(timeline.into());
        }

        if self.version {
            args.push("--version".into());
        }

        if self.fullpage {
            args.push("--fullpage".into());
        }

        if let Some(xid) = &self.xid {
            args.push("--xid".into());
            args.push(xid.into());
        }

        if let Some(stats) = &self.stats {
            args.push("--stats".into());
            args.push(stats.into());
        }

        if let Some(save_fullpage) = &self.save_fullpage {
            args.push("--save-fullpage".into());
            args.push(save_fullpage.into());
        }

        if self.help {
            args.push("--help".into());
        }

        args
    }

    /// Get the environment variables for the command
    fn get_envs(&self) -> Vec<(OsString, OsString)> {
        self.envs.clone()
    }

    /// Set an environment variable for the command
    fn env<S: AsRef<OsStr>>(mut self, key: S, value: S) -> Self {
        self.envs
            .push((key.as_ref().to_os_string(), value.as_ref().to_os_string()));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::CommandToString;
    use crate::TestSettings;
    use test_log::test;

    #[test]
    fn test_builder_new() {
        let command = PgWalDumpBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("pg_waldump"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = PgWalDumpBuilder::from(&TestSettings).build();
        assert_eq!(r#""./pg_waldump""#, command.to_command_string());
    }

    #[test]
    fn test_builder() {
        let command = PgWalDumpBuilder::new()
            .env("PGDATABASE", "database")
            .backup_details()
            .block("block")
            .end("end")
            .follow()
            .fork("fork")
            .limit("limit")
            .path("path")
            .quiet()
            .rmgr("rmgr")
            .relation("relation")
            .start("start")
            .timeline("timeline")
            .version()
            .fullpage()
            .xid("xid")
            .stats("stats")
            .save_fullpage("save_fullpage")
            .help()
            .build();

        assert_eq!(
            r#"PGDATABASE="database" "pg_waldump" "--bkp-details" "--block" "block" "--end" "end" "--follow" "--fork" "fork" "--limit" "limit" "--path" "path" "--quiet" "--rmgr" "rmgr" "--relation" "relation" "--start" "start" "--timeline" "timeline" "--version" "--fullpage" "--xid" "xid" "--stats" "stats" "--save-fullpage" "save_fullpage" "--help""#,
            command.to_command_string()
        );
    }
}
