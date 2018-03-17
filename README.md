# Python peruser of systemd-journal

## Summary

This program is intended to be used as a lightweight replacement for both epylog and fail2ban.
Its purpose is to peruse the system log entries, warn of important situations, report daily on the latest events, and act on specific patterns (IP address bans…).

* [Functional overview](doc/intro_func.md)
* [Technical overview](doc/intro_tech.md)

The benefits of Pyruse over products of the same kind are:

* **Optimization brought by systemd**  
[systemd-journal entries](https://www.freedesktop.org/software/systemd/man/systemd.journal-fields.html) play an important role in Pyruse: instead of matching log entries against message patterns only, the whole range of systemd’s journal fields is available. This allows for the much faster integer comparisons (`PRIORITY`, `_UID`…), or even faster comparisons with short strings like the `SYSLOG_IDENTIFIER`, `_SYSTEMD_UNIT`, or `_HOSTNAME`, with the opportunity to test more often for equality, and less for regular expressions.

* **Optimization brought by context**  
Programs that peruse the system logs usually apply a set of rules on each log entry, rule after rule, regardless of what can be deduced by the already-applied rules.  
In contrast, each fact learnt by applying a rule in Pyruse can be taken into account so that rules that do not apply are not even considered.  
For example, after matching the `SYSLOG_IDENTIFIER` of a journal entry to the value `sshd`, only SSH-related rules are applied, not Nginx-related rules, nor Prosody-related rules.

* **Modularity**  
Each filter (ie. a matching step) or action (eg. a ban, an email, etc.) is a Python module with a very simple API. As soon as a new need arises, a module can be written for it.  
For example, to my knowledge, there is no equivalent in any tool of the same scale, for the [DNAT-correcting actions](doc/dnat.md) now included with Pyruse.

## Get Pyruse

Pyruse is [packaged for Archlinux](https://aur.archlinux.org/packages/pyruse/).
For other distributions, please [read the manual installation instructions](doc/install.md).

Whenever your upgrade Pyruse, make sure to check the [Changelog](Changelog.md).

## Configuration

The `/etc/pyruse` directory is where system-specific files are looked-for:

* the `pyruse.json` file that contains the [configuration](doc/conffile.md),
* the `pyruse/actions` and `pyruse/filters` subfolders, which may contain [additional actions and filters](doc/customize.md).

Instead of using `/etc/pyruse`, an alternate directory may be specified with the `PYRUSE_EXTRA` environment variable.

## Documentation

For more in-depth documentation, please refer to these pages:

* [General structure of the `pyruse.json` file](doc/conffile.md)
* [How do I write the `pyruse.json` file?](doc/configure.md)
* [Writing custom filters and actions](doc/customize.md)
* More information about:
    - [the built-in filters](doc/builtinfilters.md)
    - [the counter-based actions](doc/counters.md)
    - [the DNAT-related actions](doc/dnat.md)
    - [the actions that log and ban](doc/logandban.md)
    - [the `action_noop` module](doc/noop.md)
    - [the `action_email` module](doc/action_email.md)
    - [the `action_dailyReport` module](doc/action_dailyReport.md)
