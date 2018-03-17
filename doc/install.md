# Installation

The software requirements are:

* a modern systemd-based Linux operating system (eg. [Archlinux](https://archlinux.org/)- or [Fedora](https://getfedora.org/)-based distributions);
* python, at least version 3.4 (or [more, depending on the modules](intro_tech.md) being used);
* [python-systemd](https://www.freedesktop.org/software/systemd/python-systemd/journal.html);
* [nftables](http://wiki.nftables.org/) or [ipset](http://ipset.netfilter.org/) _if_ IP address bans are to be managed;
* a sendmail-like program _if_ emails are wanted.

Besides, getting the software requires [Git](http://git-scm.com/), and packaging it requires [python-setuptools](http://pypi.python.org/pypi/setuptools).

## Get and run Pyruse

Getting the software is just a matter of cloning the repository with Git.

It can be run without being installed:

1. Create a [configuration file](conffile.md) in the root directory of the repository (where `doc`, `extra`, `pyruse`, `tests`… reside).

2. Run Pyruse like this at the root directory of the repository:

```bash
$ sudo python3 -c 'from pyruse import main; main.main()'
```

## Run the tests

To run the tests, enter the `tests` subdirectory, and run `python3 main.py` there.

## Install and run Pyruse

To install Pyruse on the system, run these commands as root, in the root directory of the repository:

```bash
# curl -o PKGBUILD 'https://aur.archlinux.org/cgit/aur.git/plain/PKGBUILD?h=pyruse'
# . PKGBUILD
# export srcdir="$PWD/.."
# export pkgdir=
# package
# rm -rf build PKGBUILD
# systemctl daemon-reload
```

The `package` line is the one that actually alters the system. Until Pyruse is packaged for your operating system, you may want to change this line to `checkinstall package`. [Checkinstall](https://en.wikipedia.org/wiki/CheckInstall) should be able to turn your Pyruse installation into a native Linux package.

Then, to run Pyruse, start (and enable) `pyruse.service`.

If you use nftables bans, you should also start (and enable) `pyruse-boot@action_nftBan.service`.
Likewise, if you use ipset bans, you should start (and enable) `pyruse-boot@action_ipsetBan.service`.
