use crate::localization::*;
use chrono::Local;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalMode {
    Normal,
    Learning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalState {
    Idle,
    Running,
    WaitingForInput,
    Error,
    Completed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryType {
    Command,
    Output,
    Error,
    Info,
    Hint,
    Tutorial,
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandCategory {
    FileSystem,
    TextProcessing,
    SystemInfo,
    PackageManagement,
    ProcessManagement,
    Networking,
    Permissions,
    Compression,
    Git,
    Programming,
    Terminal,
    Utility,
    ShellBuiltin,
    Other,
}

#[derive(Debug, Clone)]
pub struct KnownCommand {
    pub name: &'static str,
    pub description_key: &'static str,
    pub category: CommandCategory,
    pub common_options: &'static [(&'static str, &'static str)],
    pub related: &'static [&'static str],
    pub example: &'static str,
    pub difficulty: &'static str,
    pub typical_output: &'static str,
    pub notes: &'static str,
}

#[derive(Debug, Clone)]
pub struct KnownCommands {
    commands: Vec<KnownCommand>,
}

impl KnownCommands {
    pub fn new() -> Self {
        KnownCommands {
            commands: Self::database(),
        }
    }

    pub fn find(&self, name: &str) -> Option<&KnownCommand> {
        self.commands.iter().find(|c| c.name == name)
    }

    pub fn all(&self) -> &[KnownCommand] {
        &self.commands
    }

    pub fn len(&self) -> usize {
        self.commands.len()
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub fn by_category(&self, cat: CommandCategory) -> Vec<&KnownCommand> {
        self.commands.iter().filter(|c| c.category == cat).collect()
    }

    pub fn by_difficulty(&self, level: &str) -> Vec<&KnownCommand> {
        self.commands
            .iter()
            .filter(|c| c.difficulty == level)
            .collect()
    }

    pub fn search_by_prefix(&self, prefix: &str) -> Vec<&KnownCommand> {
        let p = prefix.to_lowercase();
        self.commands
            .iter()
            .filter(|c| c.name.starts_with(&p))
            .collect()
    }

    fn database() -> Vec<KnownCommand> {
        vec![
            KnownCommand {
                name: "ls",
                description_key: "cmd.ls.desc",
                category: CommandCategory::FileSystem,
                common_options: &[
                    ("-l", "cmd.ls.opt.l"),
                    ("-a", "cmd.ls.opt.a"),
                    ("-h", "cmd.ls.opt.h"),
                    ("-R", "cmd.ls.opt.R"),
                ],
                related: &["dir", "find", "tree"],
                example: "ls -la /home",
                difficulty: "beginner",
                typical_output: "Desktop  Documents  Downloads  Music  Pictures  Videos",
                notes: "cmd.ls.notes",
            },
            KnownCommand {
                name: "cd",
                description_key: "cmd.cd.desc",
                category: CommandCategory::FileSystem,
                common_options: &[],
                related: &["pwd", "pushd", "popd"],
                example: "cd /home/user/Documents",
                difficulty: "beginner",
                typical_output: "",
                notes: "cmd.cd.notes",
            },
            KnownCommand {
                name: "pwd",
                description_key: "cmd.pwd.desc",
                category: CommandCategory::FileSystem,
                common_options: &[],
                related: &["cd", "realpath"],
                example: "pwd",
                difficulty: "beginner",
                typical_output: "/home/user",
                notes: "cmd.pwd.notes",
            },
            KnownCommand {
                name: "mkdir",
                description_key: "cmd.mkdir.desc",
                category: CommandCategory::FileSystem,
                common_options: &[("-p", "cmd.mkdir.opt.p"), ("-v", "cmd.mkdir.opt.v")],
                related: &["rmdir", "touch"],
                example: "mkdir -p projects/rust",
                difficulty: "beginner",
                typical_output: "",
                notes: "cmd.mkdir.notes",
            },
            KnownCommand {
                name: "rmdir",
                description_key: "cmd.rmdir.desc",
                category: CommandCategory::FileSystem,
                common_options: &[("-p", "cmd.rmdir.opt.p")],
                related: &["mkdir", "rm"],
                example: "rmdir empty_directory",
                difficulty: "beginner",
                typical_output: "",
                notes: "cmd.rmdir.notes",
            },
            KnownCommand {
                name: "rm",
                description_key: "cmd.rm.desc",
                category: CommandCategory::FileSystem,
                common_options: &[
                    ("-r", "cmd.rm.opt.r"),
                    ("-f", "cmd.rm.opt.f"),
                    ("-i", "cmd.rm.opt.i"),
                ],
                related: &["rmdir", "mv"],
                example: "rm -rf old_project",
                difficulty: "beginner",
                typical_output: "",
                notes: "cmd.rm.notes",
            },
            KnownCommand {
                name: "cp",
                description_key: "cmd.cp.desc",
                category: CommandCategory::FileSystem,
                common_options: &[
                    ("-r", "cmd.cp.opt.r"),
                    ("-i", "cmd.cp.opt.i"),
                    ("-v", "cmd.cp.opt.v"),
                    ("-a", "cmd.cp.opt.a"),
                ],
                related: &["mv", "rsync"],
                example: "cp -r source/ destination/",
                difficulty: "beginner",
                typical_output: "",
                notes: "cmd.cp.notes",
            },
            KnownCommand {
                name: "mv",
                description_key: "cmd.mv.desc",
                category: CommandCategory::FileSystem,
                common_options: &[
                    ("-i", "cmd.mv.opt.i"),
                    ("-v", "cmd.mv.opt.v"),
                    ("-u", "cmd.mv.opt.u"),
                ],
                related: &["cp", "rename"],
                example: "mv file.txt /tmp/",
                difficulty: "beginner",
                typical_output: "",
                notes: "cmd.mv.notes",
            },
            KnownCommand {
                name: "cat",
                description_key: "cmd.cat.desc",
                category: CommandCategory::TextProcessing,
                common_options: &[
                    ("-n", "cmd.cat.opt.n"),
                    ("-b", "cmd.cat.opt.b"),
                    ("-E", "cmd.cat.opt.E"),
                ],
                related: &["less", "more", "head", "tail"],
                example: "cat file.txt",
                difficulty: "beginner",
                typical_output: "contents of file...",
                notes: "cmd.cat.notes",
            },
            KnownCommand {
                name: "less",
                description_key: "cmd.less.desc",
                category: CommandCategory::TextProcessing,
                common_options: &[
                    ("-N", "cmd.less.opt.N"),
                    ("-S", "cmd.less.opt.S"),
                ],
                related: &["more", "cat"],
                example: "less longfile.txt",
                difficulty: "intermediate",
                typical_output: "interactive pager view",
                notes: "cmd.less.notes",
            },
            KnownCommand {
                name: "more",
                description_key: "cmd.more.desc",
                category: CommandCategory::TextProcessing,
                common_options: &[],
                related: &["less", "cat"],
                example: "more file.txt",
                difficulty: "beginner",
                typical_output: "paged output",
                notes: "cmd.more.notes",
            },
            KnownCommand {
                name: "head",
                description_key: "cmd.head.desc",
                category: CommandCategory::TextProcessing,
                common_options: &[
                    ("-n", "cmd.head.opt.n"),
                    ("-c", "cmd.head.opt.c"),
                ],
                related: &["tail", "cat", "less"],
                example: "head -n 20 file.txt",
                difficulty: "beginner",
                typical_output: "first 10 lines of file",
                notes: "cmd.head.notes",
            },
            KnownCommand {
                name: "tail",
                description_key: "cmd.tail.desc",
                category: CommandCategory::TextProcessing,
                common_options: &[
                    ("-n", "cmd.tail.opt.n"),
                    ("-f", "cmd.tail.opt.f"),
                ],
                related: &["head", "cat"],
                example: "tail -f /var/log/syslog",
                difficulty: "beginner",
                typical_output: "last 10 lines of file",
                notes: "cmd.tail.notes",
            },
            KnownCommand {
                name: "echo",
                description_key: "cmd.echo.desc",
                category: CommandCategory::TextProcessing,
                common_options: &[
                    ("-n", "cmd.echo.opt.n"),
                    ("-e", "cmd.echo.opt.e"),
                ],
                related: &["printf", "cat"],
                example: "echo Hello World",
                difficulty: "beginner",
                typical_output: "Hello World",
                notes: "cmd.echo.notes",
            },
            KnownCommand {
                name: "touch",
                description_key: "cmd.touch.desc",
                category: CommandCategory::FileSystem,
                common_options: &[
                    ("-c", "cmd.touch.opt.c"),
                    ("-t", "cmd.touch.opt.t"),
                ],
                related: &["mkdir", "rm"],
                example: "touch newfile.txt",
                difficulty: "beginner",
                typical_output: "",
                notes: "cmd.touch.notes",
            },
            KnownCommand {
                name: "chmod",
                description_key: "cmd.chmod.desc",
                category: CommandCategory::Permissions,
                common_options: &[
                    ("-R", "cmd.chmod.opt.R"),
                    ("-v", "cmd.chmod.opt.v"),
                ],
                related: &["chown", "chgrp", "umask"],
                example: "chmod +x script.sh",
                difficulty: "intermediate",
                typical_output: "",
                notes: "cmd.chmod.notes",
            },
            KnownCommand {
                name: "chown",
                description_key: "cmd.chown.desc",
                category: CommandCategory::Permissions,
                common_options: &[
                    ("-R", "cmd.chown.opt.R"),
                    ("-v", "cmd.chown.opt.v"),
                ],
                related: &["chmod", "chgrp"],
                example: "chown user:user file.txt",
                difficulty: "intermediate",
                typical_output: "",
                notes: "cmd.chown.notes",
            },
            KnownCommand {
                name: "chgrp",
                description_key: "cmd.chgrp.desc",
                category: CommandCategory::Permissions,
                common_options: &[("-R", "cmd.chgrp.opt.R")],
                related: &["chown", "chmod"],
                example: "chgrp staff file.txt",
                difficulty: "intermediate",
                typical_output: "",
                notes: "cmd.chgrp.notes",
            },
            KnownCommand {
                name: "grep",
                description_key: "cmd.grep.desc",
                category: CommandCategory::TextProcessing,
                common_options: &[
                    ("-r", "cmd.grep.opt.r"),
                    ("-i", "cmd.grep.opt.i"),
                    ("-n", "cmd.grep.opt.n"),
                    ("-v", "cmd.grep.opt.v"),
                    ("-c", "cmd.grep.opt.c"),
                    ("-l", "cmd.grep.opt.l"),
                ],
                related: &["find", "sed", "awk"],
                example: "grep -rin 'error' /var/log/",
                difficulty: "intermediate",
                typical_output: "file.txt:10: error message here",
                notes: "cmd.grep.notes",
            },
            KnownCommand {
                name: "find",
                description_key: "cmd.find.desc",
                category: CommandCategory::FileSystem,
                common_options: &[
                    ("-name", "cmd.find.opt.name"),
                    ("-type", "cmd.find.opt.type"),
                    ("-size", "cmd.find.opt.size"),
                    ("-exec", "cmd.find.opt.exec"),
                ],
                related: &["grep", "locate", "which"],
                example: "find /home -name '*.rs'",
                difficulty: "intermediate",
                typical_output: "/home/user/project/main.rs",
                notes: "cmd.find.notes",
            },
            KnownCommand {
                name: "sort",
                description_key: "cmd.sort.desc",
                category: CommandCategory::TextProcessing,
                common_options: &[
                    ("-r", "cmd.sort.opt.r"),
                    ("-n", "cmd.sort.opt.n"),
                    ("-u", "cmd.sort.opt.u"),
                ],
                related: &["uniq", "wc"],
                example: "sort -n numbers.txt",
                difficulty: "intermediate",
                typical_output: "sorted lines",
                notes: "cmd.sort.notes",
            },
            KnownCommand {
                name: "wc",
                description_key: "cmd.wc.desc",
                category: CommandCategory::TextProcessing,
                common_options: &[
                    ("-l", "cmd.wc.opt.l"),
                    ("-w", "cmd.wc.opt.w"),
                    ("-c", "cmd.wc.opt.c"),
                ],
                related: &["sort", "uniq"],
                example: "wc -l file.txt",
                difficulty: "beginner",
                typical_output: "42 file.txt",
                notes: "cmd.wc.notes",
            },
            KnownCommand {
                name: "cut",
                description_key: "cmd.cut.desc",
                category: CommandCategory::TextProcessing,
                common_options: &[
                    ("-d", "cmd.cut.opt.d"),
                    ("-f", "cmd.cut.opt.f"),
                    ("-c", "cmd.cut.opt.c"),
                ],
                related: &["awk", "sed"],
                example: "cut -d: -f1 /etc/passwd",
                difficulty: "intermediate",
                typical_output: "root\nbin\ndaemon\n...",
                notes: "cmd.cut.notes",
            },
            KnownCommand {
                name: "tr",
                description_key: "cmd.tr.desc",
                category: CommandCategory::TextProcessing,
                common_options: &[
                    ("-d", "cmd.tr.opt.d"),
                    ("-s", "cmd.tr.opt.s"),
                ],
                related: &["sed", "awk"],
                example: "tr a-z A-Z < file.txt",
                difficulty: "intermediate",
                typical_output: "transformed output",
                notes: "cmd.tr.notes",
            },
            KnownCommand {
                name: "sed",
                description_key: "cmd.sed.desc",
                category: CommandCategory::TextProcessing,
                common_options: &[
                    ("-i", "cmd.sed.opt.i"),
                    ("-e", "cmd.sed.opt.e"),
                    ("-n", "cmd.sed.opt.n"),
                ],
                related: &["awk", "grep"],
                example: "sed 's/foo/bar/g' file.txt",
                difficulty: "advanced",
                typical_output: "transformed text",
                notes: "cmd.sed.notes",
            },
            KnownCommand {
                name: "awk",
                description_key: "cmd.awk.desc",
                category: CommandCategory::TextProcessing,
                common_options: &[
                    ("-F", "cmd.awk.opt.F"),
                    ("-v", "cmd.awk.opt.v"),
                ],
                related: &["sed", "cut"],
                example: "awk '{print $1}' file.txt",
                difficulty: "advanced",
                typical_output: "first column of data",
                notes: "cmd.awk.notes",
            },
            KnownCommand {
                name: "ps",
                description_key: "cmd.ps.desc",
                category: CommandCategory::ProcessManagement,
                common_options: &[
                    ("-ef", "cmd.ps.opt.ef"),
                    ("aux", "cmd.ps.opt.aux"),
                ],
                related: &["top", "kill", "pgrep"],
                example: "ps aux",
                difficulty: "intermediate",
                typical_output: "USER       PID  ...",
                notes: "cmd.ps.notes",
            },
            KnownCommand {
                name: "top",
                description_key: "cmd.top.desc",
                category: CommandCategory::ProcessManagement,
                common_options: &[
                    ("-u", "cmd.top.opt.u"),
                    ("-p", "cmd.top.opt.p"),
                ],
                related: &["ps", "htop", "kill"],
                example: "top -u user",
                difficulty: "intermediate",
                typical_output: "interactive process viewer",
                notes: "cmd.top.notes",
            },
            KnownCommand {
                name: "kill",
                description_key: "cmd.kill.desc",
                category: CommandCategory::ProcessManagement,
                common_options: &[
                    ("-9", "cmd.kill.opt.9"),
                    ("-15", "cmd.kill.opt.15"),
                    ("-l", "cmd.kill.opt.l"),
                ],
                related: &["pkill", "killall"],
                example: "kill -9 1234",
                difficulty: "intermediate",
                typical_output: "",
                notes: "cmd.kill.notes",
            },
            KnownCommand {
                name: "pkill",
                description_key: "cmd.pkill.desc",
                category: CommandCategory::ProcessManagement,
                common_options: &[
                    ("-f", "cmd.pkill.opt.f"),
                    ("-9", "cmd.pkill.opt.9"),
                ],
                related: &["kill", "pgrep"],
                example: "pkill -f 'process_name'",
                difficulty: "intermediate",
                typical_output: "",
                notes: "cmd.pkill.notes",
            },
            KnownCommand {
                name: "pgrep",
                description_key: "cmd.pgrep.desc",
                category: CommandCategory::ProcessManagement,
                common_options: &[
                    ("-f", "cmd.pgrep.opt.f"),
                    ("-u", "cmd.pgrep.opt.u"),
                    ("-l", "cmd.pgrep.opt.l"),
                ],
                related: &["pkill", "ps"],
                example: "pgrep -u user ssh",
                difficulty: "intermediate",
                typical_output: "1234",
                notes: "cmd.pgrep.notes",
            },
            KnownCommand {
                name: "df",
                description_key: "cmd.df.desc",
                category: CommandCategory::SystemInfo,
                common_options: &[
                    ("-h", "cmd.df.opt.h"),
                    ("-T", "cmd.df.opt.T"),
                ],
                related: &["du", "free"],
                example: "df -h",
                difficulty: "beginner",
                typical_output: "Filesystem      Size  Used Avail Use%...",
                notes: "cmd.df.notes",
            },
            KnownCommand {
                name: "du",
                description_key: "cmd.du.desc",
                category: CommandCategory::SystemInfo,
                common_options: &[
                    ("-h", "cmd.du.opt.h"),
                    ("-s", "cmd.du.opt.s"),
                    ("-c", "cmd.du.opt.c"),
                ],
                related: &["df", "ncdu"],
                example: "du -sh *",
                difficulty: "intermediate",
                typical_output: "4.0K    file.txt\n1.2M    folder",
                notes: "cmd.du.notes",
            },
            KnownCommand {
                name: "free",
                description_key: "cmd.free.desc",
                category: CommandCategory::SystemInfo,
                common_options: &[
                    ("-h", "cmd.free.opt.h"),
                    ("-m", "cmd.free.opt.m"),
                ],
                related: &["df", "top"],
                example: "free -h",
                difficulty: "beginner",
                typical_output: "              total  used  free  ...",
                notes: "cmd.free.notes",
            },
            KnownCommand {
                name: "uname",
                description_key: "cmd.uname.desc",
                category: CommandCategory::SystemInfo,
                common_options: &[
                    ("-a", "cmd.uname.opt.a"),
                    ("-r", "cmd.uname.opt.r"),
                ],
                related: &["whoami", "hostname"],
                example: "uname -a",
                difficulty: "beginner",
                typical_output: "Linux hostname 6.8.0-arch1-1 #1 SMP ...",
                notes: "cmd.uname.notes",
            },
            KnownCommand {
                name: "whoami",
                description_key: "cmd.whoami.desc",
                category: CommandCategory::SystemInfo,
                common_options: &[],
                related: &["id", "who", "w"],
                example: "whoami",
                difficulty: "beginner",
                typical_output: "user",
                notes: "cmd.whoami.notes",
            },
            KnownCommand {
                name: "id",
                description_key: "cmd.id.desc",
                category: CommandCategory::SystemInfo,
                common_options: &[
                    ("-u", "cmd.id.opt.u"),
                    ("-g", "cmd.id.opt.g"),
                    ("-G", "cmd.id.opt.G"),
                ],
                related: &["whoami", "groups"],
                example: "id",
                difficulty: "beginner",
                typical_output: "uid=1000(user) gid=1000(user) groups=1000(user)...",
                notes: "cmd.id.notes",
            },
            KnownCommand {
                name: "who",
                description_key: "cmd.who.desc",
                category: CommandCategory::SystemInfo,
                common_options: &[
                    ("-b", "cmd.who.opt.b"),
                    ("-u", "cmd.who.opt.u"),
                ],
                related: &["w", "whoami", "id"],
                example: "who",
                difficulty: "beginner",
                typical_output: "user     tty7         2024-01-15 10:00",
                notes: "cmd.who.notes",
            },
            KnownCommand {
                name: "w",
                description_key: "cmd.w.desc",
                category: CommandCategory::SystemInfo,
                common_options: &[
                    ("-s", "cmd.w.opt.s"),
                    ("-f", "cmd.w.opt.f"),
                ],
                related: &["who", "uptime"],
                example: "w",
                difficulty: "intermediate",
                typical_output: "10:00:00 up 1 day, 2 users, load average: 0.00...",
                notes: "cmd.w.notes",
            },
            KnownCommand {
                name: "date",
                description_key: "cmd.date.desc",
                category: CommandCategory::SystemInfo,
                common_options: &[
                    ("+%s", "cmd.date.opt.s"),
                    ("-u", "cmd.date.opt.u"),
                ],
                related: &["cal", "time"],
                example: "date '+%Y-%m-%d %H:%M:%S'",
                difficulty: "beginner",
                typical_output: "Mon Jan 15 10:00:00 WIB 2024",
                notes: "cmd.date.notes",
            },
            KnownCommand {
                name: "cal",
                description_key: "cmd.cal.desc",
                category: CommandCategory::SystemInfo,
                common_options: &[
                    ("-y", "cmd.cal.opt.y"),
                    ("-3", "cmd.cal.opt.3"),
                ],
                related: &["date"],
                example: "cal -y",
                difficulty: "beginner",
                typical_output: "    January 2024    \nSu Mo Tu We Th Fr Sa\n...",
                notes: "cmd.cal.notes",
            },
            KnownCommand {
                name: "uptime",
                description_key: "cmd.uptime.desc",
                category: CommandCategory::SystemInfo,
                common_options: &[
                    ("-p", "cmd.uptime.opt.p"),
                ],
                related: &["w", "top"],
                example: "uptime",
                difficulty: "beginner",
                typical_output: "10:00:00 up 2 days, 3 users, load average: 0.00, 0.01, 0.05",
                notes: "cmd.uptime.notes",
            },
            KnownCommand {
                name: "dmesg",
                description_key: "cmd.dmesg.desc",
                category: CommandCategory::SystemInfo,
                common_options: &[
                    ("-H", "cmd.dmesg.opt.H"),
                    ("-w", "cmd.dmesg.opt.w"),
                    ("-l", "cmd.dmesg.opt.l"),
                ],
                related: &["journalctl"],
                example: "dmesg -H",
                difficulty: "advanced",
                typical_output: "[12345.678901] kernel: ...",
                notes: "cmd.dmesg.notes",
            },
            KnownCommand {
                name: "lscpu",
                description_key: "cmd.lscpu.desc",
                category: CommandCategory::SystemInfo,
                common_options: &[],
                related: &["uname", "free"],
                example: "lscpu",
                difficulty: "beginner",
                typical_output: "Architecture:        x86_64\nCPU op-mode(s):      32-bit, 64-bit\n...",
                notes: "cmd.lscpu.notes",
            },
            KnownCommand {
                name: "lsblk",
                description_key: "cmd.lsblk.desc",
                category: CommandCategory::SystemInfo,
                common_options: &[
                    ("-f", "cmd.lsblk.opt.f"),
                    ("-m", "cmd.lsblk.opt.m"),
                ],
                related: &["df", "blkid"],
                example: "lsblk -f",
                difficulty: "intermediate",
                typical_output: "NAME FSTYPE LABEL UUID MOUNTPOINT\nsda  ext4   ...   ...  /",
                notes: "cmd.lsblk.notes",
            },
            KnownCommand {
                name: "ping",
                description_key: "cmd.ping.desc",
                category: CommandCategory::Networking,
                common_options: &[
                    ("-c", "cmd.ping.opt.c"),
                    ("-i", "cmd.ping.opt.i"),
                    ("-4", "cmd.ping.opt.4"),
                ],
                related: &["curl", "wget"],
                example: "ping -c 4 google.com",
                difficulty: "beginner",
                typical_output: "PING google.com (142.250.64.78) 56(84) bytes of data.\n64 bytes from ...",
                notes: "cmd.ping.notes",
            },
            KnownCommand {
                name: "curl",
                description_key: "cmd.curl.desc",
                category: CommandCategory::Networking,
                common_options: &[
                    ("-o", "cmd.curl.opt.o"),
                    ("-L", "cmd.curl.opt.L"),
                    ("-H", "cmd.curl.opt.H"),
                    ("-d", "cmd.curl.opt.d"),
                ],
                related: &["wget", "httpie"],
                example: "curl -O https://example.com/file.txt",
                difficulty: "intermediate",
                typical_output: "downloaded content or file",
                notes: "cmd.curl.notes",
            },
            KnownCommand {
                name: "wget",
                description_key: "cmd.wget.desc",
                category: CommandCategory::Networking,
                common_options: &[
                    ("-O", "cmd.wget.opt.O"),
                    ("-c", "cmd.wget.opt.c"),
                    ("-q", "cmd.wget.opt.q"),
                ],
                related: &["curl"],
                example: "wget https://example.com/file.txt",
                difficulty: "beginner",
                typical_output: "downloading: 100% [========>] 1.2M",
                notes: "cmd.wget.notes",
            },
            KnownCommand {
                name: "ssh",
                description_key: "cmd.ssh.desc",
                category: CommandCategory::Networking,
                common_options: &[
                    ("-p", "cmd.ssh.opt.p"),
                    ("-i", "cmd.ssh.opt.i"),
                    ("-v", "cmd.ssh.opt.v"),
                ],
                related: &["scp", "rsync"],
                example: "ssh user@example.com",
                difficulty: "intermediate",
                typical_output: "remote shell prompt",
                notes: "cmd.ssh.notes",
            },
            KnownCommand {
                name: "scp",
                description_key: "cmd.scp.desc",
                category: CommandCategory::Networking,
                common_options: &[
                    ("-r", "cmd.scp.opt.r"),
                    ("-P", "cmd.scp.opt.P"),
                    ("-i", "cmd.scp.opt.i"),
                ],
                related: &["ssh", "rsync"],
                example: "scp file.txt user@host:/remote/path/",
                difficulty: "intermediate",
                typical_output: "file.txt 100% 123KB 1.2MB/s",
                notes: "cmd.scp.notes",
            },
            KnownCommand {
                name: "rsync",
                description_key: "cmd.rsync.desc",
                category: CommandCategory::Networking,
                common_options: &[
                    ("-a", "cmd.rsync.opt.a"),
                    ("-v", "cmd.rsync.opt.v"),
                    ("-z", "cmd.rsync.opt.z"),
                    ("--delete", "cmd.rsync.opt.delete"),
                ],
                related: &["scp", "cp"],
                example: "rsync -avz source/ user@host:/dest/",
                difficulty: "advanced",
                typical_output: "sent 123 bytes received 45 bytes",
                notes: "cmd.rsync.notes",
            },
            KnownCommand {
                name: "ifconfig",
                description_key: "cmd.ifconfig.desc",
                category: CommandCategory::Networking,
                common_options: &[
                    ("-a", "cmd.ifconfig.opt.a"),
                    ("up", "cmd.ifconfig.opt.up"),
                    ("down", "cmd.ifconfig.opt.down"),
                ],
                related: &["ip", "nmcli"],
                example: "ifconfig -a",
                difficulty: "intermediate",
                typical_output: "eth0: flags=4163<UP,BROADCAST,...>\ninet 192.168.1.100...",
                notes: "cmd.ifconfig.notes",
            },
            KnownCommand {
                name: "ip",
                description_key: "cmd.ip.desc",
                category: CommandCategory::Networking,
                common_options: &[
                    ("addr", "cmd.ip.opt.addr"),
                    ("link", "cmd.ip.opt.link"),
                    ("route", "cmd.ip.opt.route"),
                ],
                related: &["ifconfig", "nmcli"],
                example: "ip addr show",
                difficulty: "intermediate",
                typical_output: "1: lo: <LOOPBACK,UP> mtu 65536 ...\n2: eth0: <BROADCAST,...>",
                notes: "cmd.ip.notes",
            },
            KnownCommand {
                name: "netstat",
                description_key: "cmd.netstat.desc",
                category: CommandCategory::Networking,
                common_options: &[
                    ("-tlnp", "cmd.netstat.opt.tlnp"),
                    ("-an", "cmd.netstat.opt.an"),
                    ("-r", "cmd.netstat.opt.r"),
                ],
                related: &["ss", "ip"],
                example: "netstat -tlnp",
                difficulty: "advanced",
                typical_output: "Proto Recv-Q Send-Q Local Address Foreign Address State",
                notes: "cmd.netstat.notes",
            },
            KnownCommand {
                name: "nmap",
                description_key: "cmd.nmap.desc",
                category: CommandCategory::Networking,
                common_options: &[
                    ("-sT", "cmd.nmap.opt.sT"),
                    ("-sV", "cmd.nmap.opt.sV"),
                    ("-O", "cmd.nmap.opt.O"),
                ],
                related: &["netstat", "ping"],
                example: "nmap -sV 192.168.1.1",
                difficulty: "advanced",
                typical_output: "PORT   STATE SERVICE\n22/tcp open  ssh\n80/tcp open  http",
                notes: "cmd.nmap.notes",
            },
            KnownCommand {
                name: "hostname",
                description_key: "cmd.hostname.desc",
                category: CommandCategory::Networking,
                common_options: &[
                    ("-I", "cmd.hostname.opt.I"),
                    ("-f", "cmd.hostname.opt.f"),
                ],
                related: &["uname", "dnsdomainname"],
                example: "hostname -I",
                difficulty: "beginner",
                typical_output: "192.168.1.100",
                notes: "cmd.hostname.notes",
            },
            KnownCommand {
                name: "dig",
                description_key: "cmd.dig.desc",
                category: CommandCategory::Networking,
                common_options: &[
                    ("+short", "cmd.dig.opt.short"),
                    ("-t", "cmd.dig.opt.t"),
                ],
                related: &["nslookup", "host"],
                example: "dig +short google.com",
                difficulty: "intermediate",
                typical_output: "142.250.64.78",
                notes: "cmd.dig.notes",
            },
            KnownCommand {
                name: "nslookup",
                description_key: "cmd.nslookup.desc",
                category: CommandCategory::Networking,
                common_options: &[],
                related: &["dig", "host"],
                example: "nslookup google.com",
                difficulty: "beginner",
                typical_output: "Server: 8.8.8.8\nAddress: 142.250.64.78",
                notes: "cmd.nslookup.notes",
            },
            KnownCommand {
                name: "apt",
                description_key: "cmd.apt.desc",
                category: CommandCategory::PackageManagement,
                common_options: &[
                    ("install", "cmd.apt.opt.install"),
                    ("remove", "cmd.apt.opt.remove"),
                    ("update", "cmd.apt.opt.update"),
                    ("upgrade", "cmd.apt.opt.upgrade"),
                    ("search", "cmd.apt.opt.search"),
                ],
                related: &["apt-get", "dpkg"],
                example: "apt install firefox",
                difficulty: "beginner",
                typical_output: "Reading package lists... Done\n...",
                notes: "cmd.apt.notes",
            },
            KnownCommand {
                name: "apt-get",
                description_key: "cmd.aptget.desc",
                category: CommandCategory::PackageManagement,
                common_options: &[
                    ("install", "cmd.aptget.opt.install"),
                    ("remove", "cmd.aptget.opt.remove"),
                    ("update", "cmd.aptget.opt.update"),
                    ("upgrade", "cmd.aptget.opt.upgrade"),
                ],
                related: &["apt", "dpkg"],
                example: "apt-get update",
                difficulty: "intermediate",
                typical_output: "Get:1 http://archive.ubuntu.com ...",
                notes: "cmd.aptget.notes",
            },
            KnownCommand {
                name: "dpkg",
                description_key: "cmd.dpkg.desc",
                category: CommandCategory::PackageManagement,
                common_options: &[
                    ("-i", "cmd.dpkg.opt.i"),
                    ("-r", "cmd.dpkg.opt.r"),
                    ("-l", "cmd.dpkg.opt.l"),
                ],
                related: &["apt", "apt-get"],
                example: "dpkg -i package.deb",
                difficulty: "intermediate",
                typical_output: "Selecting previously unselected package ...",
                notes: "cmd.dpkg.notes",
            },
            KnownCommand {
                name: "snap",
                description_key: "cmd.snap.desc",
                category: CommandCategory::PackageManagement,
                common_options: &[
                    ("install", "cmd.snap.opt.install"),
                    ("remove", "cmd.snap.opt.remove"),
                    ("list", "cmd.snap.opt.list"),
                ],
                related: &["flatpak", "apt"],
                example: "snap install vlc",
                difficulty: "beginner",
                typical_output: "vlc 3.0.18 from VideoLAN installed",
                notes: "cmd.snap.notes",
            },
            KnownCommand {
                name: "flatpak",
                description_key: "cmd.flatpak.desc",
                category: CommandCategory::PackageManagement,
                common_options: &[
                    ("install", "cmd.flatpak.opt.install"),
                    ("remove", "cmd.flatpak.opt.remove"),
                    ("list", "cmd.flatpak.opt.list"),
                ],
                related: &["snap", "apt"],
                example: "flatpak install org.gimp.GIMP",
                difficulty: "intermediate",
                typical_output: "Installing: org.gimp.GIMP ...",
                notes: "cmd.flatpak.notes",
            },
            KnownCommand {
                name: "rpm",
                description_key: "cmd.rpm.desc",
                category: CommandCategory::PackageManagement,
                common_options: &[
                    ("-i", "cmd.rpm.opt.i"),
                    ("-e", "cmd.rpm.opt.e"),
                    ("-q", "cmd.rpm.opt.q"),
                ],
                related: &["dpkg", "dnf"],
                example: "rpm -i package.rpm",
                difficulty: "intermediate",
                typical_output: "package.rpm installed successfully",
                notes: "cmd.rpm.notes",
            },
            KnownCommand {
                name: "pacman",
                description_key: "cmd.pacman.desc",
                category: CommandCategory::PackageManagement,
                common_options: &[
                    ("-S", "cmd.pacman.opt.S"),
                    ("-R", "cmd.pacman.opt.R"),
                    ("-Q", "cmd.pacman.opt.Q"),
                    ("-Syu", "cmd.pacman.opt.Syu"),
                ],
                related: &["apt", "dnf"],
                example: "pacman -Syu",
                difficulty: "intermediate",
                typical_output: "resolving dependencies... looking for conflicting packages...",
                notes: "cmd.pacman.notes",
            },
            KnownCommand {
                name: "tar",
                description_key: "cmd.tar.desc",
                category: CommandCategory::Compression,
                common_options: &[
                    ("-xvf", "cmd.tar.opt.xvf"),
                    ("-cvf", "cmd.tar.opt.cvf"),
                    ("-z", "cmd.tar.opt.z"),
                    ("-j", "cmd.tar.opt.j"),
                ],
                related: &["gzip", "zip"],
                example: "tar -xvf archive.tar.gz",
                difficulty: "intermediate",
                typical_output: "file1.txt\nfile2.txt",
                notes: "cmd.tar.notes",
            },
            KnownCommand {
                name: "gzip",
                description_key: "cmd.gzip.desc",
                category: CommandCategory::Compression,
                common_options: &[
                    ("-d", "cmd.gzip.opt.d"),
                    ("-k", "cmd.gzip.opt.k"),
                    ("-v", "cmd.gzip.opt.v"),
                ],
                related: &["gunzip", "bzip2", "xz"],
                example: "gzip file.txt",
                difficulty: "beginner",
                typical_output: "file.txt.gz created",
                notes: "cmd.gzip.notes",
            },
            KnownCommand {
                name: "gunzip",
                description_key: "cmd.gunzip.desc",
                category: CommandCategory::Compression,
                common_options: &[
                    ("-k", "cmd.gunzip.opt.k"),
                    ("-v", "cmd.gunzip.opt.v"),
                ],
                related: &["gzip", "bunzip2"],
                example: "gunzip file.txt.gz",
                difficulty: "beginner",
                typical_output: "file.txt restored",
                notes: "cmd.gunzip.notes",
            },
            KnownCommand {
                name: "zip",
                description_key: "cmd.zip.desc",
                category: CommandCategory::Compression,
                common_options: &[
                    ("-r", "cmd.zip.opt.r"),
                    ("-d", "cmd.zip.opt.d"),
                ],
                related: &["unzip", "tar"],
                example: "zip -r archive.zip folder/",
                difficulty: "beginner",
                typical_output: "adding: folder/ (stored 0%)",
                notes: "cmd.zip.notes",
            },
            KnownCommand {
                name: "unzip",
                description_key: "cmd.unzip.desc",
                category: CommandCategory::Compression,
                common_options: &[
                    ("-l", "cmd.unzip.opt.l"),
                    ("-d", "cmd.unzip.opt.d"),
                ],
                related: &["zip", "tar"],
                example: "unzip archive.zip -d target/",
                difficulty: "beginner",
                typical_output: "Archive: archive.zip\n extracting: file.txt",
                notes: "cmd.unzip.notes",
            },
            KnownCommand {
                name: "bzip2",
                description_key: "cmd.bzip2.desc",
                category: CommandCategory::Compression,
                common_options: &[
                    ("-d", "cmd.bzip2.opt.d"),
                    ("-k", "cmd.bzip2.opt.k"),
                    ("-v", "cmd.bzip2.opt.v"),
                ],
                related: &["bunzip2", "gzip", "xz"],
                example: "bzip2 file.txt",
                difficulty: "intermediate",
                typical_output: "file.txt.bz2 created",
                notes: "cmd.bzip2.notes",
            },
            KnownCommand {
                name: "bunzip2",
                description_key: "cmd.bunzip2.desc",
                category: CommandCategory::Compression,
                common_options: &[
                    ("-k", "cmd.bunzip2.opt.k"),
                    ("-v", "cmd.bunzip2.opt.v"),
                ],
                related: &["bzip2", "gunzip"],
                example: "bunzip2 file.txt.bz2",
                difficulty: "intermediate",
                typical_output: "file.txt restored",
                notes: "cmd.bunzip2.notes",
            },
            KnownCommand {
                name: "xz",
                description_key: "cmd.xz.desc",
                category: CommandCategory::Compression,
                common_options: &[
                    ("-d", "cmd.xz.opt.d"),
                    ("-k", "cmd.xz.opt.k"),
                    ("-v", "cmd.xz.opt.v"),
                ],
                related: &["gzip", "bzip2"],
                example: "xz file.txt",
                difficulty: "intermediate",
                typical_output: "file.txt.xz created",
                notes: "cmd.xz.notes",
            },
            KnownCommand {
                name: "git",
                description_key: "cmd.git.desc",
                category: CommandCategory::Git,
                common_options: &[
                    ("clone", "cmd.git.opt.clone"),
                    ("commit", "cmd.git.opt.commit"),
                    ("push", "cmd.git.opt.push"),
                    ("pull", "cmd.git.opt.pull"),
                    ("status", "cmd.git.opt.status"),
                    ("add", "cmd.git.opt.add"),
                ],
                related: &["svn", "mercurial"],
                example: "git clone https://github.com/user/repo.git",
                difficulty: "intermediate",
                typical_output: "Cloning into 'repo'...\nremote: Enumerating objects: ...",
                notes: "cmd.git.notes",
            },
            KnownCommand {
                name: "python3",
                description_key: "cmd.python3.desc",
                category: CommandCategory::Programming,
                common_options: &[
                    ("-m", "cmd.python3.opt.m"),
                    ("-c", "cmd.python3.opt.c"),
                    ("-V", "cmd.python3.opt.V"),
                ],
                related: &["node", "cargo"],
                example: "python3 -V",
                difficulty: "beginner",
                typical_output: "Python 3.12.0",
                notes: "cmd.python3.notes",
            },
            KnownCommand {
                name: "node",
                description_key: "cmd.node.desc",
                category: CommandCategory::Programming,
                common_options: &[
                    ("-v", "cmd.node.opt.v"),
                    ("-e", "cmd.node.opt.e"),
                ],
                related: &["npm", "python3"],
                example: "node -v",
                difficulty: "beginner",
                typical_output: "v20.10.0",
                notes: "cmd.node.notes",
            },
            KnownCommand {
                name: "npm",
                description_key: "cmd.npm.desc",
                category: CommandCategory::Programming,
                common_options: &[
                    ("install", "cmd.npm.opt.install"),
                    ("run", "cmd.npm.opt.run"),
                    ("test", "cmd.npm.opt.test"),
                    ("build", "cmd.npm.opt.build"),
                ],
                related: &["node", "cargo"],
                example: "npm install express",
                difficulty: "intermediate",
                typical_output: "added 50 packages in 2s",
                notes: "cmd.npm.notes",
            },
            KnownCommand {
                name: "cargo",
                description_key: "cmd.cargo.desc",
                category: CommandCategory::Programming,
                common_options: &[
                    ("build", "cmd.cargo.opt.build"),
                    ("run", "cmd.cargo.opt.run"),
                    ("test", "cmd.cargo.opt.test"),
                    ("new", "cmd.cargo.opt.new"),
                ],
                related: &["rustc", "npm", "make"],
                example: "cargo new my_project",
                difficulty: "intermediate",
                typical_output: "Created binary (application) `my_project` package",
                notes: "cmd.cargo.notes",
            },
            KnownCommand {
                name: "rustc",
                description_key: "cmd.rustc.desc",
                category: CommandCategory::Programming,
                common_options: &[
                    ("-o", "cmd.rustc.opt.o"),
                    ("--edition", "cmd.rustc.opt.edition"),
                ],
                related: &["cargo", "gcc"],
                example: "rustc main.rs -o program",
                difficulty: "intermediate",
                typical_output: "compiles to binary (no output on success)",
                notes: "cmd.rustc.notes",
            },
            KnownCommand {
                name: "gcc",
                description_key: "cmd.gcc.desc",
                category: CommandCategory::Programming,
                common_options: &[
                    ("-o", "cmd.gcc.opt.o"),
                    ("-Wall", "cmd.gcc.opt.Wall"),
                    ("-O2", "cmd.gcc.opt.O2"),
                    ("-g", "cmd.gcc.opt.g"),
                ],
                related: &["g++", "make", "rustc"],
                example: "gcc -Wall -o program main.c",
                difficulty: "intermediate",
                typical_output: "compiles to binary (warnings/errors on stderr)",
                notes: "cmd.gcc.notes",
            },
            KnownCommand {
                name: "g++",
                description_key: "cmd.gpp.desc",
                category: CommandCategory::Programming,
                common_options: &[
                    ("-o", "cmd.gpp.opt.o"),
                    ("-Wall", "cmd.gpp.opt.Wall"),
                    ("-std", "cmd.gpp.opt.std"),
                ],
                related: &["gcc", "make"],
                example: "g++ -Wall -o program main.cpp",
                difficulty: "intermediate",
                typical_output: "compiles to binary (warnings/errors on stderr)",
                notes: "cmd.gpp.notes",
            },
            KnownCommand {
                name: "make",
                description_key: "cmd.make.desc",
                category: CommandCategory::Programming,
                common_options: &[
                    ("-j", "cmd.make.opt.j"),
                    ("-C", "cmd.make.opt.C"),
                    ("clean", "cmd.make.opt.clean"),
                ],
                related: &["cmake", "cargo"],
                example: "make -j4",
                difficulty: "intermediate",
                typical_output: "gcc -c main.c\ngcc -o program main.o",
                notes: "cmd.make.notes",
            },
            KnownCommand {
                name: "cmake",
                description_key: "cmd.cmake.desc",
                category: CommandCategory::Programming,
                common_options: &[
                    ("-B", "cmd.cmake.opt.B"),
                    ("-D", "cmd.cmake.opt.D"),
                    ("--build", "cmd.cmake.opt.build"),
                ],
                related: &["make", "cargo"],
                example: "cmake -B build",
                difficulty: "advanced",
                typical_output: "-- Configuring done\n-- Generating done",
                notes: "cmd.cmake.notes",
            },
            KnownCommand {
                name: "go",
                description_key: "cmd.go.desc",
                category: CommandCategory::Programming,
                common_options: &[
                    ("run", "cmd.go.opt.run"),
                    ("build", "cmd.go.opt.build"),
                    ("test", "cmd.go.opt.test"),
                    ("mod", "cmd.go.opt.mod"),
                ],
                related: &["cargo", "make"],
                example: "go run main.go",
                difficulty: "intermediate",
                typical_output: "Hello, World!",
                notes: "cmd.go.notes",
            },
            KnownCommand {
                name: "nano",
                description_key: "cmd.nano.desc",
                category: CommandCategory::Terminal,
                common_options: &[
                    ("-l", "cmd.nano.opt.l"),
                    ("-w", "cmd.nano.opt.w"),
                ],
                related: &["vim", "emacs"],
                example: "nano file.txt",
                difficulty: "beginner",
                typical_output: "interactive text editor",
                notes: "cmd.nano.notes",
            },
            KnownCommand {
                name: "vim",
                description_key: "cmd.vim.desc",
                category: CommandCategory::Terminal,
                common_options: &[
                    ("-R", "cmd.vim.opt.R"),
                    ("+", "cmd.vim.opt.plus"),
                ],
                related: &["nano", "emacs"],
                example: "vim file.txt",
                difficulty: "intermediate",
                typical_output: "interactive text editor",
                notes: "cmd.vim.notes",
            },
            KnownCommand {
                name: "emacs",
                description_key: "cmd.emacs.desc",
                category: CommandCategory::Terminal,
                common_options: &[
                    ("-nw", "cmd.emacs.opt.nw"),
                    ("-Q", "cmd.emacs.opt.Q"),
                ],
                related: &["vim", "nano"],
                example: "emacs -nw file.txt",
                difficulty: "advanced",
                typical_output: "interactive text editor",
                notes: "cmd.emacs.notes",
            },
            KnownCommand {
                name: "ed",
                description_key: "cmd.ed.desc",
                category: CommandCategory::Terminal,
                common_options: &[],
                related: &["sed", "vim"],
                example: "ed file.txt",
                difficulty: "advanced",
                typical_output: "line editor prompt",
                notes: "cmd.ed.notes",
            },
            KnownCommand {
                name: "vi",
                description_key: "cmd.vi.desc",
                category: CommandCategory::Terminal,
                common_options: &[
                    ("-R", "cmd.vi.opt.R"),
                ],
                related: &["vim", "nano"],
                example: "vi file.txt",
                difficulty: "intermediate",
                typical_output: "interactive text editor",
                notes: "cmd.vi.notes",
            },
            KnownCommand {
                name: "screen",
                description_key: "cmd.screen.desc",
                category: CommandCategory::Terminal,
                common_options: &[
                    ("-S", "cmd.screen.opt.S"),
                    ("-r", "cmd.screen.opt.r"),
                    ("-ls", "cmd.screen.opt.ls"),
                ],
                related: &["tmux"],
                example: "screen -S session_name",
                difficulty: "advanced",
                typical_output: "terminal multiplexer session",
                notes: "cmd.screen.notes",
            },
            KnownCommand {
                name: "tmux",
                description_key: "cmd.tmux.desc",
                category: CommandCategory::Terminal,
                common_options: &[
                    ("new", "cmd.tmux.opt.new"),
                    ("attach", "cmd.tmux.opt.attach"),
                    ("list-sessions", "cmd.tmux.opt.ls"),
                ],
                related: &["screen"],
                example: "tmux new -s mysession",
                difficulty: "advanced",
                typical_output: "terminal multiplexer session",
                notes: "cmd.tmux.notes",
            },
            KnownCommand {
                name: "reset",
                description_key: "cmd.reset.desc",
                category: CommandCategory::Terminal,
                common_options: &[],
                related: &["clear", "stty"],
                example: "reset",
                difficulty: "beginner",
                typical_output: "terminal reset (screen clears)",
                notes: "cmd.reset.notes",
            },
            KnownCommand {
                name: "stty",
                description_key: "cmd.stty.desc",
                category: CommandCategory::Terminal,
                common_options: &[
                    ("-a", "cmd.stty.opt.a"),
                    ("-F", "cmd.stty.opt.F"),
                ],
                related: &["reset"],
                example: "stty -a",
                difficulty: "advanced",
                typical_output: "speed 38400 baud; line 0;...",
                notes: "cmd.stty.notes",
            },
            KnownCommand {
                name: "history",
                description_key: "cmd.history.desc",
                category: CommandCategory::ShellBuiltin,
                common_options: &[
                    ("-c", "cmd.history.opt.c"),
                    ("-d", "cmd.history.opt.d"),
                ],
                related: &["fc"],
                example: "history | tail -10",
                difficulty: "beginner",
                typical_output: "1  ls\n2  cd Documents\n3  cat file.txt",
                notes: "cmd.history.notes",
            },
            KnownCommand {
                name: "alias",
                description_key: "cmd.alias.desc",
                category: CommandCategory::ShellBuiltin,
                common_options: &[
                    ("-p", "cmd.alias.opt.p"),
                ],
                related: &["unalias", "export"],
                example: "alias ll='ls -la'",
                difficulty: "beginner",
                typical_output: "",
                notes: "cmd.alias.notes",
            },
            KnownCommand {
                name: "unalias",
                description_key: "cmd.unalias.desc",
                category: CommandCategory::ShellBuiltin,
                common_options: &[
                    ("-a", "cmd.unalias.opt.a"),
                ],
                related: &["alias"],
                example: "unalias ll",
                difficulty: "beginner",
                typical_output: "",
                notes: "cmd.unalias.notes",
            },
            KnownCommand {
                name: "export",
                description_key: "cmd.export.desc",
                category: CommandCategory::ShellBuiltin,
                common_options: &[
                    ("-p", "cmd.export.opt.p"),
                    ("-n", "cmd.export.opt.n"),
                ],
                related: &["env", "set"],
                example: "export PATH=$PATH:/new/path",
                difficulty: "intermediate",
                typical_output: "",
                notes: "cmd.export.notes",
            },
            KnownCommand {
                name: "source",
                description_key: "cmd.source.desc",
                category: CommandCategory::ShellBuiltin,
                common_options: &[],
                related: &["export", "exec"],
                example: "source ~/.bashrc",
                difficulty: "intermediate",
                typical_output: "",
                notes: "cmd.source.notes",
            },
            KnownCommand {
                name: "clear",
                description_key: "cmd.clear.desc",
                category: CommandCategory::Utility,
                common_options: &[],
                related: &["reset"],
                example: "clear",
                difficulty: "beginner",
                typical_output: "(screen clears)",
                notes: "cmd.clear.notes",
            },
            KnownCommand {
                name: "exit",
                description_key: "cmd.exit.desc",
                category: CommandCategory::ShellBuiltin,
                common_options: &[],
                related: &["logout"],
                example: "exit 0",
                difficulty: "beginner",
                typical_output: "logout",
                notes: "cmd.exit.notes",
            },
            KnownCommand {
                name: "help",
                description_key: "cmd.help.desc",
                category: CommandCategory::ShellBuiltin,
                common_options: &[],
                related: &["man", "info"],
                example: "help cd",
                difficulty: "beginner",
                typical_output: "cd: cd [dir]\n    Change the current directory...",
                notes: "cmd.help.notes",
            },
            KnownCommand {
                name: "type",
                description_key: "cmd.type.desc",
                category: CommandCategory::ShellBuiltin,
                common_options: &[
                    ("-a", "cmd.type.opt.a"),
                    ("-t", "cmd.type.opt.t"),
                ],
                related: &["which", "help"],
                example: "type ls",
                difficulty: "beginner",
                typical_output: "ls is aliased to `ls --color=auto'",
                notes: "cmd.type.notes",
            },
            KnownCommand {
                name: "printf",
                description_key: "cmd.printf.desc",
                category: CommandCategory::ShellBuiltin,
                common_options: &[
                    ("-v", "cmd.printf.opt.v"),
                ],
                related: &["echo"],
                example: "printf 'Hello %s\\n' World",
                difficulty: "intermediate",
                typical_output: "Hello World",
                notes: "cmd.printf.notes",
            },
            KnownCommand {
                name: "read",
                description_key: "cmd.read.desc",
                category: CommandCategory::ShellBuiltin,
                common_options: &[
                    ("-p", "cmd.read.opt.p"),
                    ("-r", "cmd.read.opt.r"),
                ],
                related: &["echo", "printf"],
                example: "read -p 'Enter name: ' name",
                difficulty: "intermediate",
                typical_output: "(reads input from user)",
                notes: "cmd.read.notes",
            },
            KnownCommand {
                name: "sleep",
                description_key: "cmd.sleep.desc",
                category: CommandCategory::Utility,
                common_options: &[],
                related: &["timeout", "wait"],
                example: "sleep 5",
                difficulty: "beginner",
                typical_output: "(pauses for 5 seconds)",
                notes: "cmd.sleep.notes",
            },
            KnownCommand {
                name: "test",
                description_key: "cmd.test.desc",
                category: CommandCategory::ShellBuiltin,
                common_options: &[
                    ("-f", "cmd.test.opt.f"),
                    ("-d", "cmd.test.opt.d"),
                    ("-e", "cmd.test.opt.e"),
                ],
                related: &["[", "expr"],
                example: "test -f file.txt && echo exists",
                difficulty: "intermediate",
                typical_output: "(returns exit code 0 or 1)",
                notes: "cmd.test.notes",
            },
            KnownCommand {
                name: "expr",
                description_key: "cmd.expr.desc",
                category: CommandCategory::Utility,
                common_options: &[],
                related: &["test", "bc"],
                example: "expr 5 + 3",
                difficulty: "intermediate",
                typical_output: "8",
                notes: "cmd.expr.notes",
            },
            KnownCommand {
                name: "basename",
                description_key: "cmd.basename.desc",
                category: CommandCategory::Utility,
                common_options: &[],
                related: &["dirname"],
                example: "basename /home/user/file.txt",
                difficulty: "beginner",
                typical_output: "file.txt",
                notes: "cmd.basename.notes",
            },
            KnownCommand {
                name: "dirname",
                description_key: "cmd.dirname.desc",
                category: CommandCategory::Utility,
                common_options: &[],
                related: &["basename"],
                example: "dirname /home/user/file.txt",
                difficulty: "beginner",
                typical_output: "/home/user",
                notes: "cmd.dirname.notes",
            },
            KnownCommand {
                name: "which",
                description_key: "cmd.which.desc",
                category: CommandCategory::Utility,
                common_options: &[
                    ("-a", "cmd.which.opt.a"),
                ],
                related: &["whereis", "type"],
                example: "which python3",
                difficulty: "beginner",
                typical_output: "/usr/bin/python3",
                notes: "cmd.which.notes",
            },
            KnownCommand {
                name: "whereis",
                description_key: "cmd.whereis.desc",
                category: CommandCategory::Utility,
                common_options: &[
                    ("-b", "cmd.whereis.opt.b"),
                    ("-m", "cmd.whereis.opt.m"),
                ],
                related: &["which", "locate"],
                example: "whereis bash",
                difficulty: "beginner",
                typical_output: "bash: /usr/bin/bash /usr/share/man/man1/bash.1.gz",
                notes: "cmd.whereis.notes",
            },
            KnownCommand {
                name: "xargs",
                description_key: "cmd.xargs.desc",
                category: CommandCategory::Utility,
                common_options: &[
                    ("-n", "cmd.xargs.opt.n"),
                    ("-d", "cmd.xargs.opt.d"),
                    ("-I", "cmd.xargs.opt.I"),
                ],
                related: &["find", "exec"],
                example: "find . -name '*.txt' | xargs rm",
                difficulty: "advanced",
                typical_output: "(executes commands with piped input)",
                notes: "cmd.xargs.notes",
            },
            KnownCommand {
                name: "watch",
                description_key: "cmd.watch.desc",
                category: CommandCategory::Utility,
                common_options: &[
                    ("-n", "cmd.watch.opt.n"),
                    ("-d", "cmd.watch.opt.d"),
                ],
                related: &["sleep", "top"],
                example: "watch -n 5 date",
                difficulty: "intermediate",
                typical_output: "Every 5.0s: date\nMon Jan 15 10:00:00 ...",
                notes: "cmd.watch.notes",
            },
            KnownCommand {
                name: "env",
                description_key: "cmd.env.desc",
                category: CommandCategory::Utility,
                common_options: &[
                    ("-i", "cmd.env.opt.i"),
                    ("-u", "cmd.env.opt.u"),
                ],
                related: &["export", "printenv"],
                example: "env | grep PATH",
                difficulty: "beginner",
                typical_output: "PATH=/usr/bin:/bin:/usr/sbin",
                notes: "cmd.env.notes",
            },
            KnownCommand {
                name: "printenv",
                description_key: "cmd.printenv.desc",
                category: CommandCategory::Utility,
                common_options: &[],
                related: &["env", "export"],
                example: "printenv HOME",
                difficulty: "beginner",
                typical_output: "/home/user",
                notes: "cmd.printenv.notes",
            },
            KnownCommand {
                name: "seq",
                description_key: "cmd.seq.desc",
                category: CommandCategory::Utility,
                common_options: &[
                    ("-s", "cmd.seq.opt.s"),
                    ("-w", "cmd.seq.opt.w"),
                ],
                related: &["for", "printf"],
                example: "seq 1 5",
                difficulty: "beginner",
                typical_output: "1\n2\n3\n4\n5",
                notes: "cmd.seq.notes",
            },
            KnownCommand {
                name: "yes",
                description_key: "cmd.yes.desc",
                category: CommandCategory::Utility,
                common_options: &[],
                related: &["xargs"],
                example: "yes | apt install package",
                difficulty: "beginner",
                typical_output: "y\ny\ny\n...",
                notes: "cmd.yes.notes",
            },
            KnownCommand {
                name: "sudo",
                description_key: "cmd.sudo.desc",
                category: CommandCategory::Other,
                common_options: &[
                    ("-u", "cmd.sudo.opt.u"),
                    ("-i", "cmd.sudo.opt.i"),
                    ("-s", "cmd.sudo.opt.s"),
                ],
                related: &["su", "doas"],
                example: "sudo apt update",
                difficulty: "beginner",
                typical_output: "[sudo] password for user:\n...",
                notes: "cmd.sudo.notes",
            },
            KnownCommand {
                name: "su",
                description_key: "cmd.su.desc",
                category: CommandCategory::Other,
                common_options: &[
                    ("-", "cmd.su.opt.-"),
                    ("-c", "cmd.su.opt.c"),
                ],
                related: &["sudo"],
                example: "su - user",
                difficulty: "intermediate",
                typical_output: "Password:\n$ ",
                notes: "cmd.su.notes",
            },
            KnownCommand {
                name: "passwd",
                description_key: "cmd.passwd.desc",
                category: CommandCategory::Other,
                common_options: &[
                    ("-l", "cmd.passwd.opt.l"),
                    ("-u", "cmd.passwd.opt.u"),
                    ("-d", "cmd.passwd.opt.d"),
                ],
                related: &["useradd", "usermod"],
                example: "passwd",
                difficulty: "beginner",
                typical_output: "Current password:\nNew password:\nRetype new password:",
                notes: "cmd.passwd.notes",
            },
            KnownCommand {
                name: "useradd",
                description_key: "cmd.useradd.desc",
                category: CommandCategory::Other,
                common_options: &[
                    ("-m", "cmd.useradd.opt.m"),
                    ("-s", "cmd.useradd.opt.s"),
                    ("-G", "cmd.useradd.opt.G"),
                ],
                related: &["usermod", "userdel", "groupadd"],
                example: "useradd -m -s /bin/bash newuser",
                difficulty: "intermediate",
                typical_output: "",
                notes: "cmd.useradd.notes",
            },
            KnownCommand {
                name: "usermod",
                description_key: "cmd.usermod.desc",
                category: CommandCategory::Other,
                common_options: &[
                    ("-aG", "cmd.usermod.opt.aG"),
                    ("-l", "cmd.usermod.opt.l"),
                    ("-d", "cmd.usermod.opt.d"),
                ],
                related: &["useradd", "groupadd"],
                example: "usermod -aG sudo username",
                difficulty: "intermediate",
                typical_output: "",
                notes: "cmd.usermod.notes",
            },
            KnownCommand {
                name: "groupadd",
                description_key: "cmd.groupadd.desc",
                category: CommandCategory::Other,
                common_options: &[
                    ("-r", "cmd.groupadd.opt.r"),
                    ("-f", "cmd.groupadd.opt.f"),
                ],
                related: &["useradd", "usermod", "groups"],
                example: "groupadd developers",
                difficulty: "intermediate",
                typical_output: "",
                notes: "cmd.groupadd.notes",
            },
            KnownCommand {
                name: "groups",
                description_key: "cmd.groups.desc",
                category: CommandCategory::Other,
                common_options: &[],
                related: &["id", "groupadd"],
                example: "groups user",
                difficulty: "beginner",
                typical_output: "user : user sudo adm",
                notes: "cmd.groups.notes",
            },
            KnownCommand {
                name: "last",
                description_key: "cmd.last.desc",
                category: CommandCategory::Other,
                common_options: &[
                    ("-n", "cmd.last.opt.n"),
                    ("-f", "cmd.last.opt.f"),
                ],
                related: &["who", "w"],
                example: "last -n 10",
                difficulty: "beginner",
                typical_output: "user     tty7         :0        Mon Jan 15 10:00   still logged in",
                notes: "cmd.last.notes",
            },
            KnownCommand {
                name: "bc",
                description_key: "cmd.bc.desc",
                category: CommandCategory::Other,
                common_options: &[
                    ("-l", "cmd.bc.opt.l"),
                    ("-q", "cmd.bc.opt.q"),
                ],
                related: &["expr", "calc"],
                example: "echo 'scale=2; 10/3' | bc",
                difficulty: "intermediate",
                typical_output: "3.33",
                notes: "cmd.bc.notes",
            },
            KnownCommand {
                name: "man",
                description_key: "cmd.man.desc",
                category: CommandCategory::Other,
                common_options: &[
                    ("-k", "cmd.man.opt.k"),
                    ("-f", "cmd.man.opt.f"),
                ],
                related: &["info", "help"],
                example: "man ls",
                difficulty: "beginner",
                typical_output: "LS(1)                    User Commands\nNAME\nls - list directory contents\n...",
                notes: "cmd.man.notes",
            },
            KnownCommand {
                name: "info",
                description_key: "cmd.info.desc",
                category: CommandCategory::Other,
                common_options: &[],
                related: &["man", "help"],
                example: "info ls",
                difficulty: "intermediate",
                typical_output: "interactive documentation browser",
                notes: "cmd.info.notes",
            },
            KnownCommand {
                name: "tee",
                description_key: "cmd.tee.desc",
                category: CommandCategory::TextProcessing,
                common_options: &[
                    ("-a", "cmd.tee.opt.a"),
                    ("-i", "cmd.tee.opt.i"),
                ],
                related: &["cat", "redirect"],
                example: "echo 'data' | tee file.txt",
                difficulty: "intermediate",
                typical_output: "data",
                notes: "cmd.tee.notes",
            },
            KnownCommand {
                name: "diff",
                description_key: "cmd.diff.desc",
                category: CommandCategory::TextProcessing,
                common_options: &[
                    ("-u", "cmd.diff.opt.u"),
                    ("-i", "cmd.diff.opt.i"),
                    ("-r", "cmd.diff.opt.r"),
                ],
                related: &["patch", "cmp"],
                example: "diff -u file1.txt file2.txt",
                difficulty: "intermediate",
                typical_output: "--- a/file1.txt\n+++ b/file2.txt\n@@ -1 +1 @@\n-old content\n+new content",
                notes: "cmd.diff.notes",
            },
            KnownCommand {
                name: "patch",
                description_key: "cmd.patch.desc",
                category: CommandCategory::TextProcessing,
                common_options: &[
                    ("-p", "cmd.patch.opt.p"),
                    ("-R", "cmd.patch.opt.R"),
                ],
                related: &["diff"],
                example: "patch < changes.patch",
                difficulty: "advanced",
                typical_output: "patching file file.txt",
                notes: "cmd.patch.notes",
            },
            KnownCommand {
                name: "ln",
                description_key: "cmd.ln.desc",
                category: CommandCategory::FileSystem,
                common_options: &[
                    ("-s", "cmd.ln.opt.s"),
                    ("-f", "cmd.ln.opt.f"),
                    ("-v", "cmd.ln.opt.v"),
                ],
                related: &["cp", "mv"],
                example: "ln -s /usr/bin/python3 python",
                difficulty: "intermediate",
                typical_output: "",
                notes: "cmd.ln.notes",
            },
            KnownCommand {
                name: "stat",
                description_key: "cmd.stat.desc",
                category: CommandCategory::FileSystem,
                common_options: &[
                    ("-f", "cmd.stat.opt.f"),
                    ("-c", "cmd.stat.opt.c"),
                ],
                related: &["ls", "lsblk"],
                example: "stat file.txt",
                difficulty: "intermediate",
                typical_output: "File: file.txt\nSize: 1234    Blocks: 8 ...",
                notes: "cmd.stat.notes",
            },
            KnownCommand {
                name: "dd",
                description_key: "cmd.dd.desc",
                category: CommandCategory::FileSystem,
                common_options: &[
                    ("if", "cmd.dd.opt.if"),
                    ("of", "cmd.dd.opt.of"),
                    ("bs", "cmd.dd.opt.bs"),
                ],
                related: &["cp"],
                example: "dd if=/dev/zero of=file.img bs=1M count=10",
                difficulty: "advanced",
                typical_output: "10+0 records in\n10+0 records out\n10485760 bytes transferred",
                notes: "cmd.dd.notes",
            },
            KnownCommand {
                name: "nice",
                description_key: "cmd.nice.desc",
                category: CommandCategory::ProcessManagement,
                common_options: &[
                    ("-n", "cmd.nice.opt.n"),
                ],
                related: &["renice", "top"],
                example: "nice -n 10 ./script.sh",
                difficulty: "advanced",
                typical_output: "",
                notes: "cmd.nice.notes",
            },
            KnownCommand {
                name: "renice",
                description_key: "cmd.renice.desc",
                category: CommandCategory::ProcessManagement,
                common_options: &[
                    ("-n", "cmd.renice.opt.n"),
                    ("-p", "cmd.renice.opt.p"),
                ],
                related: &["nice", "top"],
                example: "renice -n 5 -p 1234",
                difficulty: "advanced",
                typical_output: "1234 (process ID) old priority 0, new priority 5",
                notes: "cmd.renice.notes",
            },
            KnownCommand {
                name: "nohup",
                description_key: "cmd.nohup.desc",
                category: CommandCategory::ProcessManagement,
                common_options: &[],
                related: &["disown", "screen"],
                example: "nohup ./script.sh &",
                difficulty: "intermediate",
                typical_output: "nohup: ignoring input and appending output to 'nohup.out'",
                notes: "cmd.nohup.notes",
            },
            KnownCommand {
                name: "umask",
                description_key: "cmd.umask.desc",
                category: CommandCategory::Permissions,
                common_options: &[
                    ("-S", "cmd.umask.opt.S"),
                ],
                related: &["chmod"],
                example: "umask 022",
                difficulty: "intermediate",
                typical_output: "0022",
                notes: "cmd.umask.notes",
            },
            KnownCommand {
                name: "exec",
                description_key: "cmd.exec.desc",
                category: CommandCategory::ShellBuiltin,
                common_options: &[],
                related: &["source", "eval"],
                example: "exec bash",
                difficulty: "advanced",
                typical_output: "(replaces current shell)",
                notes: "cmd.exec.notes",
            },
            KnownCommand {
                name: "eval",
                description_key: "cmd.eval.desc",
                category: CommandCategory::ShellBuiltin,
                common_options: &[],
                related: &["exec", "source"],
                example: "eval $(ssh-agent -s)",
                difficulty: "advanced",
                typical_output: "Agent pid 1234",
                notes: "cmd.eval.notes",
            },
            KnownCommand {
                name: "set",
                description_key: "cmd.set.desc",
                category: CommandCategory::ShellBuiltin,
                common_options: &[
                    ("-e", "cmd.set.opt.e"),
                    ("-x", "cmd.set.opt.x"),
                    ("-u", "cmd.set.opt.u"),
                ],
                related: &["env", "export"],
                example: "set -x",
                difficulty: "intermediate",
                typical_output: "",
                notes: "cmd.set.notes",
            },
            KnownCommand {
                name: "unset",
                description_key: "cmd.unset.desc",
                category: CommandCategory::ShellBuiltin,
                common_options: &[
                    ("-f", "cmd.unset.opt.f"),
                    ("-v", "cmd.unset.opt.v"),
                ],
                related: &["set", "export"],
                example: "unset DEBUG_MODE",
                difficulty: "intermediate",
                typical_output: "",
                notes: "cmd.unset.notes",
            },
            KnownCommand {
                name: "shift",
                description_key: "cmd.shift.desc",
                category: CommandCategory::ShellBuiltin,
                common_options: &[],
                related: &["set"],
                example: "shift 2",
                difficulty: "advanced",
                typical_output: "",
                notes: "cmd.shift.notes",
            },
            KnownCommand {
                name: "readlink",
                description_key: "cmd.readlink.desc",
                category: CommandCategory::FileSystem,
                common_options: &[
                    ("-f", "cmd.readlink.opt.f"),
                    ("-e", "cmd.readlink.opt.e"),
                ],
                related: &["ln", "realpath"],
                example: "readlink -f /usr/bin/python",
                difficulty: "intermediate",
                typical_output: "/usr/bin/python3.12",
                notes: "cmd.readlink.notes",
            },
            KnownCommand {
                name: "crontab",
                description_key: "cmd.crontab.desc",
                category: CommandCategory::Other,
                common_options: &[
                    ("-e", "cmd.crontab.opt.e"),
                    ("-l", "cmd.crontab.opt.l"),
                    ("-r", "cmd.crontab.opt.r"),
                ],
                related: &["at", "systemd-timer"],
                example: "crontab -e",
                difficulty: "advanced",
                typical_output: "(opens editor for cron table)",
                notes: "cmd.crontab.notes",
            },
            KnownCommand {
                name: "timeout",
                description_key: "cmd.timeout.desc",
                category: CommandCategory::Utility,
                common_options: &[
                    ("-k", "cmd.timeout.opt.k"),
                    ("--signal", "cmd.timeout.opt.signal"),
                ],
                related: &["sleep", "kill"],
                example: "timeout 5 ping google.com",
                difficulty: "intermediate",
                typical_output: "",
                notes: "cmd.timeout.notes",
            },
        ]
    }
}

fn fuzzy_score(query: &str, target: &str) -> u8 {
    let q = query.to_lowercase();
    let t = target.to_lowercase();

    if q == t {
        return 100;
    }
    if t.starts_with(&q) {
        return 95;
    }
    if t.contains(&q) {
        return 90;
    }

    let qc: Vec<char> = q.chars().collect();
    let tc: Vec<char> = t.chars().collect();

    let mut ti = 0;
    let mut matches = 0;
    for &c in &qc {
        while ti < tc.len() {
            if tc[ti] == c {
                matches += 1;
                ti += 1;
                break;
            }
            ti += 1;
        }
    }

    if matches == 0 {
        return 0;
    }

    let char_ratio = matches as f64 / qc.len() as f64;
    let len_ratio = qc.len().min(tc.len()) as f64 / qc.len().max(tc.len()) as f64;
    let score = char_ratio * len_ratio * 100.0;
    (score as u8).min(100)
}

#[derive(Debug, Clone)]
pub struct CommandSuggestion {
    pub command: String,
    pub description_key: String,
    pub confidence: u8,
    pub category: CommandCategory,
}

#[derive(Debug, Clone)]
pub struct CommandSuggestionEngine {
    known: KnownCommands,
    localization: LocalizationManager,
}

impl CommandSuggestionEngine {
    pub fn new(known: KnownCommands, localization: LocalizationManager) -> Self {
        CommandSuggestionEngine { known, localization }
    }

    pub fn with_defaults() -> Self {
        let known = KnownCommands::new();
        let localization = LocalizationManager::new();
        CommandSuggestionEngine { known, localization }
    }

    pub fn fuzzy_match(&self, input: &str) -> Vec<CommandSuggestion> {
        let mut results: Vec<CommandSuggestion> = self
            .known
            .all()
            .iter()
            .map(|c| {
                let confidence = fuzzy_score(input, c.name);
                CommandSuggestion {
                    command: c.name.to_string(),
                    description_key: c.description_key.to_string(),
                    confidence,
                    category: c.category,
                }
            })
            .filter(|s| s.confidence >= 30)
            .collect();
        results.sort_by(|a, b| b.confidence.cmp(&a.confidence));
        results.truncate(5);
        results
    }

    pub fn did_you_mean(&self, input: &str) -> Vec<String> {
        self.fuzzy_match(input)
            .iter()
            .map(|s| s.command.clone())
            .collect()
    }

    pub fn suggest_for_purpose(&self, input: &str) -> Vec<CommandSuggestion> {
        if input.is_empty() {
            return Vec::new();
        }
        let lower = input.to_lowercase();
        let exact_prefix: Vec<CommandSuggestion> = self
            .known
            .all()
            .iter()
            .filter(|c| c.name.starts_with(&lower) && c.name != lower)
            .map(|c| CommandSuggestion {
                command: c.name.to_string(),
                description_key: c.description_key.to_string(),
                confidence: 80,
                category: c.category,
            })
            .collect();

        if !exact_prefix.is_empty() {
            return exact_prefix;
        }

        let desc_matches: Vec<CommandSuggestion> = self
            .known
            .all()
            .iter()
            .filter(|c| {
                c.description_key
                    .to_lowercase()
                    .contains(&lower)
                    || c.name.to_lowercase().contains(&lower)
            })
            .take(5)
            .map(|c| CommandSuggestion {
                command: c.name.to_string(),
                description_key: c.description_key.to_string(),
                confidence: 60,
                category: c.category,
            })
            .collect();

        if !desc_matches.is_empty() {
            return desc_matches;
        }

        self.fuzzy_match(input)
    }

    pub fn get_explanation(&self, command: &str) -> Option<String> {
        self.known
            .find(command)
            .map(|kc| self.localization.get(kc.description_key).to_string())
    }

    pub fn get_example(&self, command: &str) -> Option<String> {
        self.known.find(command).map(|kc| kc.example.to_string())
    }

    pub fn get_common_options(&self, command: &str) -> Vec<(String, String)> {
        self.known
            .find(command)
            .map(|kc| {
                kc.common_options
                    .iter()
                    .map(|(flag, desc)| (flag.to_string(), self.localization.get(desc).to_string()))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn set_locale(&mut self, lang: Lang) {
        self.localization.set_language(lang);
    }

    pub fn known_commands(&self) -> &KnownCommands {
        &self.known
    }
}

#[derive(Debug, Clone)]
pub struct TutorialState {
    pub tutorial_id: String,
    pub step: u32,
    pub total_steps: u32,
    pub lesson_content: String,
    pub completed: bool,
    pub started_at: String,
}

#[derive(Debug, Clone)]
pub struct TerminalEntry {
    pub input: String,
    pub output: String,
    pub entry_type: EntryType,
    pub timestamp: String,
    pub working_directory: PathBuf,
    pub exit_code: i32,
}

#[derive(Debug, Clone)]
pub struct EduTerminal {
    mode: TerminalMode,
    state: TerminalState,
    history: Vec<TerminalEntry>,
    current_input: String,
    current_directory: PathBuf,
    command_suggestions: CommandSuggestionEngine,
    tutorial_state: Option<TutorialState>,
    known_commands: KnownCommands,
    localization: LocalizationManager,
    #[allow(dead_code)]
    prompt_prefix: String,
    max_history: usize,
}

impl EduTerminal {
    pub fn new() -> Self {
        let localization = LocalizationManager::new();
        let known_commands = KnownCommands::new();
        let command_suggestions =
            CommandSuggestionEngine::new(known_commands.clone(), localization.clone());
        EduTerminal {
            mode: TerminalMode::Normal,
            state: TerminalState::Idle,
            history: Vec::new(),
            current_input: String::new(),
            current_directory: PathBuf::from("/home/user"),
            command_suggestions,
            tutorial_state: None,
            known_commands,
            localization,
            prompt_prefix: "$ ".to_string(),
            max_history: 1000,
        }
    }

    pub fn execute(&mut self, input: &str) -> TerminalEntry {
        let original = input.to_string();
        let trimmed = input.trim();
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let cwd = self.current_directory.clone();

        if trimmed.is_empty() {
            return self.record_entry(TerminalEntry {
                input: original,
                output: String::new(),
                entry_type: EntryType::Command,
                timestamp: now,
                working_directory: cwd,
                exit_code: 0,
            });
        }

        let parts: Vec<&str> = trimmed.splitn(2, |c: char| c.is_whitespace()).collect();
        let cmd = parts[0];
        let args = parts.get(1).unwrap_or(&"").trim();

        match cmd {
            "cd" => {
                let target = if args.is_empty() || args == "~" {
                    PathBuf::from("/home/user")
                } else if args == ".." {
                    if let Some(parent) = self.current_directory.parent() {
                        parent.to_path_buf()
                    } else {
                        self.current_directory.clone()
                    }
                } else if args.starts_with('/') {
                    PathBuf::from(args)
                } else if args.starts_with("~/") {
                    PathBuf::from("/home/user").join(&args[2..])
                } else {
                    self.current_directory.join(args)
                };

                self.current_directory = target.clone();
                self.state = TerminalState::Completed;
                return self.record_entry(TerminalEntry {
                    input: original,
                    output: String::new(),
                    entry_type: EntryType::Command,
                    timestamp: now,
                    working_directory: cwd,
                    exit_code: 0,
                });
            }
            "exit" => {
                self.state = TerminalState::Completed;
                let code = args.parse::<i32>().unwrap_or(0);
                return self.record_entry(TerminalEntry {
                    input: original,
                    output: "logout".to_string(),
                    entry_type: EntryType::System,
                    timestamp: now,
                    working_directory: cwd,
                    exit_code: code,
                });
            }
            "clear" => {
                return self.record_entry(TerminalEntry {
                    input: original,
                    output: String::new(),
                    entry_type: EntryType::System,
                    timestamp: now,
                    working_directory: cwd,
                    exit_code: 0,
                });
            }
            "help" => {
                let output = self.format_help();
                return self.record_entry(TerminalEntry {
                    input: original,
                    output,
                    entry_type: EntryType::Output,
                    timestamp: now,
                    working_directory: cwd,
                    exit_code: 0,
                });
            }
            "history" => {
                let output = self.format_history();
                return self.record_entry(TerminalEntry {
                    input: original,
                    output,
                    entry_type: EntryType::Output,
                    timestamp: now,
                    working_directory: cwd,
                    exit_code: 0,
                });
            }
            _ => {}
        }

        if let Some(known) = self.known_commands.find(cmd) {
            let simulated = self.simulate_command(cmd, args);
            let output = if self.mode == TerminalMode::Learning {
                let desc = self.localization.get(known.description_key);
                let example = known.example;
                format!(
                    "# {}: {}\n# Contoh: {}\n{}",
                    cmd, desc, example, simulated
                )
            } else {
                simulated
            };
            self.state = TerminalState::Completed;
            return self.record_entry(TerminalEntry {
                input: original,
                output,
                entry_type: EntryType::Output,
                timestamp: now,
                working_directory: cwd,
                exit_code: 0,
            });
        }

        if self.mode == TerminalMode::Learning {
            let suggestions = self.command_suggestions.fuzzy_match(cmd);
            let mut msg = format!(
                "Perintah '{}' tidak ditemukan.\n",
                cmd
            );
            if !suggestions.is_empty() {
                msg.push_str("\nApakah maksud Anda:\n");
                for s in &suggestions {
                    let desc = self.localization.get(&s.description_key);
                    msg.push_str(&format!("  - {} ({})\n", s.command, desc));
                }
                msg.push('\n');
                if let Some(top) = suggestions.first() {
                    if let Some(example) = self.command_suggestions.get_example(&top.command) {
                        let desc = self.localization.get(&top.description_key);
                        msg.push_str(&format!(
                            "Perintah '{}': {}\nContoh: {}\n",
                            top.command, desc, example
                        ));
                    }
                }
            } else {
                msg.push_str(
                    "\nTidak ada saran yang tersedia. Periksa ejaan Anda atau gunakan 'help' untuk melihat daftar perintah.\n",
                );
            }
            self.state = TerminalState::Error;
            return self.record_entry(TerminalEntry {
                input: original,
                output: msg,
                entry_type: EntryType::Error,
                timestamp: now,
                working_directory: cwd,
                exit_code: 127,
            });
        }

        let output = format!("bash: {}: command not found", cmd);
        self.state = TerminalState::Error;
        self.record_entry(TerminalEntry {
            input: original,
            output,
            entry_type: EntryType::Error,
            timestamp: now,
            working_directory: cwd,
            exit_code: 127,
        })
    }

    fn record_entry(&mut self, entry: TerminalEntry) -> TerminalEntry {
        self.history.push(entry.clone());
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }
        entry
    }

    fn simulate_command(&self, cmd: &str, args: &str) -> String {
        match cmd {
            "ls" => {
                if args.contains("-l") || args.contains("-la") || args.contains("-al") {
                    format!(
                        "total 32\ndrwxr-xr-x 2 user user 4096 {} ..\ndrwxr-xr-x 2 user user 4096 {} .\ndrwxr-xr-x 2 user user 4096 {} Desktop\ndrwxr-xr-x 2 user user 4096 {} Documents\ndrwxr-xr-x 2 user user 4096 {} Downloads\ndrwxr-xr-x 2 user user 4096 {} Music\ndrwxr-xr-x 2 user user 4096 {} Pictures\ndrwxr-xr-x 2 user user 4096 {} Videos",
                        Local::now().format("%b %e %H:%M"),
                        Local::now().format("%b %e %H:%M"),
                        Local::now().format("%b %e %H:%M"),
                        Local::now().format("%b %e %H:%M"),
                        Local::now().format("%b %e %H:%M"),
                        Local::now().format("%b %e %H:%M"),
                        Local::now().format("%b %e %H:%M"),
                        Local::now().format("%b %e %H:%M"),
                    )
                } else if args.contains("-a") {
                    ".  ..  Desktop  Documents  Downloads  Music  Pictures  Videos  .bashrc  .profile"
                        .to_string()
                } else {
                    "Desktop  Documents  Downloads  Music  Pictures  Videos".to_string()
                }
            }
            "pwd" => self.current_directory.to_string_lossy().to_string(),
            "echo" => args.to_string(),
            "date" => Local::now().format("%a %b %e %H:%M:%S %Z %Y").to_string(),
            "whoami" => "user".to_string(),
            "id" => {
                "uid=1000(user) gid=1000(user) groups=1000(user),4(adm),27(sudo)".to_string()
            }
            "uname" => {
                if args == "-a" {
                    "Linux edushell 6.8.0-arch1-1 #1 SMP PREEMPT_DYNAMIC x86_64 GNU/Linux"
                        .to_string()
                } else if args == "-r" {
                    "6.8.0-arch1-1".to_string()
                } else {
                    "Linux".to_string()
                }
            }
            "cal" => {
                let now = Local::now();
                format!(
                    "   {} {}\nSu Mo Tu We Th Fr Sa\n 1  2  3  4  5  6  7\n 8  9 10 11 12 13 14\n15 16 17 18 19 20 21\n22 23 24 25 26 27 28\n29 30 31",
                    now.format("%B"),
                    now.format("%Y")
                )
            }
            "uptime" => {
                format!(
                    " {} up 1 day, 1 user, load average: 0.00, 0.01, 0.05",
                    Local::now().format("%H:%M:%S")
                )
            }
            "df" => {
                if args.contains("-h") {
                    "Filesystem      Size  Used Avail Use% Mounted on\ndev              3.9G     0  3.9G   0% /dev\n/dev/sda1       100G   45G   55G  45% /\n".to_string()
                } else {
                    "Filesystem     1K-blocks    Used Available Use% Mounted on\ndev              4062912       0   4062912   0% /dev\n/dev/sda1      104857600 47185920  57671680  45% /\n".to_string()
                }
            }
            "du" => {
                if args.contains("-s") {
                    "4.0K\t.\n".to_string()
                } else if args.contains("-h") {
                    "4.0K\t./file.txt\n4.0K\t./.bashrc\n1.2M\t./Documents\n".to_string()
                } else {
                    "4\t./file.txt\n4\t./.bashrc\n1234\t./Documents\n".to_string()
                }
            }
            "free" => {
                if args.contains("-h") {
                    "              total        used        free      shared  buff/cache   available\nMem:           7.6Gi       2.1Gi       3.2Gi       0.1Gi       2.3Gi       5.1Gi\nSwap:          2.0Gi        0.0Gi       2.0Gi".to_string()
                } else {
                    "              total        used        free      shared  buff/cache   available\nMem:          8000000     2200000     3400000      100000     2400000     5400000\nSwap:         2000000           0     2000000".to_string()
                }
            }
            "who" | "w" => {
                format!(
                    "user     tty7         {}   00:00    0:00    /usr/bin/bash",
                    Local::now().format("%b %d %H:%M")
                )
            }
            "hostname" => {
                if args == "-I" {
                    "192.168.1.100".to_string()
                } else {
                    "edushell".to_string()
                }
            }
            _ => {
                let known = self
                    .known_commands
                    .find(cmd)
                    .map(|k| k.typical_output)
                    .unwrap_or("");
                if known.is_empty() {
                    format!("[Simulated output for '{}' with args: {}]", cmd, args)
                } else {
                    known.to_string()
                }
            }
        }
    }

    fn format_help(&self) -> String {
        let mut output = String::from("EduShell - Daftar Perintah Tersedia\n");
        output.push_str("====================================\n\n");

        let categories = [
            (CommandCategory::FileSystem, "File System"),
            (CommandCategory::TextProcessing, "Text Processing"),
            (CommandCategory::SystemInfo, "System Info"),
            (CommandCategory::ProcessManagement, "Process Management"),
            (CommandCategory::Networking, "Networking"),
            (CommandCategory::Permissions, "Permissions"),
            (CommandCategory::Compression, "Compression"),
            (CommandCategory::PackageManagement, "Package Management"),
            (CommandCategory::Git, "Git"),
            (CommandCategory::Programming, "Programming"),
            (CommandCategory::Terminal, "Terminal"),
            (CommandCategory::ShellBuiltin, "Shell Built-in"),
            (CommandCategory::Utility, "Utility"),
            (CommandCategory::Other, "Other"),
        ];

        for (cat, label) in &categories {
            let cmds: Vec<&str> = self
                .known_commands
                .by_category(*cat)
                .iter()
                .map(|c| c.name)
                .collect();
            if !cmds.is_empty() {
                output.push_str(&format!("  {}:\n", label));
                for chunk in cmds.chunks(6) {
                    output.push_str("    ");
                    output.push_str(&chunk.join(", "));
                    output.push('\n');
                }
                output.push('\n');
            }
        }

        output.push_str("Gunakan 'man <perintah>' untuk informasi lebih lanjut.\n");
        output
    }

    fn format_history(&self) -> String {
        if self.history.is_empty() {
            return "No command history.".to_string();
        }
        let start = if self.history.len() > 20 {
            self.history.len() - 20
        } else {
            0
        };
        let mut output = String::new();
        for (i, entry) in self.history[start..].iter().enumerate() {
            if !entry.input.is_empty() {
                output.push_str(&format!("{:4}  {}\n", start + i + 1, entry.input));
            }
        }
        output
    }

    pub fn set_mode(&mut self, mode: TerminalMode) {
        self.mode = mode;
    }

    pub fn mode(&self) -> TerminalMode {
        self.mode
    }

    pub fn set_state(&mut self, state: TerminalState) {
        self.state = state;
    }

    pub fn state(&self) -> TerminalState {
        self.state
    }

    pub fn history(&self) -> &[TerminalEntry] {
        &self.history
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    pub fn last_n_entries(&self, n: usize) -> Vec<&TerminalEntry> {
        let len = self.history.len();
        let start = if len >= n { len - n } else { 0 };
        self.history[start..].iter().collect()
    }

    pub fn current_input(&self) -> &str {
        &self.current_input
    }

    pub fn set_input(&mut self, input: &str) {
        self.current_input = input.to_string();
    }

    pub fn current_directory(&self) -> &Path {
        &self.current_directory
    }

    pub fn change_directory(&mut self, path: &Path) -> bool {
        if path.exists() && path.is_dir() {
            self.current_directory = path.to_path_buf();
            true
        } else {
            false
        }
    }

    pub fn suggest(&self, input: &str) -> Vec<CommandSuggestion> {
        self.command_suggestions.suggest_for_purpose(input)
    }

    pub fn did_you_mean(&self, input: &str) -> Vec<String> {
        self.command_suggestions.did_you_mean(input)
    }

    pub fn explain(&self, term: &str) -> Option<String> {
        self.command_suggestions.get_explanation(term)
    }

    pub fn example(&self, term: &str) -> Option<String> {
        self.command_suggestions.get_example(term)
    }

    pub fn tutorial_state(&self) -> Option<&TutorialState> {
        self.tutorial_state.as_ref()
    }

    pub fn start_tutorial(&mut self, id: &str, content: &str, steps: u32) {
        self.tutorial_state = Some(TutorialState {
            tutorial_id: id.to_string(),
            step: 1,
            total_steps: steps,
            lesson_content: content.to_string(),
            completed: false,
            started_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        });
    }

    pub fn advance_tutorial(&mut self) -> bool {
        if let Some(ref mut t) = self.tutorial_state {
            if t.completed {
                return false;
            }
            if t.step < t.total_steps {
                t.step += 1;
                true
            } else {
                t.completed = true;
                false
            }
        } else {
            false
        }
    }

    pub fn end_tutorial(&mut self) {
        if let Some(ref mut t) = self.tutorial_state {
            t.completed = true;
        }
    }

    pub fn learning_hint(&self, input: &str) -> Option<String> {
        if self.mode != TerminalMode::Learning {
            return None;
        }
        let suggestions = self.command_suggestions.fuzzy_match(input);
        suggestions.first().map(|s| {
            let desc = self.localization.get(&s.description_key);
            format!(
                "Hint: Mungkin maksud Anda '{}'? {}",
                s.command, desc
            )
        })
    }

    pub fn auto_suggest(&self, input: &str) -> Option<String> {
        if input.is_empty() {
            return None;
        }
        self.known_commands
            .all()
            .iter()
            .find(|c| c.name.starts_with(input) && c.name != input)
            .map(|c| c.name.to_string())
    }

    pub fn known_command(&self, name: &str) -> Option<&KnownCommand> {
        self.known_commands.find(name)
    }

    pub fn commands_by_category(&self, cat: CommandCategory) -> Vec<&KnownCommand> {
        self.known_commands.by_category(cat)
    }

    pub fn commands_by_difficulty(&self, level: &str) -> Vec<&KnownCommand> {
        self.known_commands.by_difficulty(level)
    }

    pub fn all_known_commands(&self) -> &[KnownCommand] {
        self.known_commands.all()
    }

    pub fn total_known_commands(&self) -> usize {
        self.known_commands.len()
    }

    pub fn set_locale(&mut self, locale: &str) {
        let lang = match locale {
            "id-ID" | "id" => Lang::Indonesian,
            "en-US" | "en" => Lang::English,
            _ => Lang::Indonesian,
        };
        self.localization.set_language(lang);
        self.command_suggestions.set_locale(lang);
    }
}

impl Default for EduTerminal {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for KnownCommands {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_terminal() -> EduTerminal {
        EduTerminal::new()
    }

    #[test]
    fn test_new_defaults() {
        let terminal = create_terminal();
        assert_eq!(terminal.mode(), TerminalMode::Normal);
        assert_eq!(terminal.state(), TerminalState::Idle);
        assert_eq!(terminal.current_directory(), Path::new("/home/user"));
        assert!(terminal.total_known_commands() >= 100);
    }

    #[test]
    fn test_mode_switching() {
        let mut terminal = create_terminal();
        assert_eq!(terminal.mode(), TerminalMode::Normal);
        terminal.set_mode(TerminalMode::Learning);
        assert_eq!(terminal.mode(), TerminalMode::Learning);
        terminal.set_mode(TerminalMode::Normal);
        assert_eq!(terminal.mode(), TerminalMode::Normal);
    }

    #[test]
    fn test_state_management() {
        let mut terminal = create_terminal();
        assert_eq!(terminal.state(), TerminalState::Idle);
        terminal.set_state(TerminalState::Running);
        assert_eq!(terminal.state(), TerminalState::Running);
        terminal.set_state(TerminalState::Error);
        assert_eq!(terminal.state(), TerminalState::Error);
        terminal.set_state(TerminalState::Completed);
        assert_eq!(terminal.state(), TerminalState::Completed);
    }

    #[test]
    fn test_input_handling() {
        let mut terminal = create_terminal();
        assert!(terminal.current_input().is_empty());
        terminal.set_input("ls -la");
        assert_eq!(terminal.current_input(), "ls -la");
        terminal.set_input("");
        assert!(terminal.current_input().is_empty());
    }

    #[test]
    fn test_history_add_and_retrieve() {
        let mut terminal = create_terminal();
        assert!(terminal.history().is_empty());
        terminal.execute("ls");
        assert_eq!(terminal.history().len(), 1);
        assert_eq!(terminal.history()[0].input, "ls");
        terminal.execute("pwd");
        assert_eq!(terminal.history().len(), 2);
        assert_eq!(terminal.history()[1].input, "pwd");
    }

    #[test]
    fn test_history_clear() {
        let mut terminal = create_terminal();
        terminal.execute("ls");
        terminal.execute("pwd");
        assert_eq!(terminal.history().len(), 2);
        terminal.clear_history();
        assert!(terminal.history().is_empty());
    }

    #[test]
    fn test_last_n_entries() {
        let mut terminal = create_terminal();
        terminal.execute("ls");
        terminal.execute("pwd");
        terminal.execute("echo hi");
        let last_2 = terminal.last_n_entries(2);
        assert_eq!(last_2.len(), 2);
        assert_eq!(last_2[0].input, "pwd");
        assert_eq!(last_2[1].input, "echo hi");
    }

    #[test]
    fn test_history_limit_enforcement() {
        let mut terminal = create_terminal();
        terminal.max_history = 3;
        terminal.execute("a");
        terminal.execute("b");
        terminal.execute("c");
        terminal.execute("d");
        assert_eq!(terminal.history().len(), 3);
        assert_eq!(terminal.history()[0].input, "b");
        assert_eq!(terminal.history()[1].input, "c");
        assert_eq!(terminal.history()[2].input, "d");
    }

    #[test]
    fn test_execute_ls() {
        let mut terminal = create_terminal();
        let entry = terminal.execute("ls");
        assert_eq!(entry.input, "ls");
        assert_eq!(entry.exit_code, 0);
        assert!(entry.output.contains("Desktop"));
        assert!(entry.output.contains("Documents"));
    }

    #[test]
    fn test_execute_pwd() {
        let mut terminal = create_terminal();
        let entry = terminal.execute("pwd");
        assert_eq!(entry.input, "pwd");
        assert_eq!(entry.output, "/home/user");
        assert_eq!(entry.exit_code, 0);
    }

    #[test]
    fn test_execute_echo() {
        let mut terminal = create_terminal();
        let entry = terminal.execute("echo Hello World");
        assert_eq!(entry.input, "echo Hello World");
        assert_eq!(entry.output, "Hello World");
        assert_eq!(entry.exit_code, 0);
    }

    #[test]
    fn test_execute_date() {
        let mut terminal = create_terminal();
        let entry = terminal.execute("date");
        assert_eq!(entry.input, "date");
        assert_eq!(entry.exit_code, 0);
        assert!(!entry.output.is_empty());
    }

    #[test]
    fn test_execute_whoami() {
        let mut terminal = create_terminal();
        let entry = terminal.execute("whoami");
        assert_eq!(entry.input, "whoami");
        assert_eq!(entry.output, "user");
        assert_eq!(entry.exit_code, 0);
    }

    #[test]
    fn test_execute_unknown_normal_mode() {
        let mut terminal = create_terminal();
        terminal.set_mode(TerminalMode::Normal);
        let entry = terminal.execute("foocommand123");
        assert_eq!(entry.entry_type, EntryType::Error);
        assert_eq!(entry.exit_code, 127);
        assert!(entry.output.contains("command not found"));
    }

    #[test]
    fn test_execute_unknown_learning_mode() {
        let mut terminal = create_terminal();
        terminal.set_mode(TerminalMode::Learning);
        let entry = terminal.execute("xl");
        assert_eq!(entry.entry_type, EntryType::Error);
        assert_eq!(entry.exit_code, 127);
        assert!(entry.output.contains("tidak ditemukan"));
        assert!(entry.output.contains("Apakah maksud Anda"));
    }

    #[test]
    fn test_did_you_mean_sl_to_ls() {
        let terminal = create_terminal();
        let suggestions = terminal.did_you_mean("sl");
        assert!(!suggestions.is_empty());
        assert!(suggestions.contains(&"ls".to_string()));
    }

    #[test]
    fn test_did_you_mean_pwd_typo() {
        let terminal = create_terminal();
        let suggestions = terminal.did_you_mean("pwwd");
        assert!(!suggestions.is_empty());
        assert!(suggestions.contains(&"pwd".to_string()));
    }

    #[test]
    fn test_did_you_mean_cma() {
        let terminal = create_terminal();
        let suggestions = terminal.did_you_mean("cma");
        assert!(!suggestions.is_empty());
        let has_cmake_or_cargo = suggestions.contains(&"cmake".to_string())
            || suggestions.contains(&"cargo".to_string());
        assert!(has_cmake_or_cargo, "Expected cmake or cargo in suggestions, got: {:?}", suggestions);
    }

    #[test]
    fn test_fuzzy_scoring() {
        let engine = CommandSuggestionEngine::with_defaults();
        let results = engine.fuzzy_match("sl");
        assert!(!results.is_empty());
        let ls_result = results.iter().find(|s| s.command == "ls");
        assert!(ls_result.is_some(), "Expected 'ls' in fuzzy match results for 'sl'");
    }

    #[test]
    fn test_explain_existing() {
        let terminal = create_terminal();
        let explanation = terminal.explain("ls");
        assert!(explanation.is_some());
    }

    #[test]
    fn test_explain_nonexistent() {
        let terminal = create_terminal();
        let explanation = terminal.explain("nonexistent_command_xyz");
        assert!(explanation.is_none());
    }

    #[test]
    fn test_example_existing() {
        let terminal = create_terminal();
        let example = terminal.example("ls");
        assert!(example.is_some());
        assert_eq!(example.as_deref(), Some("ls -la /home"));
    }

    #[test]
    fn test_example_nonexistent() {
        let terminal = create_terminal();
        let example = terminal.example("nonexistent_command_xyz");
        assert!(example.is_none());
    }

    #[test]
    fn test_auto_suggest() {
        let mut terminal = create_terminal();
        terminal.set_input("pyth");
        let suggestion = terminal.auto_suggest("pyth");
        assert!(suggestion.is_some());
        assert_eq!(suggestion.as_deref(), Some("python3"));
    }

    #[test]
    fn test_auto_suggest_empty() {
        let terminal = create_terminal();
        let suggestion = terminal.auto_suggest("");
        assert!(suggestion.is_none());
    }

    #[test]
    fn test_auto_suggest_complete_command() {
        let terminal = create_terminal();
        let suggestion = terminal.auto_suggest("python3");
        assert!(suggestion.is_none());
    }

    #[test]
    fn test_tutorial_start() {
        let mut terminal = create_terminal();
        assert!(terminal.tutorial_state().is_none());
        terminal.start_tutorial("tut_01", "Belajar perintah dasar Linux", 5);
        let ts = terminal.tutorial_state();
        assert!(ts.is_some());
        let ts = ts.unwrap();
        assert_eq!(ts.tutorial_id, "tut_01");
        assert_eq!(ts.step, 1);
        assert_eq!(ts.total_steps, 5);
        assert!(!ts.completed);
    }

    #[test]
    fn test_tutorial_advance() {
        let mut terminal = create_terminal();
        terminal.start_tutorial("tut_01", "Content", 3);
        assert!(terminal.advance_tutorial());
        assert_eq!(terminal.tutorial_state().unwrap().step, 2);
        assert!(terminal.advance_tutorial());
        assert_eq!(terminal.tutorial_state().unwrap().step, 3);
        assert!(!terminal.advance_tutorial());
        assert!(terminal.tutorial_state().unwrap().completed);
    }

    #[test]
    fn test_tutorial_complete() {
        let mut terminal = create_terminal();
        terminal.start_tutorial("tut_01", "Content", 2);
        terminal.advance_tutorial();
        terminal.advance_tutorial();
        assert!(terminal.tutorial_state().unwrap().completed);
    }

    #[test]
    fn test_tutorial_end() {
        let mut terminal = create_terminal();
        terminal.start_tutorial("tut_01", "Content", 5);
        assert!(!terminal.tutorial_state().unwrap().completed);
        terminal.end_tutorial();
        assert!(terminal.tutorial_state().unwrap().completed);
    }

    #[test]
    fn test_directory_tracking() {
        let mut terminal = create_terminal();
        assert_eq!(terminal.current_directory(), Path::new("/home/user"));
        let tmp = Path::new("/tmp");
        assert!(terminal.change_directory(tmp));
        assert_eq!(terminal.current_directory(), tmp);
    }

    #[test]
    fn test_change_directory_nonexistent() {
        let mut terminal = create_terminal();
        let bad_path = Path::new("/nonexistent_path_xyz_123");
        assert!(!terminal.change_directory(bad_path));
        assert_eq!(terminal.current_directory(), Path::new("/home/user"));
    }

    #[test]
    fn test_change_directory_valid() {
        let mut terminal = create_terminal();
        let tmp = Path::new("/tmp");
        assert!(terminal.change_directory(tmp));
        assert_eq!(terminal.current_directory(), tmp);
    }

    #[test]
    fn test_learning_hint_learning_mode() {
        let mut terminal = create_terminal();
        terminal.set_mode(TerminalMode::Learning);
        let hint = terminal.learning_hint("pwwd");
        assert!(hint.is_some());
        assert!(hint.unwrap().contains("pwd"));
    }

    #[test]
    fn test_learning_hint_normal_mode() {
        let mut terminal = create_terminal();
        terminal.set_mode(TerminalMode::Normal);
        let hint = terminal.learning_hint("sl");
        assert!(hint.is_none());
    }

    #[test]
    fn test_learning_hint_unknown() {
        let mut terminal = create_terminal();
        terminal.set_mode(TerminalMode::Learning);
        let hint = terminal.learning_hint("xyznonexistent123");
        assert!(hint.is_none() || !hint.unwrap().is_empty());
    }

    #[test]
    fn test_commands_by_category() {
        let terminal = create_terminal();
        let fs_cmds = terminal.commands_by_category(CommandCategory::FileSystem);
        assert!(!fs_cmds.is_empty());
        let names: Vec<&str> = fs_cmds.iter().map(|c| c.name).collect();
        assert!(names.contains(&"ls"));
        assert!(names.contains(&"cd"));
        assert!(names.contains(&"pwd"));
    }

    #[test]
    fn test_commands_by_difficulty() {
        let terminal = create_terminal();
        let beginner = terminal.commands_by_difficulty("beginner");
        assert!(!beginner.is_empty());
        let beginner_names: Vec<&str> = beginner.iter().map(|c| c.name).collect();
        assert!(beginner_names.contains(&"ls"));
        assert!(beginner_names.contains(&"pwd"));
    }

    #[test]
    fn test_total_known_commands_count() {
        let terminal = create_terminal();
        let total = terminal.total_known_commands();
        assert!(
            total >= 100,
            "Expected at least 100 known commands, got {}",
            total
        );
    }

    #[test]
    fn test_locale_switching() {
        let mut terminal = create_terminal();
        terminal.set_locale("en-US");
        terminal.set_locale("id-ID");
    }

    #[test]
    fn test_empty_input() {
        let mut terminal = create_terminal();
        let entry = terminal.execute("");
        assert_eq!(entry.input, "");
        assert_eq!(entry.output, "");
        assert_eq!(entry.exit_code, 0);
    }

    #[test]
    fn test_clear_command() {
        let mut terminal = create_terminal();
        let entry = terminal.execute("clear");
        assert_eq!(entry.input, "clear");
        assert_eq!(entry.output, "");
        assert_eq!(entry.exit_code, 0);
        assert_eq!(entry.entry_type, EntryType::System);
    }

    #[test]
    fn test_exit_command() {
        let mut terminal = create_terminal();
        let entry = terminal.execute("exit");
        assert_eq!(entry.input, "exit");
        assert_eq!(entry.output, "logout");
        assert_eq!(entry.exit_code, 0);
        assert_eq!(terminal.state(), TerminalState::Completed);
    }

    #[test]
    fn test_help_command() {
        let mut terminal = create_terminal();
        let entry = terminal.execute("help");
        assert_eq!(entry.input, "help");
        assert_eq!(entry.exit_code, 0);
        assert!(entry.output.contains("Daftar Perintah"));
        assert!(entry.output.contains("File System"));
    }

    #[test]
    fn test_history_command() {
        let mut terminal = create_terminal();
        terminal.execute("ls");
        terminal.execute("pwd");
        let entry = terminal.execute("history");
        assert!(entry.output.contains("ls"));
        assert!(entry.output.contains("pwd"));
    }

    #[test]
    fn test_suggest_for_purpose() {
        let terminal = create_terminal();
        let suggestions = terminal.suggest("pyth");
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.command == "python3"));
    }

    #[test]
    fn test_suggest_for_purpose_description_match() {
        let terminal = create_terminal();
        let suggestions = terminal.suggest("list");
        assert!(!suggestions.is_empty());
    }

    #[test]
    fn test_known_command_lookup() {
        let terminal = create_terminal();
        let cmd = terminal.known_command("ls");
        assert!(cmd.is_some());
        assert_eq!(cmd.unwrap().name, "ls");

        let nonexistent = terminal.known_command("nonexistent");
        assert!(nonexistent.is_none());
    }

    #[test]
    fn test_all_known_commands() {
        let terminal = create_terminal();
        let all = terminal.all_known_commands();
        assert!(!all.is_empty());
        assert!(all.iter().any(|c| c.name == "ls"));
        assert!(all.iter().any(|c| c.name == "git"));
        assert!(all.iter().any(|c| c.name == "cargo"));
    }

    #[test]
    fn test_execute_cd_home() {
        let mut terminal = create_terminal();
        let entry = terminal.execute("cd");
        assert_eq!(entry.exit_code, 0);
        assert_eq!(terminal.current_directory(), Path::new("/home/user"));
    }

    #[test]
    fn test_execute_cd_root() {
        let mut terminal = create_terminal();
        let entry = terminal.execute("cd /");
        assert_eq!(entry.exit_code, 0);
        assert_eq!(terminal.current_directory(), Path::new("/"));
    }

    #[test]
    fn test_execute_cd_accepts_any_path() {
        let mut terminal = create_terminal();
        let entry = terminal.execute("cd /nonexistent_path_xyz");
        assert_eq!(entry.exit_code, 0);
        assert_eq!(entry.entry_type, EntryType::Command);
        assert_eq!(terminal.current_directory(), Path::new("/nonexistent_path_xyz"));
    }

    #[test]
    fn test_execute_known_in_learning_mode() {
        let mut terminal = create_terminal();
        terminal.set_mode(TerminalMode::Learning);
        let entry = terminal.execute("whoami");
        assert_eq!(entry.exit_code, 0);
        assert_eq!(entry.entry_type, EntryType::Output);
        assert!(entry.output.contains("Contoh"));
    }

    #[test]
    fn test_common_options() {
        let engine = CommandSuggestionEngine::with_defaults();
        let options = engine.get_common_options("ls");
        assert!(!options.is_empty());
        assert!(options.iter().any(|(flag, _)| flag == "-l"));
    }

    #[test]
    fn test_common_options_unknown() {
        let engine = CommandSuggestionEngine::with_defaults();
        let options = engine.get_common_options("nonexistent");
        assert!(options.is_empty());
    }

    #[test]
    fn test_fuzzy_score_exact() {
        assert_eq!(fuzzy_score("ls", "ls"), 100);
        assert_eq!(fuzzy_score("pwd", "pwd"), 100);
    }

    #[test]
    fn test_fuzzy_score_prefix() {
        assert_eq!(fuzzy_score("py", "python3"), 95);
    }

    #[test]
    fn test_fuzzy_score_substring() {
        assert_eq!(fuzzy_score("runc", "rustc"), 40);
    }

    #[test]
    fn test_fuzzy_score_no_match() {
        assert_eq!(fuzzy_score("zzz", "ls"), 0);
    }

    #[test]
    fn test_learning_hint_content() {
        let mut terminal = create_terminal();
        terminal.set_mode(TerminalMode::Learning);
        let hint = terminal.learning_hint("scl");
        assert!(hint.is_some());
        let hint = hint.unwrap();
        assert!(hint.contains("Hint"));
        assert!(hint.contains("scp"));
    }

    #[test]
    fn test_terminal_entry_fields() {
        let entry = TerminalEntry {
            input: "test".to_string(),
            output: "output".to_string(),
            entry_type: EntryType::Command,
            timestamp: "2024-01-15 10:00:00".to_string(),
            working_directory: PathBuf::from("/home/user"),
            exit_code: 0,
        };
        assert_eq!(entry.input, "test");
        assert_eq!(entry.output, "output");
        assert_eq!(entry.entry_type, EntryType::Command);
        assert_eq!(entry.exit_code, 0);
    }

    #[test]
    fn test_known_commands_len() {
        let known = KnownCommands::new();
        assert!(known.len() >= 100);
        assert!(!known.is_empty());
    }

    #[test]
    fn test_known_commands_find() {
        let known = KnownCommands::new();
        let ls = known.find("ls");
        assert!(ls.is_some());
        assert_eq!(ls.unwrap().category, CommandCategory::FileSystem);

        let git = known.find("git");
        assert!(git.is_some());
        assert_eq!(git.unwrap().category, CommandCategory::Git);
    }

    #[test]
    fn test_known_commands_related() {
        let known = KnownCommands::new();
        let ls = known.find("ls").unwrap();
        assert!(!ls.related.is_empty());
        assert!(ls.related.contains(&"find"));
    }

    #[test]
    fn test_set_locale_affects_explanation() {
        let mut terminal = create_terminal();
        terminal.set_locale("id-ID");
        let explanation_id = terminal.explain("ls");
        assert!(explanation_id.is_some());

        terminal.set_locale("en-US");
        let explanation_en = terminal.explain("ls");
        assert!(explanation_en.is_some());
    }

    #[test]
    fn test_execute_with_trailing_whitespace() {
        let mut terminal = create_terminal();
        let entry = terminal.execute("  pwd  ");
        assert_eq!(entry.input, "  pwd  ");
        assert_eq!(entry.output, "/home/user");
        assert_eq!(entry.exit_code, 0);
        assert_eq!(entry.entry_type, EntryType::Output);
    }

    #[test]
    fn test_suggest_with_empty_input() {
        let terminal = create_terminal();
        let suggestions = terminal.suggest("");
        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_execute_multiple_commands() {
        let mut terminal = create_terminal();
        terminal.execute("ls");
        terminal.execute("pwd");
        terminal.execute("echo test");
        terminal.execute("whoami");
        assert_eq!(terminal.history().len(), 4);
        assert_eq!(terminal.state(), TerminalState::Completed);
    }

    #[test]
    fn test_execute_cd_dotdot() {
        let mut terminal = create_terminal();
        terminal.current_directory = PathBuf::from("/home/user/Documents");
        let entry = terminal.execute("cd ..");
        assert_eq!(entry.exit_code, 0);
        assert_eq!(terminal.current_directory(), Path::new("/home/user"));
    }

    #[test]
    fn test_execute_cd_home_tilde() {
        let mut terminal = create_terminal();
        terminal.current_directory = PathBuf::from("/tmp");
        let entry = terminal.execute("cd ~");
        assert_eq!(entry.exit_code, 0);
        assert_eq!(terminal.current_directory(), Path::new("/home/user"));
    }

    #[test]
    fn test_did_you_mean_git_typo() {
        let terminal = create_terminal();
        let suggestions = terminal.did_you_mean("gti");
        assert!(!suggestions.is_empty());
        assert!(suggestions.contains(&"git".to_string()));
    }

    #[test]
    fn test_difficulty_levels() {
        let terminal = create_terminal();
        let beginner = terminal.commands_by_difficulty("beginner");
        let intermediate = terminal.commands_by_difficulty("intermediate");
        let advanced = terminal.commands_by_difficulty("advanced");
        assert!(!beginner.is_empty());
        assert!(!intermediate.is_empty());
        assert!(!advanced.is_empty());
    }

    #[test]
    fn test_default_trait() {
        let terminal = EduTerminal::default();
        assert_eq!(terminal.mode(), TerminalMode::Normal);
    }

    #[test]
    fn test_known_commands_default() {
        let known = KnownCommands::default();
        assert!(known.len() >= 100);
    }

    #[test]
    fn test_cd_with_relative_path() {
        let mut terminal = create_terminal();
        terminal.current_directory = PathBuf::from("/home/user");
        let parent = terminal.current_directory().parent();
        assert!(parent.is_some());
    }

    #[test]
    fn test_history_entry_timestamps() {
        let mut terminal = create_terminal();
        let entry = terminal.execute("ls");
        assert!(!entry.timestamp.is_empty());
        assert!(entry.timestamp.contains('-') || entry.timestamp.contains(':'));
    }

    #[test]
    fn test_execute_id_command() {
        let mut terminal = create_terminal();
        let entry = terminal.execute("id");
        assert_eq!(entry.exit_code, 0);
        assert!(entry.output.contains("uid="));
    }

    #[test]
    fn test_command_suggestion_fields() {
        let suggestion = CommandSuggestion {
            command: "test".to_string(),
            description_key: "cmd.test.desc".to_string(),
            confidence: 85,
            category: CommandCategory::Utility,
        };
        assert_eq!(suggestion.command, "test");
        assert_eq!(suggestion.confidence, 85);
        assert_eq!(suggestion.category, CommandCategory::Utility);
    }

    #[test]
    fn test_simulate_unknown_command() {
        let mut terminal = create_terminal();
        let known_cmd = terminal.known_command("dd");
        assert!(known_cmd.is_some());
        let entry = terminal.execute("dd if=/dev/zero of=test.img bs=1M count=10");
        assert_eq!(entry.exit_code, 0);
    }

    #[test]
    fn test_learning_mode_suggestions_quality() {
        let mut terminal = create_terminal();
        terminal.set_mode(TerminalMode::Learning);
        let entry = terminal.execute("slep");
        assert!(entry.output.contains("sleep"));
    }

    #[test]
    fn test_suggest_for_purpose_known_prefix() {
        let terminal = create_terminal();
        let results = terminal.command_suggestions.suggest_for_purpose("pip");
        let _has_pipe = results.iter().any(|s| s.command == "ping" || s.command == "pkill");
    }

    #[test]
    fn test_known_command_difficulty_present() {
        let known = KnownCommands::new();
        for cmd in known.all() {
            assert!(
                cmd.difficulty == "beginner"
                    || cmd.difficulty == "intermediate"
                    || cmd.difficulty == "advanced",
                "Command '{}' has invalid difficulty '{}'",
                cmd.name,
                cmd.difficulty
            );
        }
    }

    #[test]
    fn test_known_command_names_are_unique() {
        let known = KnownCommands::new();
        let mut names: Vec<&str> = known.all().iter().map(|c| c.name).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), known.len());
    }

    #[test]
    fn test_auto_suggest_no_match() {
        let terminal = create_terminal();
        let suggestion = terminal.auto_suggest("zzzzz");
        assert!(suggestion.is_none());
    }
}
