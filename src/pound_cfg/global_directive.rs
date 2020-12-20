// https://linux.die.net/man/8/pound
struct GlobalDirective {
    user: Optional<String>,
    group: Optional<String>,
    root_jail: Optional<PathBuf>,
    daemon: Bool, // default: true
    threads: u32, // default: 128
    log_facility: LogFacility, // default: LogFacility.Daemon
}

// https://knowledge.sakura.ad.jp/8969/
enum LogFacility {
    Kern,
    User,
    Mail,
    Daemon,
    Auth,
    Syslog,
    Lpr,
    News,
    Uucp,
    Cron,
    Authpriv,
    Ftp,
    Local0,
    Local1,
    Local2,
    Local3,
    Local4,
    Local5,
    Local6,
    Local7,
    Hyphen,
}
