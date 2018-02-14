# Python peruser of systemd-journal

This program is intended to be used as a lightweight replacement for both epylog and fail2ban.
Its purpose is to peruse the system log entries, warn of important situations, report daily on the latest events, and act on specific patterns (IP address bans…).

* [Functional overview](doc/intro_func.md)
* [Technical overview](doc/intro_tech.md)

Pyruse is [packaged for Archlinux](https://aur.archlinux.org/packages/pyruse/).
For other distributions, please [read the manual installation instructions](doc/install.md).

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
