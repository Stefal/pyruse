# Python peruser of systemd-journal

This program is intended to be used as a lightweight replacement for both epylog and fail2ban.
Its purpose is to peruse the system log entries, warn of important situations, report daily on the latest events, and act on specific patterns (IP address bans…).

* [Functional overview](doc/intro_func.md)
* [Technical overview](doc/intro_tech.md)

The software requirements are:

* a modern systemd-based Linux operating system (eg. [Archlinux](https://archlinux.org/)- or [Fedora](https://getfedora.org/)-based distributions);
* python, at least version 3.1 (or [more, depending on the modules](doc/intro_tech.md) being used);
* [python-systemd](https://www.freedesktop.org/software/systemd/python-systemd/journal.html);
* [nftables](http://wiki.nftables.org/) _if_ IP address bans are to be managed;
* a sendmail-like program _if_ emails are wanted.

The `/etc/pyruse` directory is where system-specific files are looked-for:

* the `pyruse.json` file that contains the [configuration](doc/conffile.md),
* the `pyruse/actions` and `pyruse/filters` subfolders, which may contain [additional actions and filters](doc/customize.md).

Instead of using `/etc/pyruse`, an alternate directory may be specified with the `PYRUSE_EXTRA` environment variable.

For more in-depth documentation, please refer to these pages:

* [General structure of the `pyruse.json` file](doc/conffile.md)
* [How do I write the `pyruse.json` file?](doc/configure.md)
* [Writing custom filters and actions](doc/customize.md)
* More information about:
    - [the built-in filters](doc/builtinfilters.md)
    - [the counter-based actions](doc/counters.md)
    - [the `action_noop` module](doc/noop.md)
    - [the `action_email` module](doc/action_email.md)
    - [the `action_dailyReport` module](doc/action_dailyReport.md)
    - [the `action_nftBan` module](doc/action_nftBan.md)
