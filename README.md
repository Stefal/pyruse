# Python peruser of systemd-journal

This program is intended to be used as a lightweight replacement for both epylog and fail2ban.

The wanted features are these:

* Peruse all log entries from systemd’s journal, and only those (ie: no log files).
* Passively wait on new entries; no active polling.
* Filter-out uninteresting log lines according to the settings.
* Act on matches in the journal, with some pre-defined actions.
* Create a daily report with 2 parts:
    - events of interest (according to the settings),
    - and other non-filtered-out log entries.
* Send an immediate email when something important happens (according to the settings).

Interesting [filtering entries](https://www.freedesktop.org/software/systemd/man/systemd.journal-fields.html) are:
* `_TRANSPORT`: how the log entry got to the journal (`stdout`, `syslog`, `journal`)
* `PRIORITY`: see https://en.wikipedia.org/wiki/Syslog#Severity_level
* `SYSLOG_FACILITY`: see https://en.wikipedia.org/wiki/Syslog#Facility
* `_CAP_EFFECTIVE`: effective capabilities as an hexadecimal mask
* `_BOOT_ID`: boot identifier (may be used to detect reboots)
* `_MACHINE_ID`: internal systemd ID for the machine where the log entry occurred
* `_HOSTNAME`: short hostname of the machine where the log entry occurred
* `_UID`: user ID of the systemd service that produced the log entry
* `_GID`: group ID of the systemd service that produced the log entry
* `SYSLOG_IDENTIFIER`: service name as reported to the “syslog” API
* `_COMM`: name of the command that produced the log entry
* `_EXE`: path to the executable file launched by systemd
* `_SYSTEMD_CGROUP`: cgroup of the service, eg. `/system.slice/systemd-uwsgi.slice/uwsgi@nextcloud.service`
* `_SYSTEMD_UNIT`: name of the systemd unit that produced the log entry
* `_SYSTEMD_SLICE`: name of the systemd slice
* `_CMDLINE`: process name as reported by the main process of the systemd service
* `_PID`: process ID of the systemd unit’s main process
* `MESSAGE`: the actual message of the log entry
* `__REALTIME_TIMESTAMP`: Python `datetime` of the log entry, formatted as: `YYYY-MM-DD HH:MM:SS:µµµµµµ`

The `/etc/pyruse` directory is where system-specific files are looked-for:
* the `pyruse.json` file that contains the configuration,
* the `pyruse/actions` and `pyruse/filters` subfolders, which may contain additional actions and filters.

Instead of using `/etc/pyruse`, an alternate directory may be specified with the `PYRUSE_EXTRA` environment variable.
