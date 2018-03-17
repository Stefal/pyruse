# Configuration tips

In contrast with legacy log parsers, Pyruse works with structured [systemd-journal entries](https://www.freedesktop.org/software/systemd/man/systemd.journal-fields.html).
This allows for better performance, since targeted comparisons become possible.

The general intent, when writing the configuration file, should be to handle the log entries that appear the most often first, in as few steps as possible.
For example, I ran some stats on my server on the log entries of the past week; I got:

| SYSLOG_IDENTIFIER       | number of journal entries |
| ----------------------- | -------------------------:|
| `uwsgi` (for Nextcloud) |                     55930 |
| `gitea`                 |                     38923 |
| `prosody`               |                     25596 |
| `haproxy`               |                     21877 |
| `postgres`              |                     12990 |
| `nginx`                 |                     12808 |
| `dovecot`               |                      7062 |
| `exim`                  |                      2540 |
| `systemd`               |                      1997 |
| `su`                    |                      1458 |
| `ownCloud` (Nextcloud)  |                      1067 |
| `sshd`                  |                      1051 |
| `mandb`                 |                       953 |
| `spamd`                 |                       855 |
| `pyruse`                |                       615 |
| `kernel`                |                       420 |
| `msmtp`                 |                       295 |
| `sa-compile`            |                       255 |
| `ansible-*`             |                       103 |
| `systemd-logind`        |                       102 |
| `python`                |                        78 |
| `rpc.mountd`            |                        52 |
| `ldapwhoami`            |                        42 |
| `prosody_auth`          |                        42 |
| `minidlnad`             |                        39 |
| `kill`                  |                        28 |
| `sudo`                  |                        26 |
| `loolwsd`               |                        17 |
| `exportfs`              |                        15 |
| `dehydrated`            |                         6 |
| `sa-update`             |                         5 |
| `nslcd`                 |                         4 |
| `rpc.idmapd`            |                         1 |

For reference, here is the command that gives these statistics:

```bash
$ bash ./extra/examples/get-systemd-stats.sh >~/systemd-units.stats.tsv
```

One should also remember, that numeric comparison are faster that string comparison, which in turn are faster than regular expression pattern-matching. Further more, some log entries are not worth checking for, because they are too rare: it costs more to grab them with filters (that most log entries will have to pass through), than letting them get caught by the catch-all last execution chain, which typically uses the `action_dailyReport` module.

An efficient way to organize the configuration file is by handling Syslog-identifiers from the most verbose to the least verbose, and for each one, filter-out useless entries based on the `PRIORITY` (which is an integer number) whenever it is possible.
In short, filtering on the actual message, while not entirely avoidable, is the last-resort operation.

NOTE: I used to group my log entries (and Pyruse execution chains) by `_SYSTEMD_UNIT`, which seemed logical at the time.
However, for some reason, there is some “leaking” of logs from some units to others; for example, I had Nginx logs appearing in the Exim `_SYSTEMD_UNIT`… The reason probably lies somewhere in inter-process communication, or with the launching of external commands.
Anyway, I found that grouping by `SYSLOG_IDENTIFIER` actually gives better results:

* `SYSLOG_IDENTIFIER` names are shorter than `_SYSTEMD_UNIT` names, hence probably quicker to compare `:-p`
* Several `_SYSTEMD_UNIT` names from generic units (like `unit-name@instance-name`) end up into the same `SYSLOG_IDENTIFIER`, which allows to occasionaly replace `filter_pcre` with `filter_equals`.
* A single program often does several tasks, and `SYSLOG_IDENTIFIER` reflects this diversity, which makes writing rules much easier.  
For example, Pyruse sends emails using msmtp; I do not care about `msmtp`’s logs, but I do about `pyruse`’s. Filtering-out logs from the `msmtp` `SYSLOG_IDENTIFIER` is much easier to do than getting rid of email-related logs from the `pyruse.service` systemd unit.

An [example based on the above statistics](../extra/examples/full_pyruse.json) is available in the `extra/examples/` source directory.
