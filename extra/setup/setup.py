from distutils.core import setup

setup(
    name='pyruse',
    version='1.0',
    license='GPL-3',
    description='Route systemd-journal logs to filters and actions (ban, report…)',
    long_description='''
================
Python peruser of systemd-journal
================

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
    ''',
    author='Yves G.',
    author_email='theYinYeti@yalis.fr',
    maintainer='Yves G.',
    maintainer_email='theYinYeti@yalis.fr',
    url='https://yalis.fr/git/yves/pyruse',
    download_url='https://yalis.fr/git/yves/pyruse',
    packages=['pyruse', 'pyruse.actions', 'pyruse.filters'],
)
