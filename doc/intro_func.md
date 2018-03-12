# Introduction to what Pyruse is

Everyone knows [Fail2ban](http://www.fail2ban.org/).
This program is excellent at what it does, which is matching patterns in a number of log files, and keeping track of counters in order to launch actions (ban an IP, send an email…) when thresholds are crossed.
[Fail2ban is extremely customizable](http://yalis.fr/cms/index.php/post/2014/11/02/Migrate-from-DenyHosts-to-Fail2ban)… to a point; and to my knowledge it cannot benefit from all the features of modern-day logging with systemd.

Then, there is the less-known and aging [Epylog](http://freshmeat.sourceforge.net/projects/epylog/); several programs exist with the same features.
Just like Fail2ban, this program continuously scans the logs to do pattern matching.
In contrast to Fail2ban, though, this tool’s purpose is to deliver an email every day, with a summary of the past day’s events.
Epylog is unfortunately very limited, with few hooks, if the default behaviour is not exactly what you want.

The point is, both kinds of tools have a lot of overlap in their inner workings, even though their outcome differ.
This is a waste of resources.
Besides, these tools suffer from the legacy handling of logs, in files, where text messages are roughtly the only information available.

Guidelines for Pyruse:

* **modern**: systemd, python3, no deprecated API…
* **light-weight**;
* **efficient**.

At the origin of the project are these wanted features:

* Peruse all log entries from systemd’s journal, and only those (ie: no log files).
* Passively wait on new entries; no active polling.
* Filter-out uninteresting log lines according to the settings.
* Act on matches in the journal, with some pre-defined actions available.
* Create a daily report with 3 parts:
    - events of importance (according to the settings) that should be checked,
    - events of interest (according to the settings),
    - and other non-filtered-out log entries.
* Send an immediate email when something very important happens (according to the settings).

The result looks a bit like the way a Netfilter firewall is built, with [execution chains made of filters and actions](configure.md).
Both filters and actions work on a systemd-journal entry, where all fields are available, and more fields can be computed and stored, to be worked-on later, and so on.

The most interesting [filtering or informational entries](https://www.freedesktop.org/software/systemd/man/systemd.journal-fields.html) are:

* `PRIORITY`: see [Syslog at Wikipedia](https://en.wikipedia.org/wiki/Syslog#Severity_level) for the definitions
* `SYSLOG_FACILITY`: see [Syslog at Wikipedia](https://en.wikipedia.org/wiki/Syslog#Facility) for the definitions
* `SYSLOG_IDENTIFIER`: short name for the program that produced the log entry (better accuracy than `_SYSTEMD_UNIT`)
* `_HOSTNAME`: short hostname of the machine where the log entry occurred
* `_UID`: user ID of the systemd service that produced the log entry
* `_GID`: group ID of the systemd service that produced the log entry
* `_SYSTEMD_UNIT`: name of the systemd unit that produced the log entry
* `MESSAGE`: the actual message of the log entry
* `__REALTIME_TIMESTAMP`: Python `datetime` of the log entry (gets formatted as: `YYYY-MM-DD hh:mm:ss:µµµµµµ`)

Pyruse already comes with some common modules. [More can be easily written](customize.md).
