# Configuration tips

In contrast with legacy log parsers, Pyruse works with structured [systemd-journal entries](https://www.freedesktop.org/software/systemd/man/systemd.journal-fields.html). This allows for better performance, since targeted comparisons become possible.

The general intent, when writing the configuration file, should be to handle the log entries that appear the most often first, in as few steps as possible. For example, I ran some stats on my server before writing my own configuration file; I got:

| systemd units                   | number of journal entries |
| ------------------------------- | -------------------------:|
| `prosody.service`               |                    518019 |
| `gitea.service`                 |                    329389 |
| `uwsgi@nextcloud.service`       |                    217342 |
| `session-*.scope`               |                     89813 |
| `nginx.service`                 |                     80762 |
| `dovecot.service`               |                     61898 |
| `exim.service`                  |                     60743 |
| `init.scope`                    |                     43021 |
| `nextcloud-maintenance.service` |                     20775 |
| `haproxy.service`               |                     18445 |
| `user@*.service`                |                      7306 |
| `minidlna.service`              |                      6032 |
| `loolwsd.service`               |                      5797 |
| `sshd.service`                  |                      4959 |
| `spamassassin-update.service`   |                      2383 |
| `systemd-nspawn@*.service`      |                      1497 |
| `nslcd.service`                 |                       867 |
| `nfs-mountd.service`            |                       723 |
| `systemd-logind.service`        |                       696 |
| `nfs-server.service`            |                       293 |
| `systemd-networkd.service`      |                       121 |
| misc. units with < 100 entries  |                           |

For reference, here is the command that gives these statistics:

```bash
$ bash ./extra/examples/get-systemd-stats.sh >~/systemd-units.stats.tsv
```

One should also remember, that numeric comparison are faster that string comparison, which in turn are faster than regular expression pattern-matching. Further more, some log entries are not worth checking for, because they are too rare: it costs more to grab them with filters (that most log entries will have to pass through), than letting them get caught by the catch-all last execution chain, which typically uses the `action_dailyReport` module.

An efficient way to organize the configuration file is by handling units from the most verbose to the least verbose, and for each unit, filter-out useless entries based on the `PRIORITY` (which is an integer number) whenever it is possible. In short, filtering on the actual message, while not entirely avoidable, is the last-resort operation.

An [example based on the above statistics](../extra/examples/full_pyruse.json) is available in the `extra/examples/` source directory.
