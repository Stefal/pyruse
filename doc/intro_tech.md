# Technical overview of Pyruse

Pyruse is built on [python-systemd](https://www.freedesktop.org/software/systemd/python-systemd/journal.html).
This API already brings a number of benefits:

* Each log entry is obtained as a Python dictionary where all systemd-journal fields are available.
* Each log field has the good data type, eg. the systemd timestamp becomes a Python `datetime`.
* An API call is provided to passively wait for new log entries.

The core of Pyruse has no other dependency.
However, each module may have its own dependencies, for example the availability of a program that can accept emails on its standard input (usually `/usr/bin/sendmail`), for modules that send emails.

It should be noted, that modern Python API are used. Thus:

* Python version ≥ 3.1 is required for managing modules (`importlib`);
* Python version ≥ 3.1 is required for loading the configuration (json’s `object_pairs_hook`);
* Python version ≥ 3.2 is required for the daily report and emails (string’s `format_map`);
* Python version ≥ 3.4 is required for the daily report and logging, thus also the log action (`enum`);
* Python version ≥ 3.5 is required for IP address bans and emails, thus also the daily report (subprocess’ `run`);
* Python version ≥ 3.6 is required for emails, thus also the daily report (`headerregistry`, `EmailMessage`).

In order to be fast, this program avoids dynamic decisions while running.
To this end, a static workflow of filters and actions is built upon start, based on the configuration file.
After that, log entries are pushed into the workflow, much like a train is on a railway, the switchpoints being the Pyruse filters.

Pyruse is split into several Python files:

* `main.py`: As expected, this is the conductor, responsible for interfacing with the configuration, the workflow, and systemd. It also has a “boot” mode, where actions are allowed to set the stage before Pyruse starts.
* `config.py`: The `Config` class is responsible for reading the configuration file, and making it available as a Python dictionary.
* `log.py`: This one allows Pyruse’s own logs to be forwarded to systemd’s journal, which opens the road to recidive detection for example.
* `workflow.py`: This is where the configuration file get translated into actual execution chains, linked together into a single static workflow.
* `module.py`: Whenever the workflow needs to add a filter or an action to an execution chain, this module finds it in the filesystem.
* `base.py`: All actions and filters inherit from the `Action` and `Filter` classes defined in there; they act as an abstraction layer between the workflow and the modules.
* `ban.py`: This utility class is parent to modules that ban IP addresses using [Netfilter](https://netfilter.org/) (Python supports multiple-inheritance).
* `counter.py`: This utility class is parent to modules that manage a counter.
* `dnat.py`: This file contains utility parent classes for actions that try and restore the actual client IP addresses.
* `email.py`: This utility class is parent to modules that send emails.

All else is actions and filters…
Some are delivered with Pyruse itself; [more can be added](customize.md).

_Tip_: The documentation is part of the source repository.
Contributions to the code or the documentation are welcome `;-)`
